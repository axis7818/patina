use std::path::PathBuf;

use enum_as_inner::EnumAsInner;

/// An enum representing all possible errors that can occur in this crate
#[allow(dead_code)]
#[derive(Debug, EnumAsInner)]
pub enum Error {
    /// An error that occurs when a file cannot be read
    FileRead(PathBuf, std::io::Error),

    /// An error that occurs when a file cannot be written
    FileWrite(PathBuf, std::io::Error),

    /// An error that occurs when parsing Toml data
    TomlParse(toml::de::Error),

    /// An error that occurs when rendering a handlebars template
    RenderTemplate(handlebars::RenderError),
}

/// A Result type that uses the [`Error`] enum
pub type Result<T> = std::result::Result<T, Error>;
