use std::cmp::max;

use colored::{Color, Colorize};
use similar::{Change, ChangeTag, TextDiff};

/// DiffAnalysis provides functionality for diffs within dotpatina
pub trait DiffAnalysis {
    /// Determine if there are any changes in the diff
    fn any_changes(&self) -> bool;

    /// Get a String representation of the diff for display
    fn to_string(&self) -> String;
}

/// Details for a line output for a diff
struct DiffLine {
    old_line_num: Option<usize>,
    new_line_num: Option<usize>,
    diff_char: char,
    change_tag: ChangeTag,
    change_string: String,
    color: Option<Color>,
    count_from_change: usize,
}

fn diff_line_to_string(diff_line: &DiffLine, line_number_width: usize) -> std::string::String {
    let old_line_num = match diff_line.old_line_num {
        Some(l) => l.to_string(),
        None => "".to_string(),
    };
    let new_line_num = match diff_line.new_line_num {
        Some(l) => l.to_string(),
        None => "".to_string(),
    };
    let line = format!(
        "{} {: >num_width$} {: >num_width$} | {}",
        diff_line.diff_char,
        old_line_num,
        new_line_num,
        diff_line.change_string,
        num_width = line_number_width
    );
    match diff_line.color {
        Some(color) => line.color(color).to_string(),
        None => line,
    }
}

