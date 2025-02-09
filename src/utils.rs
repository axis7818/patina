use enum_as_inner::EnumAsInner;

#[allow(dead_code)]
#[derive(Debug, EnumAsInner)]
pub enum Error {
    FileRead(std::io::Error),
    TomlParse(toml::de::Error),
    RenderTemplate(handlebars::RenderError),
}

pub type Result<T> = std::result::Result<T, Error>;
