use similar::{ChangeTag, TextDiff};

pub trait DiffAnalysis {
    fn any_changes(&self) -> bool;
}

impl<'lines, O: ?Sized> DiffAnalysis for TextDiff<'lines, 'lines, '_, O>
where
    O: similar::DiffableStr,
{
    fn any_changes(&self) -> bool {
        self.iter_all_changes()
            .any(|change| change.tag() != ChangeTag::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let old = r#"
        AAA
        BBB
        CCC
        "#;
        let new = r#"
        AAA
        BBB
        CCC
        DDD
        "#;
        let diff = TextDiff::from_lines(old, new);

        assert!(diff.any_changes())
    }
}
