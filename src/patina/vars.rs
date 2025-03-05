//! [Patina] utilities for managing variables.

use std::path::PathBuf;
use log::debug;
use serde_json::Value;
use crate::patina::Patina;
use crate::utils::{Error, Result};

/// Overlay the contents of source onto target as json maps recursively
fn merge_values(a: &mut Value, b: Value) {
    if let Value::Object(a) = a {
        if let Value::Object(b) = b {
            for (k, v) in b {
                if v.is_null() {
                    a.remove(&k);
                } else {
                    merge_values(a.entry(k).or_insert(Value::Null), v);
                }
            }
            return;
        }
    }

    *a = b;
}

impl Patina {
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

        match self.vars {
            Some(ref mut self_vars) => merge_values(self_vars, vars),
            None => self.vars = Some(vars),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use serde_json::json;
    use crate::patina::Patina;
    use crate::patina::vars::merge_values;
    use crate::tests::test_utils::TmpTestDir;

    #[test]
    fn test_merge_values() {
        let mut a = json!({ "a": "a" });
        let b = json!({ "b": "b" });

        merge_values(&mut a, b);

        assert_eq!(
            a,
            json!({
                "a": "a",
                "b": "b",
            })
        );
    }

    #[test]
    fn test_merge_values_nested() {
        let mut a = json!({
            "a": "a",
            "me": {
                "name": "Patina User"
            }
        });
        let b = json!({
            "b": "b",
            "me": {
                "email": "patina@mail.com"
            }
        });

        merge_values(&mut a, b);

        assert_eq!(
            a,
            json!({
                "a": "a",
                "b": "b",
                "me": {
                    "name": "Patina User",
                    "email": "patina@mail.com"
                }
            })
        );
    }

    #[test]
    fn test_load_vars_files() {
        let tmp_dir = TmpTestDir::new();
        let path = tmp_dir.write_file(
            "patina-vars.toml",
            r#"
                name = "patina-vars"
                description = "This is a patina with variables"

                [vars]
                me.name = "Patina"

                [[files]]
                template = "hello-vars.txt.hbs"
                target = "./output/vars.txt"
            "#,
        );
        let vars_a_path = tmp_dir.write_file(
            "vars-a.toml",
            r#"
                me.email = "aaa@mail.com"
                a_var = "aaa"
                example_var = "aaa"
            "#,
        );
        let vars_b_path = tmp_dir.write_file(
            "vars-b.toml",
            r#"
                me.email = "bbb@mail.com"
                b_var = "bbb"
                example_var = "bbb"
            "#,
        );

        let patina = Patina::from_toml_file(&path);
        assert!(patina.is_ok());
        let mut patina = patina.unwrap();

        let load_vars = patina.load_vars_files(vec![vars_a_path, vars_b_path]);
        assert!(load_vars.is_ok());

        assert_eq!(
            patina.vars,
            Some(json!({
                "me": {
                    "name": "Patina",
                    "email": "bbb@mail.com"
                },
                "a_var": "aaa",
                "b_var": "bbb",
                "example_var": "bbb"
            }))
        );
    }

    #[test]
    fn test_load_vars_files_file_does_not_exist() {
        let tmp_dir = TmpTestDir::new();
        let path = tmp_dir.write_file(
            "patina-vars.toml",
            r#"
                name = "patina-vars"
                description = "This is a patina with variables"

                [vars]
                name = "Patina"

                [[files]]
                template = "hello-vars.txt.hbs"
                target = "./output/vars.txt"
            "#,
        );

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
        let tmp_dir = TmpTestDir::new();
        let path = tmp_dir.write_file(
            "patina-vars.toml",
            r#"
                name = "patina-vars"
                description = "This is a patina with variables"

                [vars]
                name = "Patina"

                [[files]]
                template = "hello-vars.txt.hbs"
                target = "./output/vars.txt"
            "#,
        );
        let invalid_vars_path = tmp_dir.write_file(
            "invalid_vars.toml",
            r#"
                [[]]1]1[1[1]]1
            "#,
        );

        let patina = Patina::from_toml_file(&path);
        assert!(patina.is_ok());
        let mut patina = patina.unwrap();

        let load_vars = patina.load_vars_files(vec![invalid_vars_path]);
        assert!(load_vars.is_err());
        let err = load_vars.unwrap_err();
        assert!(err.is_toml_parse())
    }
}
