use std::path::PathBuf;

use yaml_rust::YamlLoader;

use crate::patina_yaml::FromYaml;

#[derive(Debug, Default, PartialEq)]
pub struct Patina {
    pub name: String,
    pub description: String,
    pub files: Vec<PatinaFile>,
}

#[derive(Debug, Default, PartialEq)]
pub struct PatinaFile {
    pub template: PathBuf,
    pub target: PathBuf,
}

#[derive(Debug)]
pub enum ValidationError {
    NameMissing,
    DescriptionMissing,
    FileTemplateMissing,
    FileTargetMissing,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    FileRead(std::io::Error),
    YamlParse(yaml_rust::ScanError),
    Validation(ValidationError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Patina {
    pub fn from_yaml_file(yaml_file_path: &PathBuf) -> Result<Vec<Patina>> {
        let yaml_str = match std::fs::read_to_string(yaml_file_path) {
            Ok(yaml_str) => yaml_str,
            Err(e) => return Err(Error::FileRead(e)),
        };

        let yaml = match YamlLoader::load_from_str(&yaml_str) {
            Ok(yaml) => yaml,
            Err(e) => return Err(Error::YamlParse(e)),
        };

        match yaml.iter().map(Patina::from_yaml).collect() {
            Ok(patina) => Ok(patina),
            Err(e) => Err(Error::Validation(e)),
        }
    }
}
