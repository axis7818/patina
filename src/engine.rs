use colored::Colorize;
use std::{fs, path::PathBuf};

use log::info;
use similar::{ChangeTag, TextDiff};

use crate::{
    patina::Patina,
    templating::render_patina,
    utils::{Error, Result},
};

/// Renders a Patina from a Patina toml file path.
pub fn render_patina_from_file(patina_path: PathBuf) -> Result<Vec<String>> {
    let patina = Patina::from_toml_file(&patina_path)?;

    info!("got patina: {:#?}", patina);

    render_patina(&patina)
}

pub fn apply_patina_from_file(patina_path: PathBuf) -> Result<Vec<String>> {
    let patina = Patina::from_toml_file(&patina_path)?;

    info!("got patina: {:#?}", patina);

    let render = render_patina(&patina)?;

    #[allow(clippy::needless_range_loop)]
    for i in 0..render.len() {
        let render_str = &render[i];
        let target_file = &patina.files[i].target;

        let target_file_str = fs::read_to_string(target_file).unwrap_or_default();
        let diff = TextDiff::from_lines(&target_file_str, render_str);
        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Insert => print!("{}", format!("+ {}", change).green().bold()),
                ChangeTag::Equal => print!("{}", format!("  {}", change).bold()),
                ChangeTag::Delete => print!("{}", format!("- {}", change).red().bold()),
            }
        }

        if let Err(e) = fs::write(target_file, render_str) {
            return Err(Error::FileWrite(target_file.clone(), e));
        }
    }

    Ok(render)
}

#[cfg(test)]
mod tests {
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

        let render = render_patina_from_file(patina_path);

        assert!(render.is_ok());
        let render = &render.unwrap()[0];

        let expected = r#"Hello, Patina User!

This is an example Patina template file.

Templates use the Handebars templating language. For more information, see <https://handlebarsjs.com/guide/>.
"#;
        assert_eq!(expected, render);
    }

    #[test]
    fn test_render_patina_from_file_failed_file_load() {
        let patina_path = PathBuf::from("this/path/does/not/exist.toml");

        let render = render_patina_from_file(patina_path);
        assert!(render.is_err());
        assert!(render.unwrap_err().is_file_read());
    }

    #[test]
    fn test_render_patina_from_file_render_fails() {
        let patina_path = PathBuf::from("tests/fixtures/missing_template_patina.toml");

        let render = render_patina_from_file(patina_path);
        assert!(render.is_err());
        assert!(render.unwrap_err().is_file_read());
    }

    #[test]
    fn test_apply_patina_from_file() {
        let patina_path = PathBuf::from("tests/fixtures/template_patina.toml");

        let render = apply_patina_from_file(patina_path);

        assert!(render.is_ok());

        let expected = r#"Hello, Patina User!

This is an example Patina template file.

Templates use the Handebars templating language. For more information, see <https://handlebarsjs.com/guide/>.
"#;

        let applied_file_path = TestTargetFile::new("tests/fixtures/template.txt");

        let applied_file = fs::read_to_string(&applied_file_path.target);
        assert!(applied_file.is_ok());
        assert_eq!(applied_file.unwrap(), expected);
    }

    #[test]
    fn test_apply_patina_from_file_write_failed() {
        let patina_path = PathBuf::from("tests/fixtures/invalid_target_template_patina.toml");

        let render = apply_patina_from_file(patina_path);

        assert!(render.is_err());
        assert!(render.unwrap_err().is_file_write());
    }
}
