use handlebars::Handlebars;

use crate::errors::{Error, Result};
use crate::patina::{Patina, PatinaFile};

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
