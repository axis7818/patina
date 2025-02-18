use colored::Colorize;
use similar::{ChangeTag, TextDiff};

/// DiffAnalysis provides functionality for diffs within dotpatina
pub trait DiffAnalysis {
    /// Determine if there are any changes in the diff
    fn any_changes(&self) -> bool;

    /// Get a String representation of the diff for display
    fn to_string(&self) -> String;
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
        self.iter_all_changes()
            .map(|change| match change.tag() {
                ChangeTag::Insert => format!("{} {}", "+".bold(), change).green().to_string(),
                ChangeTag::Equal => format!("{} {}", "|".bold(), change).to_string(),
                ChangeTag::Delete => format!("{} {}", "-".bold(), change).red().to_string(),
            })
            .reduce(|result, change| result + &change)
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_diff() -> TextDiff<'static, 'static, 'static, str> {
        let old = r#"
        aaa
        bbb
        ccc
        "#;
        let new = r#"
        AAA
        BBB
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
        assert_eq!(
            "| \n-         aaa\n-         bbb\n-         ccc\n+         AAA\n+         BBB\n+         CCC\n+         DDD\n|         \n",
            result
        );
    }
}
