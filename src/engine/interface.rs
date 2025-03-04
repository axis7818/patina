//! This module contains definitions for interfacing with [super::PatinaEngine]

use std::path::Path;

use colored::Colorize;
use similar::TextDiff;

use crate::{
    diff::DiffAnalysis,
    utils::{Error, Result},
};

/// Specifies operations for interfacing with [super::PatinaEngine]
pub trait PatinaInterface {
    /// Output a single string
    fn output<S>(&self, s: S)
    where
        S: Into<String>;

    /// Set whether or not input is enabled
    fn set_is_input_enabled(&mut self, value: bool);

    /// Get whether or not input is enabled
    fn is_input_enabled(&self) -> bool;

    /// Prompts the user for confirmation to apply the patina
    fn confirm_apply(&self) -> Result<bool> {
        self.output("Do you want to continue? (y/n): ");
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim().to_lowercase() != "y" {
                    return Ok(false);
                }
            }
            Err(e) => return Err(Error::GetUserInput(e)),
        }

        Ok(true)
    }

    /// Output a patina render
    fn output_file_header(&self, template_path: &Path) {
        let template_path = template_path.display().to_string();
        self.output(
            format!("\n{}\n", template_path)
                .yellow()
                .bold()
                .underline()
                .to_string(),
        );
    }

    /// Output a diff view
    fn output_diff<'a>(&self, diff: &TextDiff<'a, 'a, 'a, str>) {
        self.output(diff.to_string());
    }
}

#[cfg(test)]
pub mod test {
    use std::cell::RefCell;

    use super::*;

    pub struct TestPatinaInterface {
        pub confirm_apply: bool,
        is_input_enabled: bool,
        pub lines: RefCell<Vec<String>>,
    }

    impl TestPatinaInterface {
        pub fn new() -> TestPatinaInterface {
            colored::control::set_override(false);

            TestPatinaInterface {
                confirm_apply: true,
                is_input_enabled: true,
                lines: RefCell::new(vec![]),
            }
        }

        pub fn get_all_output(self) -> String {
            self.lines.into_inner().join("")
        }
    }

    impl PatinaInterface for TestPatinaInterface {
        fn output<S>(&self, s: S)
        where
            S: Into<String>,
        {
            self.lines.borrow_mut().push(s.into());
        }

        fn confirm_apply(&self) -> Result<bool> {
            Ok(self.confirm_apply)
        }

        fn set_is_input_enabled(&mut self, value: bool) {
            self.is_input_enabled = value
        }

        fn is_input_enabled(&self) -> bool {
            self.is_input_enabled
        }
    }
}
