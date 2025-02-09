use std::{fs, path::PathBuf};
use yaml_rust::{Yaml, YamlLoader};

use super::patina_file::PatinaFile;
use super::{PatinaError, Result};

#[derive(Debug, Default, PartialEq)]
pub struct Patina {
    pub name: String,
    pub description: String,
    pub files: Vec<PatinaFile>,
}

impl Patina {
    pub fn from_yaml_file(yaml_file_path: &PathBuf) -> Result<Vec<Patina>> {
        let yaml_str = match fs::read_to_string(yaml_file_path) {
            Ok(yaml_str) => yaml_str,
            Err(e) => return Err(PatinaError::FileReadError(e)),
        };

        let yaml = match YamlLoader::load_from_str(&yaml_str) {
            Ok(yaml) => yaml,
            Err(e) => return Err(PatinaError::YamlParseError(e)),
        };

        yaml.iter().map(Patina::from_yaml).collect()
    }

    fn from_yaml(yaml: &Yaml) -> Result<Patina> {
        let name = yaml["name"]
            .as_str()
            .ok_or(PatinaError::NameMissing)?
            .to_string();

        let description = yaml["description"]
            .as_str()
            .ok_or(PatinaError::DescriptionMissing)?
            .to_string();

        let files = yaml["files"]
            .as_vec()
            .unwrap_or(&vec![])
            .iter()
            .map(PatinaFile::from_yaml)
            .collect::<Result<Vec<PatinaFile>>>()?;

        Ok(Patina {
            name,
            description,
            files,
        })
    }
}

#[cfg(test)]
mod patina_tests {
    use super::*;

    #[test]
    fn test_patina_from_yaml() {
        let yaml = YamlLoader::load_from_str(
            r#"
            name: test-patina
            description: a patina used for testing
            vars:
              key: value
            files:
            - template: template.txt
              target: target.txt
            "#,
        )
        .unwrap();

        let patina = Patina::from_yaml(&yaml[0]);
        assert!(patina.is_ok());
        let patina = patina.unwrap();

        assert_eq!(
            patina,
            Patina {
                name: String::from("test-patina"),
                description: String::from("a patina used for testing"),
                files: vec![PatinaFile {
                    template: PathBuf::from("template.txt"),
                    target: PathBuf::from("target.txt"),
                }],
            }
        );
    }

    #[test]
    fn test_patina_from_yaml_missing_name() {
        let yaml = YamlLoader::load_from_str(
            r#"
            # name: test-patina
            description: a patina used for testing
            vars:
              key: value
            files:
            - template: template.txt
              target: target.txt
            "#,
        )
        .unwrap();

        let patina = Patina::from_yaml(&yaml[0]);
        assert!(matches!(patina, Err(PatinaError::NameMissing)));
    }

    #[test]
    fn test_patina_from_yaml_missing_description() {
        let yaml = YamlLoader::load_from_str(
            r#"
            name: test-patina
            # description: a patina used for testing
            vars:
              key: value
            files:
            - template: template.txt
              target: target.txt
            "#,
        )
        .unwrap();

        let patina = Patina::from_yaml(&yaml[0]);
        assert!(matches!(patina, Err(PatinaError::DescriptionMissing)));
    }

    #[test]
    fn test_patina_from_yaml_invalid_patine_file() {
        let yaml = YamlLoader::load_from_str(
            r#"
            name: test-patina
            description: a patina used for testing
            vars:
              key: value
            files:
            - template: template.txt
              # target: target.txt
            "#,
        )
        .unwrap();

        let patina = Patina::from_yaml(&yaml[0]);
        assert!(matches!(patina, Err(PatinaError::FileTargetMissing)));
    }
}
