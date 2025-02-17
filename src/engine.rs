use std::{
    fs,
    path::{Path, PathBuf},
};

use colored::Colorize;
use interface::PatinaInterface;
use log::info;
use similar::TextDiff;

pub mod interface;

use crate::{
    diff::DiffAnalysis,
    patina::Patina,
    templating,
    utils::{Error, Result},
};

/// The PatinaEngine is the main driver of logic for dotpatina operations
pub struct PatinaEngine<'a, PI>
where
    PI: PatinaInterface,
{
    /// A reference to the PatinaInterface that defines how to interact with the user via input & output
    pi: &'a PI,

    /// The path to the patina file on disk
    patina_path: PathBuf,

    /// The set of tags to filter on
    tags: Vec<String>,
}

impl<'a, PI> PatinaEngine<'a, PI>
where
    PI: PatinaInterface,
{
    /// Create a new PatinaEngine
    pub fn new(pi: &'a PI, patina_path: &Path, tags: Vec<String>) -> PatinaEngine<'a, PI> {
        PatinaEngine {
            pi,
            patina_path: patina_path.to_path_buf(),
            tags,
        }
    }

    /// Renders a Patina
    pub fn render_patina(&self) -> Result<()> {
        let patina = Patina::from_toml_file(&self.patina_path)?;
        info!("got patina: {:#?}", patina);
        let render = templating::render_patina(&patina)?;

        self.pi
            .output(format!("Rendered {} files\n\n", render.len()));
        for (i, r) in render.iter().enumerate() {
            let template_file = &patina.files[i].template;

            self.pi.output_file_header(template_file);
            self.pi.output(format!("{}\n", r));
        }

        Ok(())
    }

    /// Applies all of the Patina files
    pub fn apply_patina(&self) -> Result<()> {
        let patina = Patina::from_toml_file(&self.patina_path)?;
        info!("got patina: {:#?}", patina);
        let render = templating::render_patina(&patina)?;

        let mut any_changes = false;

        // Generate and display diffs
        for (i, r) in render.iter().enumerate() {
            let target_file = &patina.files[i].target;
            let target_path = patina.get_patina_path(target_file);

            let target_file_str = fs::read_to_string(&target_path).unwrap_or_default();
            let diff = TextDiff::from_lines(&target_file_str, r);
            if diff.any_changes() {
                any_changes = true
            }

            self.pi.output_file_header(&target_path);
            self.pi.output_diff(&diff);
            self.pi.output("\n");
        }

        // If there are not changes, quit
        if !any_changes {
            self.pi.output("No file changes detected in the patina");
            return Ok(());
        }

        // Get user confirmation to continue
        if self.pi.is_input_enabled() && !self.pi.confirm_apply()? {
            self.pi.output("Not applying patina.");
            return Ok(());
        }

        // Write out all files
        self.pi.output("\nApplying patina files\n");
        for (i, r) in render.iter().enumerate() {
            let target_file = &patina.files[i].target;
            let target_path = patina.get_patina_path(target_file);

            self.pi.output(format!("   {}", target_path.display()));
            if let Some(target_parent) = target_path.parent() {
                if let Err(e) = fs::create_dir_all(target_parent) {
                    return Err(Error::FileWrite(target_path, e));
                }
            }
            if let Err(e) = fs::write(&target_path, r) {
                return Err(Error::FileWrite(target_path.clone(), e));
            }
            self.pi.output(" ✓\n".green().to_string());
        }

        self.pi.output("Done\n");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::interface::test::TestPatinaInterface;

    use super::*;

    struct TestTargetFile {
        target: PathBuf,
    }

    impl TestTargetFile {
        fn new(target_file_path: &str) -> TestTargetFile {
            let target = PathBuf::from(target_file_path);
            let _ = fs::remove_file(&target);
            TestTargetFile { target }
        }
    }

    impl Drop for TestTargetFile {
        fn drop(&mut self) {
            let _ = fs::remove_file(self.target.clone());
        }
    }

    #[test]
    fn test_render_patina() {
        let patina_path = PathBuf::from("tests/fixtures/template_patina.toml");
        let pi = TestPatinaInterface::new();
        let engine = PatinaEngine::new(&pi, &patina_path, vec![]);

        let render = engine.render_patina();

        assert!(render.is_ok());

        assert_eq!(
            pi.get_all_output(),
            r#"Rendered 1 files

====================
> template.txt.hbs <
====================
Hello, Patina User!
This is an example Patina template file.
Templates use the Handebars templating language. For more information, see <https://handlebarsjs.com/guide/>.

"#
        );
    }

    #[test]
    fn test_render_patina_failed_file_load() {
        let patina_path = PathBuf::from("this/path/does/not/exist.toml");
        let pi = TestPatinaInterface::new();
        let engine = PatinaEngine::new(&pi, &patina_path, vec![]);

        let render = engine.render_patina();
        assert!(render.is_err());
        assert!(render.unwrap_err().is_file_read());
    }

    #[test]
    fn test_render_patina_render_fails() {
        let patina_path = PathBuf::from("tests/fixtures/missing_template_patina.toml");
        let pi = TestPatinaInterface::new();
        let engine = PatinaEngine::new(&pi, &patina_path, vec![]);

        let render = engine.render_patina();
        assert!(render.is_err());
        assert!(render.unwrap_err().is_file_read());
    }

    #[test]
    fn test_apply_patina() {
        let patina_path = PathBuf::from("tests/fixtures/template_patina.toml");
        let applied_file_path = TestTargetFile::new("tests/fixtures/template.txt");
        let pi = TestPatinaInterface::new();
        let engine = PatinaEngine::new(&pi, &patina_path, vec![]);

        let apply = engine.apply_patina();

        assert!(apply.is_ok());

        assert_eq!(
            pi.get_all_output(),
            r#"===============================
> tests/fixtures/template.txt <
===============================
+ Hello, Patina User!
+ This is an example Patina template file.
+ Templates use the Handebars templating language. For more information, see <https://handlebarsjs.com/guide/>.


Applying patina files
   tests/fixtures/template.txt ✓
Done
"#
        );

        let applied_file = fs::read_to_string(&applied_file_path.target);
        assert!(applied_file.is_ok());
        assert_eq!(
            applied_file.unwrap(),
            r#"Hello, Patina User!
This is an example Patina template file.
Templates use the Handebars templating language. For more information, see <https://handlebarsjs.com/guide/>.
"#
        );
    }

    #[test]
    fn test_apply_patina_abort_without_user_confirmation() {
        let patina_path = PathBuf::from("tests/fixtures/template_patina.toml");
        let mut pi = TestPatinaInterface::new();
        pi.confirm_apply = false;
        let engine = PatinaEngine::new(&pi, &patina_path, vec![]);

        let apply = engine.apply_patina();

        assert!(apply.is_ok());
        assert!(pi.get_all_output().contains("Not applying patina."))
    }

    #[test]
    fn test_apply_patina_does_nothing_if_there_are_no_changes() {
        let patina_path = PathBuf::from("tests/fixtures/no_files_patina.toml");
        let pi = TestPatinaInterface::new();
        let engine = PatinaEngine::new(&pi, &patina_path, vec![]);
        let apply = engine.apply_patina();

        assert!(apply.is_ok());
        assert!(pi
            .get_all_output()
            .contains("No file changes detected in the patina"));
    }
}
