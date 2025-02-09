use handlebars::Handlebars;

use crate::patina::{Patina, PatinaFile};
use crate::utils::{Error, Result};

pub fn render_patina(patina: &Patina) -> Result<Vec<String>> {
    println!("rendering patina");
    let hb = Handlebars::new();
    patina
        .files
        .iter()
        .map(|pf| render_patina_file(&hb, patina, pf))
        .collect()
}

fn render_patina_file(
    hb: &Handlebars,
    patina: &Patina,
    patina_file: &PatinaFile,
) -> Result<String> {
    println!("rendering patina file: {}", patina_file.template.display());

    let template_str = patina_file.load_template_file_as_string()?;

    match hb.render_template(&template_str, &patina.vars) {
        Ok(render) => Ok(render),
        Err(e) => Err(Error::RenderTemplate(e)),
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf};

    use super::*;

    #[test]
    fn test_render_patina() {
        let patina = Patina {
            name: String::from("sample-patina"),
            description: String::from("This is a sample Patina"),
            vars: [(String::from("name"), String::from("Patina"))]
                .iter()
                .cloned()
                .collect::<HashMap<String, String>>(),
            files: vec![PatinaFile {
                template: PathBuf::from("tests/fixtures/template.txt.hbs"),
                target: PathBuf::from("tests/fixtures/template.txt"),
            }],
        };

        let render = render_patina(&patina);

        assert!(render.is_ok());
        let render = render.unwrap();
        assert_eq!(render.len(), 1);
        let render = &render[0];

        let expected = r#"Hello, Patina!

This is an example Patina template file.

Templates use the Handebars templating language. For more information, see <https://handlebarsjs.com/guide/>.
"#;
        assert_eq!(expected, render);
    }
}
