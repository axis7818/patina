use std::{fs, path::PathBuf};

use interface::PatinaInterface;
use log::info;
use similar::TextDiff;

pub mod interface;

use crate::{
    patina::Patina,
    templating::render_patina,
    utils::{Error, Result},
};

/// Renders a Patina from a Patina toml file path.
pub fn render_patina_from_file<PI: PatinaInterface>(patina_path: &PathBuf, pi: &PI) -> Result<()> {
    let patina = Patina::from_toml_file(patina_path)?;

    info!("got patina: {:#?}", patina);
    let render = render_patina(&patina)?;

    pi.output(format!("Rendered {} files\n\n", render.len()));
    for (i, r) in render.iter().enumerate() {
        let template_file = &patina.files[i].template;

        pi.output_file_header(template_file);
        pi.output(format!("{}\n", r));
    }

    Ok(())
}

/// Applies all of the Patina files
pub fn apply_patina_from_file<PI: PatinaInterface>(patina_path: &PathBuf, pi: &PI) -> Result<()> {
    let patina = Patina::from_toml_file(patina_path)?;

    info!("got patina: {:#?}", patina);

    let render = render_patina(&patina)?;

    // Generate and display diffs
    for (i, r) in render.iter().enumerate() {
        let target_file = &patina.files[i].target;

        let target_file_str = fs::read_to_string(target_file).unwrap_or_default();
        let diff = TextDiff::from_lines(&target_file_str, r);
        pi.output_file_header(target_file);
        pi.output_diff(&diff);
        pi.output("\n");
    }

    // Get user confirmation to continue
    if !pi.confirm_apply()? {
        pi.output("Not applying patina.");
        return Ok(());
    }

    // Write out all files
    pi.output("Applying patina files...\n");
    for (i, r) in render.iter().enumerate() {
        let target_file = &patina.files[i].target;
        if let Err(e) = fs::write(target_file, r) {
            return Err(Error::FileWrite(target_file.clone(), e));
        }
    }

    pi.output("Done\n");
    Ok(())
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
            TestTargetFile {
                target: PathBuf::from(target_file_path),
            }
        }
    }

    impl Drop for TestTargetFile {
        fn drop(&mut self) {
            let _ = fs::remove_file(self.target.clone());
        }
    }

    #[test]
    fn test_render_patina_from_file() {
        let patina_path = PathBuf::from("tests/fixtures/template_patina.toml");
        let output = TestPatinaInterface::new();

        let render = render_patina_from_file(&patina_path, &output);

        assert!(render.is_ok());

        assert_eq!(
            output.get_all_output(),
            r#"Rendered 1 files

===============================================
> Patina file tests/fixtures/template.txt.hbs <
===============================================
Hello, Patina User!
This is an example Patina template file.
Templates use the Handebars templating language. For more information, see <https://handlebarsjs.com/guide/>.

"#
        );
    }

    #[test]
    fn test_render_patina_from_file_failed_file_load() {
        let patina_path = PathBuf::from("this/path/does/not/exist.toml");

        let render = render_patina_from_file(&patina_path, &TestPatinaInterface::new());
        assert!(render.is_err());
        assert!(render.unwrap_err().is_file_read());
    }

    #[test]
    fn test_render_patina_from_file_render_fails() {
        let patina_path = PathBuf::from("tests/fixtures/missing_template_patina.toml");

        let render = render_patina_from_file(&patina_path, &TestPatinaInterface::new());
        assert!(render.is_err());
        assert!(render.unwrap_err().is_file_read());
    }

    #[test]
    fn test_apply_patina_from_file() {
        let patina_path = PathBuf::from("tests/fixtures/template_patina.toml");

        let pi = TestPatinaInterface::new();
        let render = apply_patina_from_file(&patina_path, &pi);

        assert!(render.is_ok());

        assert_eq!(
            pi.get_all_output(),
            r#"===========================================
> Patina file tests/fixtures/template.txt <
===========================================
+ Hello, Patina User!
+ This is an example Patina template file.
+ Templates use the Handebars templating language. For more information, see <https://handlebarsjs.com/guide/>.

Applying patina files...
Done
"#
        );

        let applied_file_path = TestTargetFile::new("tests/fixtures/template.txt");

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
    fn test_apply_patina_from_file_write_failed() {
        let patina_path = PathBuf::from("tests/fixtures/invalid_target_template_patina.toml");

        let render = apply_patina_from_file(&patina_path, &TestPatinaInterface::new());

        assert!(render.is_err());
        assert!(render.unwrap_err().is_file_write());
    }
}
