#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    FileRead(std::io::Error),
    TomlParse(toml::de::Error),
    RenderTemplate(handlebars::RenderError),
}

pub type Result<T> = std::result::Result<T, Error>;
