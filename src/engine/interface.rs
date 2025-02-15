use std::path::Path;

use colored::Colorize;
use similar::{ChangeTag, TextDiff};

/// PatinaOutput specifies operations for displaying output from the Patina engine
pub trait PatinaOutput {
    /// Output a single string
    fn output(&self, s: &str);

    /// Output a patina render
    fn output_file_header(&self, template_path: &Path) {
        let template_path = template_path.display().to_string();
        self.output(
            &format!("{}\n", "=".repeat(template_path.len() + 16))
                .yellow()
                .bold()
                .to_string(),
        );
        self.output(
            &format!("> Patina file {} <\n", template_path)
                .yellow()
                .bold()
                .to_string(),
        );
        self.output(
            &format!("{}\n", "=".repeat(template_path.len() + 16))
                .yellow()
                .bold()
                .to_string(),
        );
    }

    /// Output a diff view
    fn output_diff<'a>(&self, diff: &TextDiff<'a, 'a, 'a, str>) {
        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Insert => {
                    self.output(&format!("+ {}", change).green().bold().to_string())
                }
                ChangeTag::Equal => self.output(&format!("| {}", change).bold().to_string()),
                ChangeTag::Delete => self.output(&format!("- {}", change).red().bold().to_string()),
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use std::cell::RefCell;

    use super::*;

    pub struct TestPatinaOutput {
        pub lines: RefCell<Vec<String>>,
    }

    impl TestPatinaOutput {
        pub fn new() -> TestPatinaOutput {
            colored::control::set_override(false);
            TestPatinaOutput {
                lines: RefCell::new(vec![]),
            }
        }

        pub fn get_all_output(self) -> String {
            self.lines.into_inner().join("")
        }
    }

    impl PatinaOutput for TestPatinaOutput {
        fn output(&self, s: &str) {
            self.lines.borrow_mut().push(s.to_string());
        }
    }
}
