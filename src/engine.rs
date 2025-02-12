use std::path::PathBuf;

use log::info;

use crate::{patina::Patina, templating::render_patina, utils::Result};

/// Renders a Patina from a Patina toml file path.
pub fn render_patina_from_file(patina_path: PathBuf) -> Result<Vec<String>> {
    let patina = Patina::from_toml_file(&patina_path)?;

    info!("got patina: {:#?}", patina);

    render_patina(&patina)
}

pub fn apply_patina_from_file(patina_path: PathBuf) -> Result<Vec<String>> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_patina_from_file() {
        let patina_path = PathBuf::from("tests/fixtures/template_patina.toml");

        let render = render_patina_from_file(patina_path);
        let render = &render.unwrap()[0];

        let expected = r#"Hello, Patina!

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
}
