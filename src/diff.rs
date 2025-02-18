use std::cmp::max;

use colored::{Color, Colorize};
use similar::{ChangeTag, TextDiff};

/// DiffAnalysis provides functionality for diffs within dotpatina
pub trait DiffAnalysis {
    /// Determine if there are any changes in the diff
    fn any_changes(&self) -> bool;

    /// Get a String representation of the diff for display
    fn to_string(&self) -> String;
}

type DiffLine = (Option<usize>, Option<usize>, char, String, Option<Color>);

impl<'lines, O: ?Sized> DiffAnalysis for TextDiff<'lines, 'lines, '_, O>
where
    O: similar::DiffableStr,
{
    fn any_changes(&self) -> bool {
        self.iter_all_changes()
            .any(|change| change.tag() != ChangeTag::Equal)
    }

    fn to_string(&self) -> String {
        let mut old_line_count = 1;
        let mut new_line_count = 1;

        let diff_lines = self
            .iter_all_changes()
            .map(|change| match change.tag() {
                ChangeTag::Insert => {
                    let line: DiffLine = (
                        None,
                        Some(new_line_count),
                        '+',
                        change.to_string(),
                        Some(Color::Green),
                    );
                    new_line_count += 1;
                    line
                }
                ChangeTag::Equal => {
                    let line: DiffLine = (
                        Some(old_line_count),
                        Some(new_line_count),
                        ' ',
                        change.to_string(),
                        None,
                    );
                    old_line_count += 1;
                    new_line_count += 1;
                    line
                }
                ChangeTag::Delete => {
                    let line: DiffLine = (
                        Some(old_line_count),
                        None,
                        '-',
                        change.to_string(),
                        Some(Color::Red),
                    );
                    old_line_count += 1;
                    line
                }
            })
            .collect::<Vec<DiffLine>>();

        let line_number_width = max(
            old_line_count.to_string().len(),
            new_line_count.to_string().len(),
        );

        diff_lines
            .iter()
            .map(|diff_line| {
                let old_line_num = match diff_line.0 {
                    Some(l) => l.to_string(),
                    None => "".to_string(),
                };
                let new_line_num = match diff_line.1 {
                    Some(l) => l.to_string(),
                    None => "".to_string(),
                };
                let line = format!(
                    "{} {: >num_width$} {: >num_width$} | {}",
                    diff_line.2,
                    old_line_num,
                    new_line_num,
                    diff_line.3,
                    num_width = line_number_width
                );
                match diff_line.4 {
                    Some(color) => line.color(color).to_string(),
                    None => line,
                }
            })
            .reduce(|result, line| result + &line)
            .unwrap_or_default()
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
}
