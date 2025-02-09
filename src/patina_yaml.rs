use std::path::PathBuf;

use crate::patina::{Patina, PatinaFile, ValidationError};
use yaml_rust::Yaml;

pub trait FromYaml: Sized {
    fn from_yaml(yaml: &Yaml) -> Result<Self>;
}

type Result<T> = std::result::Result<T, ValidationError>;

impl FromYaml for PatinaFile {
    fn from_yaml(yaml: &Yaml) -> Result<PatinaFile> {
        let template = yaml["template"]
            .as_str()
            .ok_or(ValidationError::FileTemplateMissing)?
            .to_string();

        let target = yaml["target"]
            .as_str()
            .ok_or(ValidationError::FileTargetMissing)?
            .to_string();

        Ok(PatinaFile {
            template: PathBuf::from(template),
            target: PathBuf::from(target),
        })
    }
}

impl FromYaml for Patina {
    fn from_yaml(yaml: &Yaml) -> Result<Patina> {
        let name = yaml["name"]
            .as_str()
            .ok_or(ValidationError::NameMissing)?
            .to_string();

        let description = yaml["description"]
            .as_str()
            .ok_or(ValidationError::DescriptionMissing)?
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
mod tests {
    use yaml_rust::YamlLoader;

    use crate::patina::ValidationError;

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
        assert!(matches!(patina_file, ValidationError::FileTemplateMissing));
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
        assert!(matches!(patina_file, ValidationError::FileTargetMissing));
    }

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
        assert!(matches!(patina, Err(ValidationError::NameMissing)));
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
        assert!(matches!(patina, Err(ValidationError::DescriptionMissing)));
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
        assert!(matches!(patina, Err(ValidationError::FileTargetMissing)));
    }
}
