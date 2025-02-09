use std::path::PathBuf;
use yaml_rust::Yaml;

use super::{PatinaError, Result};

#[derive(Debug, Default, PartialEq)]
pub struct PatinaFile {
    pub template: PathBuf,
    pub target: PathBuf,
}

impl PatinaFile {
    pub fn from_yaml(yaml: &Yaml) -> Result<PatinaFile> {
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
