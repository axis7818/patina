use std::fs;

use handlebars::Handlebars;
use log::info;

use crate::patina::patina_file::PatinaFile;
use crate::patina::Patina;
use crate::utils::{Error, Result};

#[derive(Debug)]
pub struct PatinaFileRender<'pf> {
    pub patina_file: &'pf PatinaFile,
    pub render_str: String,
}

/// Renders all of the files in a Patina, each to a string in the result vector.
pub fn render_patina(patina: &Patina, tags: Option<Vec<String>>) -> Result<Vec<PatinaFileRender>> {
    let mut hb = Handlebars::new();
    hb.register_escape_fn(handlebars::no_escape);
    hb.set_strict_mode(true);

    patina
        .files_for_tags(tags)
        .map(|pf| {
            let render = render_patina_file(&hb, patina, pf)?;
            Ok(PatinaFileRender {
                patina_file: pf,
                render_str: render,
            })
        })
        .collect()
}

fn render_patina_file(
    hb: &Handlebars,
    patina: &Patina,
    patina_file: &PatinaFile,
) -> Result<String> {
    info!("rendering patina file: {}", patina_file.template.display());

    let template_path = patina.get_patina_path(&patina_file.template);
    let template_str = match fs::read_to_string(&template_path) {
        Ok(template_str) => template_str,
        Err(e) => return Err(Error::FileRead(template_path, e)),
    };

    match hb.render_template(&template_str, &patina.vars) {
        Ok(render) => Ok(render),
        Err(mut e) => {
            e.template_name = Some(patina_file.template.display().to_string());
            Err(Error::RenderTemplate(e))
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_render_patina() {
        let patina = Patina {
            base_path: None,
            name: String::from("sample-patina"),
            description: String::from("This is a sample Patina"),
            vars: Some(json!({
                "name": {
                    "first": "Patina",
                    "last": "User"
                }
            })),
            files: vec![PatinaFile::new(
                "tests/fixtures/template.txt.hbs",
                "tests/fixtures/template.txt",
            )],
        };

        let render = render_patina(&patina, None);

        assert!(render.is_ok());
        let render = render.unwrap();
        assert_eq!(render.len(), 1);
        let render = &render[0];

        assert_eq!(render.patina_file, &patina.files[0]);

        assert_eq!(
            render.render_str,
            r#"Hello, Patina User!
This is an example Patina template file.
Templates use the Handebars templating language. For more information, see <https://handlebarsjs.com/guide/>.
"#
        );
    }

    #[test]
    fn test_render_patina_multiple_templates() {
        let patina = Patina {
            base_path: None,
            name: String::from("multi-template-patina"),
            description: String::from("This is a patina with multiple templates"),
            vars: Some(json!({
                "A": "template_a",
                "B": "template_b",
                "C": "template_c",
            })),
            files: vec![
                PatinaFile::new("tests/fixtures/template_a.txt.hbs", "output_a.txt"),
                PatinaFile::new("tests/fixtures/template_b.txt.hbs", "output_b.txt"),
                PatinaFile::new("tests/fixtures/template_c.txt.hbs", "output_c.txt"),
            ],
        };

        let render = render_patina(&patina, None);

        assert!(render.is_ok());
        let render = render.unwrap();

        assert_eq!(render.len(), 3);
        assert_eq!(render[0].render_str, "This is template_a.\n");
        assert_eq!(render[1].render_str, "This is template_b.\n");
        assert_eq!(render[2].render_str, "This is template_c.\n");
    }

    #[test]
    fn test_render_patina_missing_variable() {
        let patina = Patina {
            base_path: None,
            name: String::from("sample-patina"),
            description: String::from("This is a sample Patina"),
            vars: Some(json!({})),
            files: vec![PatinaFile::new(
                "tests/fixtures/template.txt.hbs",
                "tests/fixtures/template.txt",
            )],
        };

        let render = render_patina(&patina, None);
        assert!(render.is_err());
        let render = render.unwrap_err();
        assert!(render.is_render_template());
        let err = render.as_render_template().unwrap();
        assert_eq!(
            err.reason().to_string(),
            "Failed to access variable in strict mode Some(\"name.first\")"
        );
    }

    #[test]
    fn test_render_patina_invalid_template() {
        let patina = Patina {
            base_path: None,
            name: String::from("sample-patina"),
            description: String::from("This is a sample Patina"),
            vars: Some(json!({})),
            files: vec![PatinaFile::new(
                "tests/fixtures/invalid_template.txt.hbs",
                "tests/fixtures/template.txt",
            )],
        };

        let render = render_patina(&patina, None);

        assert!(render.is_err());
        assert!(render.unwrap_err().is_render_template());
    }

    #[test]
    fn test_render_patina_escaped_handlebars() {
        let patina = Patina {
            name: "escaped_handlebars".to_string(),
            description: "this patina shows escaping handlebars".to_string(),
            base_path: None,
            vars: None,
            files: vec![PatinaFile::new(
                "tests/fixtures/template_with_escaped_handlebars.hbs",
                "tests/fixtures/output.txt",
            )],
        };

        let render = render_patina(&patina, None);
        assert!(render.is_ok());
        assert_eq!(
            render.unwrap()[0].render_str,
            "This file has {{ escaped }} handlebars\n"
        );
    }
}
