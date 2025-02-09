use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::utils::{Error, Result};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Patina {
    pub name: String,

    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub vars: HashMap<String, String>,

    #[serde(default)]
    pub files: Vec<PatinaFile>,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PatinaFile {
    pub template: PathBuf,
    pub target: PathBuf,
}

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

impl PatinaFile {
    pub fn load_template_file_as_string(&self) -> Result<String> {
        match std::fs::read_to_string(self.template.clone()) {
            Ok(template_str) => Ok(template_str),
            Err(e) => Err(Error::FileRead(e)),
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

    #[test]
    fn test_patina_from_toml_file() {
        let path = PathBuf::from("tests/fixtures/patina.toml");

        let patina = Patina::from_toml_file(&path);
        assert!(patina.is_ok());
        let patina = patina.unwrap();
        assert_eq!(patina.name, "simple-patina");
    }

    #[test]
    fn test_patina_from_toml_file_missing_file() {
        let path = PathBuf::from("this/file/does/not/exist.toml");
        let patina = Patina::from_toml_file(&path);

        let err = match patina {
            Ok(_) => panic!("expected error"),
            Err(e) => e,
        };

        let err = match err.as_file_read() {
            Some(err) => err,
            None => panic!("expected FileRead error"),
        };
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    }

    #[test]
    fn test_patina_from_toml_file_invalid_format() {
        let path = PathBuf::from("tests/fixtures/invalid_patina.toml");

        let patina = Patina::from_toml_file(&path);
        assert!(patina.is_err());
        let err = patina.unwrap_err();
        assert!(matches!(err, Error::TomlParse(_)));
    }

    #[test]
    fn test_patina_file_load_template_file_as_string() {
        let patina_file = PatinaFile {
            template: PathBuf::from("tests/fixtures/template.txt.hbs"),
            target: PathBuf::from("output/template.txt"),
        };

        let template_str = patina_file.load_template_file_as_string();
        assert!(template_str.is_ok());
        let template_str = template_str.unwrap();

        let expected = r#"Hello, {{ name }}!

This is an example Patina template file.

Templates use the Handebars templating language. For more information, see <https://handlebarsjs.com/guide/>.
"#;
        assert_eq!(template_str, expected);
    }
}