impl<'lines, O: ?Sized> DiffAnalysis for TextDiff<'lines, 'lines, '_, O>
where
    O: similar::DiffableStr,
{
    fn any_changes(&self) -> bool {
        self.iter_all_changes()
            .any(|change| change.tag() != ChangeTag::Equal)
    }

    fn to_string(&self) -> String {
        static DIFF_GAP_BUFFER_SIZE: usize = 4;

        let mut old_line_count = 1;
        let mut new_line_count = 1;
        let mut count_from_change = 0;

        let change_to_diff_line = |change: Change<&O>| match change.tag() {
            ChangeTag::Insert => {
                count_from_change = 0;
                let line = DiffLine {
                    old_line_num: None,
                    new_line_num: Some(new_line_count),
                    diff_char: '+',
                    change_tag: ChangeTag::Insert,
                    change_string: change.to_string(),
                    color: Some(Color::Green),
                    count_from_change,
                };
                new_line_count += 1;
                line
            }
            ChangeTag::Equal => {
                count_from_change += 1;
                let line = DiffLine {
                    old_line_num: Some(old_line_count),
                    new_line_num: Some(new_line_count),
                    diff_char: ' ',
                    change_tag: ChangeTag::Equal,
                    change_string: change.to_string(),
                    color: None,
                    count_from_change,
                };
                old_line_count += 1;
                new_line_count += 1;
                line
            }
            ChangeTag::Delete => {
                count_from_change = 0;
                let line = DiffLine {
                    old_line_num: Some(old_line_count),
                    new_line_num: None,
                    diff_char: '-',
                    change_tag: ChangeTag::Delete,
                    change_string: change.to_string(),
                    color: Some(Color::Red),
                    count_from_change,
                };
                old_line_count += 1;
                line
            }
        };

        let diff_lines = self
            .iter_all_changes()
            .map(change_to_diff_line)
            .collect::<Vec<DiffLine>>();

        let line_number_width = max(
            old_line_count.to_string().len(),
            new_line_count.to_string().len(),
        );

        let mut count_until_changed = 0;

        let mut check_diff_gap = |diff_line: &DiffLine| {
            match diff_line.change_tag {
                ChangeTag::Insert => {
                    count_until_changed = 0;
                }
                ChangeTag::Equal => {
                    count_until_changed += 1;
                }
                ChangeTag::Delete => {
                    count_until_changed = 0;
                }
            }

            let far_from_start = diff_line.count_from_change > DIFF_GAP_BUFFER_SIZE;
            let far_from_end = count_until_changed > DIFF_GAP_BUFFER_SIZE;
            !far_from_start || !far_from_end
        };

        let mut result = String::from("");
        let mut skipped_lines = 0;
        for diff_line in diff_lines.iter().rev() {
            let show_line = check_diff_gap(diff_line);
            if show_line {
                if skipped_lines > 0 {
                    result = format!("\n... {} lines ...\n\n", skipped_lines)
                        .bold()
                        .blue()
                        .to_string()
                        + &result;
                }
                skipped_lines = 0;
                let line = diff_line_to_string(diff_line, line_number_width);
                result = line + &result;
            } else {
                skipped_lines += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_diff() -> TextDiff<'static, 'static, 'static, str> {
        let old = r#"
        AAA
        BBB
        CCC
        "#;
        let new = r#"
        AAA
        CCC
        DDD
        "#;
        TextDiff::from_lines(old, new)
    }

    #[test]
    fn test_diff_analysis_any_changes_no_changes() {
        let old = "this is some text";
        let new = "this is some text";
        let diff = TextDiff::from_lines(old, new);

        assert!(!diff.any_changes())
    }

    #[test]
    fn test_diff_analysis_any_changes_deleted_line() {
        let old = r#"
        AAA
        BBB
        CCC
        "#;
        let new = r#"
        AAA
        CCC
        "#;
        let diff = TextDiff::from_lines(old, new);

        assert!(diff.any_changes())
    }

    #[test]
    fn test_diff_analysis_any_changes_added_line() {
        let diff = build_test_diff();
        assert!(diff.any_changes())
    }

    #[test]
    fn test_to_string() {
        colored::control::set_override(false);
        let diff = build_test_diff();
        let result = diff.to_string();

        let expected_lines = [
            "  1 1 | ",
            "  2 2 |         AAA",
            "- 3   |         BBB",
            "  4 3 |         CCC",
            "+   4 |         DDD",
            "  5 5 |         ",
            "",
        ];
        assert_eq!(result, expected_lines.join("\n"));
    }

    #[test]
    fn test_to_string_with_skipped_lines() {
        colored::control::set_override(false);
        let old = r#"[alias]
    lg = !git lg1
    lg1 = !git lg1-specific --all
    lg2 = !git lg2-specific --all
    lg3 = !git lg3-specific --all

[pager]
    branch = false

[core]
    editor = vim

[pull]
    rebase = false

[init]
    defaultBranch = main

[filter "lfs"]
    clean = git-lfs clean -- %f
    smudge = git-lfs smudge -- %f
    process = git-lfs filter-process
    required = true
"#;
        let new = r#"[alias]
    lg = !git lg1
    lg1 = !git lg1-specific --all

[pager]
    branch = false

[core]
    editor = vim

[pull]
    rebase = false

[init]
    defaultBranch = main

[filter "lfs"]
    clean = git-lfs clean -- %f
    smudge = git-lfs smudge -- %f
    process = git-lfs filter-process
    required = true

[fetch]
    prune = true
"#;
        let diff = TextDiff::from_lines(old, new);

        let result = diff.to_string();
        let expected_lines = [
            "   1  1 | [alias]",
            "   2  2 |     lg = !git lg1",
            "   3  3 |     lg1 = !git lg1-specific --all",
            "-  4    |     lg2 = !git lg2-specific --all",
            "-  5    |     lg3 = !git lg3-specific --all",
            "   6  4 | ",
            "   7  5 | [pager]",
            "   8  6 |     branch = false",
            "   9  7 | ",
            "",
            "... 10 lines ...",
            "",
            "  20 18 |     clean = git-lfs clean -- %f",
            "  21 19 |     smudge = git-lfs smudge -- %f",
            "  22 20 |     process = git-lfs filter-process",
            "  23 21 |     required = true",
            "+    22 | ",
            "+    23 | [fetch]",
            "+    24 |     prune = true",
            "",
        ];
        assert_eq!(result, expected_lines.join("\n"));
    }
}
