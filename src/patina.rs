use std::path::{Path, PathBuf};

use log::debug;
use patina_file::PatinaFile;
use serde::{Deserialize, Serialize};

use crate::utils::{normalize_path, Error, Result};

pub mod patina_file;

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

    /// Load vars files from disk and overlay them onto the current vars in order
    pub fn load_vars_files(&mut self, vars_files: Vec<PathBuf>) -> Result<()> {
        vars_files
            .iter()
            .try_for_each(|f| self.overlay_vars_from_file(f))
    }

    /// Overlay the contents of vars_file onto the current vars
    fn overlay_vars_from_file(&mut self, vars_file: &PathBuf) -> Result<()> {
        let vars_str = match std::fs::read_to_string(vars_file) {
            Ok(vars_str) => vars_str,
            Err(e) => return Err(Error::FileRead(vars_file.clone(), e)),
        };

        let vars: serde_json::Value = match toml::from_str(&vars_str) {
            Ok(vars) => vars,
            Err(e) => return Err(Error::TomlParse(e)),
        };

        debug!("overlaying vars from file: {:?}, \n{:#?}", vars_file, vars);

        self.overlay_vars(Some(vars))?;
        Ok(())
    }

    /// Overlay the contents of vars onto the current vars
    fn overlay_vars(&mut self, vars: Option<serde_json::Value>) -> Result<()> {
        let vars = match vars {
            Some(vars) => vars,
            None => return Ok(()),
        };

        // overlay the contents of vars onto self.vars
        self.vars = match &self.vars {
            Some(current_vars) => {
                let mut current_vars_clone = current_vars.clone();
                let new_vars = match current_vars_clone.as_object_mut() {
                    Some(new_vars) => new_vars,
                    None => return Err(Error::InvalidVars()),
                };

                let vars = match vars.as_object() {
                    Some(vars) => vars,
                    None => return Err(Error::InvalidVars()),
                };

                new_vars.extend(vars.clone());
                Some(serde_json::Value::Object(new_vars.clone()))
            }
            None => Some(vars),
        };

        Ok(())
    }

    /// Get a path within the context of this Patina
    pub fn get_patina_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        let path = path.as_ref();
        let path = normalize_path(path).unwrap_or(path.to_path_buf());

        if path.is_absolute() {
            return path.to_path_buf();
        }

        let mut result = self
            .base_path
            .as_ref()
            .unwrap_or(&PathBuf::from("."))
            .clone();
        result.push(path);

        match normalize_path(&result) {
            Some(result) => result,
            None => result,
        }
    }

    /// Get an iterator for all PatinaFiles that are tagged with any of the provided tags
    pub fn files_for_tags(&self, tags: Option<Vec<String>>) -> impl Iterator<Item = &PatinaFile> {
        self.files.iter().filter(move |f| match &tags {
            Some(tags) => f.tags.iter().any(|t| tags.contains(t)),
            None => true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tests::get_home_dir;

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

    #[test]
    fn test_patina_get_patina_path_in_home_dir() {
        let patina = Patina {
            name: "test".to_string(),
            description: "".to_string(),
            base_path: Some(PathBuf::from("tests")),
            vars: None,
            files: vec![],
        };

        let result = patina.get_patina_path(PathBuf::from("~/.dotpatina/home-dir-test.txt"));
        assert_eq!(
            PathBuf::from(format!("{}/.dotpatina/home-dir-test.txt", get_home_dir())),
            result
        );
    }

    #[test]
    fn test_patina_get_patina_path_absolute_dir() {
        let patina = Patina {
            name: "test".to_string(),
            description: "".to_string(),
            base_path: Some(PathBuf::from("tests")),
            vars: None,
            files: vec![],
        };

        let result = patina.get_patina_path(PathBuf::from("/tmp/dotpatina/absolute-test.txt"));
        assert_eq!(PathBuf::from("/tmp/dotpatina/absolute-test.txt"), result);
    }

    #[test]
    fn test_patina_get_patina_path_combine_parts() {
        let patina = Patina {
            name: "test".to_string(),
            description: "".to_string(),
            base_path: Some(PathBuf::from("tests")),
            vars: None,
            files: vec![],
        };

        let result = patina.get_patina_path(PathBuf::from("fixtures/test.txt"));
        assert_eq!(PathBuf::from("tests/fixtures/test.txt"), result);
    }

    #[test]
    fn test_patina_files_for_tags() {
        let patina = Patina {
            name: "test".to_string(),
            description: "".to_string(),
            base_path: None,
            vars: None,
            files: vec![
                PatinaFile {
                    template: PathBuf::from("a.hbs"),
                    target: PathBuf::from("a.txt"),
                    tags: vec!["a".to_string()],
                },
                PatinaFile {
                    template: PathBuf::from("b.hbs"),
                    target: PathBuf::from("b.txt"),
                    tags: vec!["b".to_string()],
                },
                PatinaFile {
                    template: PathBuf::from("ab.hbs"),
                    target: PathBuf::from("ab.txt"),
                    tags: vec!["a".to_string(), "b".to_string()],
                },
            ],
        };

        let patina_file_a = &patina.files[0];
        let patina_file_b = &patina.files[1];
        let patina_file_ab = &patina.files[2];

        let tags = None;
        let filter_none = patina.files_for_tags(tags).collect::<Vec<&PatinaFile>>();
        assert_eq!(filter_none.len(), 3);

        let tags = Some(vec!["a".to_string()]);
        let filter_a: Vec<&PatinaFile> = patina.files_for_tags(tags).collect();
        assert_eq!(filter_a.len(), 2);
        assert_eq!(filter_a[0], patina_file_a);
        assert_eq!(filter_a[1], patina_file_ab);

        let tags = Some(vec!["b".to_string()]);
        let filter_b: Vec<&PatinaFile> = patina.files_for_tags(tags).collect();
        assert_eq!(filter_b.len(), 2);
        assert_eq!(filter_b[0], patina_file_b);
        assert_eq!(filter_b[1], patina_file_ab);

        let tags = Some(vec!["a".to_string(), "b".to_string()]);
        let filter_ab: Vec<&PatinaFile> = patina.files_for_tags(tags).collect();
        assert_eq!(filter_ab.len(), 3);
        assert_eq!(filter_ab[0], patina_file_a);
        assert_eq!(filter_ab[1], patina_file_b);
        assert_eq!(filter_ab[2], patina_file_ab);
    }

    #[test]
    fn test_load_vars_files() {
        let path = PathBuf::from("tests/fixtures/patina-vars.toml");

        let patina = Patina::from_toml_file(&path);
        assert!(patina.is_ok());
        let mut patina = patina.unwrap();

        let load_vars = patina.load_vars_files(vec![
            PathBuf::from("tests/fixtures/vars-a.toml"),
            PathBuf::from("tests/fixtures/vars-b.toml"),
        ]);
        assert!(load_vars.is_ok());

        assert_eq!(
            patina.vars,
            Some(
                serde_json::json!({"name": "Patina", "a_var": "aaa", "b_var": "bbb", "example_var": "bbb"})
            )
        );
    }

    #[test]
    fn test_load_vars_files_file_does_not_exist() {
        let path = PathBuf::from("tests/fixtures/patina-vars.toml");

        let patina = Patina::from_toml_file(&path);
        assert!(patina.is_ok());
        let mut patina = patina.unwrap();

        let load_vars =
            patina.load_vars_files(vec![PathBuf::from("this/path/does/not/exist.toml")]);
        assert!(load_vars.is_err());
        let err = load_vars.unwrap_err();
        assert!(err.is_file_read())
    }

    #[test]
    fn test_load_vars_files_invalid_file_contents() {
        let path = PathBuf::from("tests/fixtures/patina-vars.toml");

        let patina = Patina::from_toml_file(&path);
        assert!(patina.is_ok());
        let mut patina = patina.unwrap();

        let load_vars =
            patina.load_vars_files(vec![PathBuf::from("tests/fixtures/invalid_vars.toml")]);
        assert!(load_vars.is_err());
        let err = load_vars.unwrap_err();
        assert!(err.is_toml_parse())
    }
}
