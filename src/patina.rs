#![allow(unused_imports)]

mod patina;
mod patina_file;

#[derive(Debug)]
#[allow(dead_code)]
pub enum PatinaError {
    FileReadError(std::io::Error),
    YamlParseError(yaml_rust::ScanError),
    NameMissing,
    DescriptionMissing,
    FileTemplateMissing,
    FileTargetMissing,
}

type Result<T> = std::result::Result<T, PatinaError>;

pub use patina::Patina;
pub use patina_file::PatinaFile;
