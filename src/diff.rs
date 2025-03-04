//! The diff module provides diffing behavior for files. This includes diff analysis and utilities
//! for displaying diff results.

use std::cmp::max;

use colored::{Color, Colorize};
use similar::{Change, ChangeTag, TextDiff};

/// [DiffAnalysis] provides functionality for diffs within dotpatina
pub trait DiffAnalysis {
    /// Determine if there are any changes in the diff
    fn any_changes(&self) -> bool;

    /// Get a String representation of the diff for display
    fn to_string(&self) -> String;
}

/// Details for a line output for a diff
struct DiffLine {
    /// The line number in the old file
    old_line_num: Option<usize>,

    /// The line number in the new file
    new_line_num: Option<usize>,

    /// A character indicating the type of change
    diff_char: char,

    /// The change tag for the line
    change_tag: ChangeTag,

    /// The string representation of the change
    change_string: String,

    /// The color to use for the line
    color: Option<Color>,

    /// The number of lines since the [DiffLine] that was a change
    count_from_change: isize,
}

impl DiffLine {
    fn to_string(&self, line_number_width: usize) -> std::string::String {
        let old_line_num = match self.old_line_num {
            Some(l) => l.to_string(),
            None => "".to_string(),
        };
        let new_line_num = match self.new_line_num {
            Some(l) => l.to_string(),
            None => "".to_string(),
        };
        let line = format!(
            "{} {: >num_width$} {: >num_width$} | {}",
            self.diff_char,
            old_line_num,
            new_line_num,
            self.change_string,
            num_width = line_number_width
        );
        match self.color {
            Some(color) => line.color(color).to_string(),
            None => line,
        }
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
        /// The number of unchanged lines to show around diff changes.
        /// This provides context to the viewer within the file.
        static DIFF_GAP_BUFFER_SIZE: isize = 4;

        // Keep a running count of line number for the old file
        // the line number will increase for the old file if lines were removed or are unchanged
        let mut old_line_count = 1;

        // Keep a running count of line number for the new file
        // the line number will increase for the new file if lines were inserted or are unchanged
        let mut new_line_count = 1;

        // Keep a count of the number of lines since a change happened.
        // If negative, there hasn't been a change yet.
        // This is used for determining what lines are far away enough from changes to skip displaying.
        let mut count_from_change = -1;

        // Keep track if there have been any changes at all.
        // If not, we can return early.
        let mut any_changes = false;

        // This closure is used for the first iteration over the changes.
        // It generates a DiffLine struct with relevant data for the diff
        // and keeps track of the variables defined above.
        let change_to_diff_line = |change: Change<&O>| match change.tag() {
            ChangeTag::Insert => {
                count_from_change = 0;
                any_changes = true;
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
                if count_from_change >= 0 {
                    count_from_change += 1;
                }
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
                any_changes = true;
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

        // Iterate through the changes and collect to a vector
        let mut diff_lines = self
            .iter_all_changes()
            .map(change_to_diff_line)
            .collect::<Vec<DiffLine>>();

        // If there aren't any changes, return early with a message noting the number of lines
        if !any_changes {
            return format!("{} lines, no changes detected\n", diff_lines.len())
                .bold()
                .blue()
                .to_string();
        }

        // Determine the width of the line number columns
        let line_number_width = max(
            old_line_count.to_string().len(),
            new_line_count.to_string().len(),
        );

        // Keep a count of the number of lines until a change happens.
        // If negative, there wont be another change.
        // This is used for determining what lines are far away enough from changes to skip displaying.
        let mut count_to_changed = -1;

        // This closure is used in the second (reversed) iteration through the diff lines.
        // It updates the count_to_changed, and uses that value to determine (and return)
        // whether or not this line should be displayed.
        let mut check_diff_gap = |diff_line: &mut DiffLine| {
            match diff_line.change_tag {
                ChangeTag::Insert => {
                    count_to_changed = 0;
                }
                ChangeTag::Equal => {
                    if count_to_changed >= 0 {
                        count_to_changed += 1;
                    }
                }
                ChangeTag::Delete => {
                    count_to_changed = 0;
                }
            }

            let far_from_start = diff_line.count_from_change > DIFF_GAP_BUFFER_SIZE
                || diff_line.count_from_change < 0;
            let far_from_end = count_to_changed > DIFF_GAP_BUFFER_SIZE || count_to_changed < 0;

            // The line should be displayed if it is not far (within DIFF_GAP_BUFFER_SIZE lines)
            // from the start or end of a change.
            !far_from_start || !far_from_end
        };

        // Create a variable for the resulting string
        let mut result = String::from("");

        // Keep a running count of skipped lines
        let mut skipped_lines = 0;

        // Iterate through the list for the second time in reverse.
        // This is reversed so that we can track count_to_changed properly.
        for diff_line in diff_lines.iter_mut().rev() {
            let show_line = check_diff_gap(diff_line);
            if show_line {
                // If we are showing a line, but have been skipping lines,
                // display the number of unchanged lines
                if skipped_lines > 0 {
                    result = format!("\n... {} unchanged lines\n\n", skipped_lines)
                        .bold()
                        .blue()
                        .to_string()
                        + &result;
                }
                skipped_lines = 0;

                // Add the currnet line string to beginning the result.
                // This reverses the reverse iteration.
                let line = diff_line.to_string(line_number_width);
                result = line + &result;
            } else {
                skipped_lines += 1;
            }
        }

        // If we finished the second iteration and were skipping lines,
        // then display the number of unchanged lines again.
        // This is the case when there aren't any changes at the beginning of a file.
        if skipped_lines > 0 {
            result = format!("\n... {} unchanged lines\n\n", skipped_lines)
                .bold()
                .blue()
                .to_string()
                + &result;
        }

        // Finally, return the result
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
    fn test_diff_to_string() {
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
    fn test_diff_to_string_with_skipped_lines() {
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
            "... 10 unchanged lines",
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

    #[test]
    fn test_diff_to_string_with_skipped_lines_at_the_start_and_end() {
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
    lg2 = !git lg2-specific --all
    lg3 = !git lg3-specific --all

[pager]
    branch = false

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

        let diff = TextDiff::from_lines(old, new);

        let result = diff.to_string();
        let expected_lines = [
            "",
            "... 5 unchanged lines",
            "",
            "   6  6 | ",
            "   7  7 | [pager]",
            "   8  8 |     branch = false",
            "   9  9 | ",
            "- 10    | [core]",
            "- 11    |     editor = vim",
            "- 12    | ",
            "  13 10 | [pull]",
            "  14 11 |     rebase = false",
            "  15 12 | ",
            "  16 13 | [init]",
            "",
            "... 7 unchanged lines",
            "",
            "",
        ];
        assert_eq!(result, expected_lines.join("\n"));
    }

    #[test]
    fn test_diff_to_string_no_changes() {
        colored::control::set_override(false);
        let old = r#"aaa
        bbb
        ccc"#;
        let new = r#"aaa
        bbb
        ccc"#;

        let diff = TextDiff::from_lines(old, new);

        let result = diff.to_string();
        let expected_lines = ["3 lines, no changes detected", ""];
        assert_eq!(result, expected_lines.join("\n"));
    }
}
