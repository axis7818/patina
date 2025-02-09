use std::{fs, path::PathBuf};
use yaml_rust::{Yaml, YamlLoader};

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

#[derive(Debug, Default, PartialEq)]
pub struct Patina {
    name: String,
    description: String,
    files: Vec<PatinaFile>,
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

#[derive(Debug, Default, PartialEq)]
pub struct PatinaFile {
    template: PathBuf,
    target: PathBuf,
}

impl PatinaFile {
    fn from_yaml(yaml: &Yaml) -> Result<PatinaFile> {
        let template = yaml["template"]
            .as_str()
            .ok_or(PatinaError::FileTemplateMissing)?
            .to_string();

        let target = yaml["target"]
            .as_str()
            .ok_or(PatinaError::FileTargetMissing)?
            .to_string();

        Ok(PatinaFile {
            template: PathBuf::from(template),
            target: PathBuf::from(target),
        })
    }
}

#[cfg(test)]
mod patina_file_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_patina_file_from_yaml() {
        let yaml = yaml_rust::YamlLoader::load_from_str(
            r#"
            template: "template.txt"
            target: "target.txt"
            "#,
        )
        .unwrap();

        let patina_file = PatinaFile::from_yaml(&yaml[0]);
        assert!(patina_file.is_ok());
        let patina_file = patina_file.unwrap();

        assert_eq!(
            patina_file,
            PatinaFile {
                template: PathBuf::from("template.txt"),
                target: PathBuf::from("target.txt"),
            }
        );
    }

    #[test]
    fn test_patina_file_from_yaml_missing_template() {
        let yaml = yaml_rust::YamlLoader::load_from_str(
            r#"
            target: "target.txt"
            "#,
        )
        .unwrap();

        let patina_file = PatinaFile::from_yaml(&yaml[0]);
        assert!(patina_file.is_err());
        let patina_file = patina_file.unwrap_err();
        assert!(matches!(patina_file, PatinaError::FileTemplateMissing));
    }

    #[test]
    fn test_patina_file_from_yaml_missing_target() {
        let yaml = yaml_rust::YamlLoader::load_from_str(
            r#"
            template: "template.txt"
            "#,
        )
        .unwrap();

        let patina_file = PatinaFile::from_yaml(&yaml[0]);
        assert!(patina_file.is_err());
        let patina_file = patina_file.unwrap_err();
        assert!(matches!(patina_file, PatinaError::FileTargetMissing));
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
