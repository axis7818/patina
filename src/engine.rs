use std::path::PathBuf;

use crate::{errors::Result, patina::Patina, templating::render_patina};

pub fn render_patina_from_file(patina_path: PathBuf) -> Result<Vec<String>> {
    let patina = match Patina::from_toml_file(&patina_path) {
        Ok(patina) => patina,
        Err(e) => panic!("{:?}", e),
    };

    println!("got patina: {:#?}", patina);

    render_patina(&patina)
}
