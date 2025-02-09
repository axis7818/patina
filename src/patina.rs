use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Patina {
    pub name: String,

    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub files: Vec<PatinaFile>,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PatinaFile {
    pub template: PathBuf,
    pub target: PathBuf,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    FileRead(std::io::Error),
    YamlParse(yaml_rust::ScanError),
    TomlParse(toml::de::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Patina {
    pub fn from_toml_file(toml_file_path: &PathBuf) -> Result<Patina> {
        let toml_str = match std::fs::read_to_string(toml_file_path) {
            Ok(toml_str) => toml_str,
            Err(e) => return Err(Error::FileRead(e)),
        };

        match toml::from_str(&toml_str) {
            Ok(toml) => Ok(toml),
            Err(e) => Err(Error::TomlParse(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patina_deserialize() {
        let patina = r#"
            name = "simple-patina"
            description = "This is a simple Patina example"

            [vars]
            name = "Patina"

            [[files]]
            template = "./templates/hello.txt"
            target = "./output/hello.txt"
        "#;

        let patina = toml::from_str::<Patina>(patina);
        assert!(patina.is_ok());
        let patina = patina.unwrap();

        assert_eq!(patina.name, "simple-patina");
        assert_eq!(patina.description, "This is a simple Patina example");

        let patina_files = patina.files;
        assert_eq!(patina_files.len(), 1);
        let patina_file = &patina_files[0];
        assert_eq!(patina_file.template, PathBuf::from("./templates/hello.txt"));
        assert_eq!(patina_file.target, PathBuf::from("./output/hello.txt"));
    }

    #[test]
    fn test_patina_deserialize_name_missing() {
        let patina = r#"
            description = "This is a simple Patina example"

            [vars]
            name = "Patina"

            [[files]]
            template = "./templates/hello.txt"
            target = "./output/hello.txt"
        "#;

        let patina = toml::from_str::<Patina>(patina);
        assert!(patina.is_err());
        assert_eq!(patina.unwrap_err().message(), "missing field `name`");
    }

    #[test]
    fn test_patina_deserialize_default_description() {
        let patina = r#"
            name = "simple-patina"

            [vars]
            name = "Patina"

            [[files]]
            template = "./templates/hello.txt"
            target = "./output/hello.txt"
        "#;

        let patina = toml::from_str::<Patina>(patina);
        assert!(patina.is_ok());
        let patina = patina.unwrap();

        assert!(patina.description.is_empty());
    }

    #[test]
    fn test_patina_deserialize_default_files() {
        let patina = r#"
            name = "simple-patina"
            description = "This is a simple Patina example"

            [vars]
            name = "Patina"
        "#;

        let patina = toml::from_str::<Patina>(patina);
        assert!(patina.is_ok());
        let patina = patina.unwrap();

        assert!(patina.files.is_empty());
    }

    #[test]
    fn test_patina_file_template_missing() {
        let patina = r#"
            name = "simple-patina"
            description = "This is a simple Patina example"

            [vars]
            name = "Patina"

            [[files]]
            target = "./output/hello.txt"
        "#;

        let patina = toml::from_str::<Patina>(patina);
        assert!(patina.is_err());
        assert_eq!(patina.unwrap_err().message(), "missing field `template`");
    }

    #[test]
    fn test_patina_file_target_missing() {
        let patina = r#"
            name = "simple-patina"
            description = "This is a simple Patina example"

            [vars]
            name = "Patina"

            [[files]]
            template = "./templates/hello.txt"
        "#;

        let patina = toml::from_str::<Patina>(patina);
        assert!(patina.is_err());
        assert_eq!(patina.unwrap_err().message(), "missing field `target`");
    }
}
