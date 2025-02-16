use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::utils::{Error, Result};

/// A Patina describes a set of variables and templates that can be rendered to files.
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Patina {
    /// The name of the Patina
    pub name: String,

    /// A short description of the Patina
    #[serde(default)]
    pub description: String,

    /// A map of variables that can be used in the templates
    #[serde(default)]
    pub vars: Option<serde_json::Value>,

    /// A list of files referencing templates and their target output paths
    #[serde(default)]
    pub files: Vec<PatinaFile>,

    /// The path to this patina
    #[serde(skip)]
    pub base_path: Option<PathBuf>,
}

/// A PatinaFile describes a template file and its target output path.
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PatinaFile {
    /// The path to the template file
    pub template: PathBuf,

    /// The path to the garget output file
    pub target: PathBuf,
}

impl Patina {
    /// Load a Patina from a TOML file
    pub fn from_toml_file(toml_file_path: &PathBuf) -> Result<Patina> {
        let toml_str = match std::fs::read_to_string(toml_file_path) {
            Ok(toml_str) => toml_str,
            Err(e) => return Err(Error::FileRead(toml_file_path.clone(), e)),
        };

        let mut patina: Patina = match toml::from_str(&toml_str) {
            Ok(patina) => patina,
            Err(e) => return Err(Error::TomlParse(e)),
        };

        patina.base_path = Some(toml_file_path.parent().unwrap().to_path_buf());

        Ok(patina)
    }

    /// Get a path within the context of this Patina
    pub fn get_patina_path(&self, path: &Path) -> PathBuf {
        if path.is_absolute() {
            return path.to_path_buf();
        }

        let mut result = self
            .base_path
            .as_ref()
            .unwrap_or(&PathBuf::from("."))
            .clone();
        result.push(path);

        match fs::canonicalize(&result) {
            Ok(r) => r,
            Err(_) => result,
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

        // assert vars
        assert!(patina.vars.is_some());
        let vars = patina.vars.unwrap();
        assert!(vars.is_object());
        assert_eq!(vars.as_object().unwrap().get("name").unwrap(), "Patina");

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

        let (path, err) = match err.as_file_read() {
            Some(err) => err,
            None => panic!("expected FileRead error"),
        };
        assert_eq!(path, &PathBuf::from("this/file/does/not/exist.toml"));
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
}
