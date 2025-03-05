//! Miscellaneous utilities used throughout the crate

use std::{
    fs,
    path::{Path, PathBuf},
};

use enum_as_inner::EnumAsInner;

/// An enum representing all possible errors that can occur in this crate
#[allow(dead_code)]
#[derive(Debug, EnumAsInner)]
pub enum Error {
    /// A general error with a message
    Message(String),

    /// An error that occurs when a file cannot be read
    FileRead(PathBuf, std::io::Error),

    /// An error that occurs when a file cannot be written
    FileWrite(PathBuf, std::io::Error),

    /// Failed to get input from the user
    GetUserInput(std::io::Error),

    /// An error that occurs when parsing Toml data
    TomlParse(toml::de::Error),

    /// An error that occurs when rendering a handlebars template
    RenderTemplate(handlebars::RenderError),

    /// A vars object is invalid
    InvalidVars(),

    /// Failed to trash a file
    MoveFileToTrash(trash::Error),
}

/// A Result type that uses the [`Error`] enum
pub type Result<T> = std::result::Result<T, Error>;

/// Given a path, normalize it to an absolute path with cwd (`.`), home (`~`), and environment variables resolved.
pub fn normalize_path<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    let path = path.as_ref();

    // resolve home dir and environment variables
    let path = path.to_str()?;
    let path = match shellexpand::full(path) {
        Ok(path) => path.into_owned(),
        Err(_) => return None,
    };

    // clean it by resolving `.` and multiple `/`s
    let path = path_clean::clean(path);

    // get the canonical path
    match fs::canonicalize(&path) {
        Ok(path) => Some(path),
        Err(_) => Some(path),
    }
}

#[cfg(test)]
pub mod tests {
    use std::path::PathBuf;

    use super::normalize_path;

    pub fn get_home_dir() -> String {
        let home_dir = dirs::home_dir().unwrap();
        let home_dir = home_dir.to_str().unwrap();
        String::from(home_dir)
    }

    #[test]
    fn test_normalize_path() {
        let path = normalize_path(PathBuf::from("path/to/file.txt"));
        assert_eq!(PathBuf::from("path/to/file.txt"), path.unwrap());
    }

    #[test]
    fn test_normalize_path_with_dotdot() {
        let path = normalize_path(PathBuf::from("path/to/../file.txt"));
        assert_eq!(PathBuf::from("path/file.txt"), path.unwrap());
    }

    #[test]
    fn test_normalize_path_with_multiple_slashes() {
        let path = normalize_path(PathBuf::from("path/to///file.txt"));
        assert_eq!(PathBuf::from("path/to/file.txt"), path.unwrap());
    }

    #[test]
    fn test_normalize_path_home_dir() {
        let path = normalize_path(PathBuf::from("~/path/to/file.txt"));
        assert_eq!(
            PathBuf::from(format!("{}/path/to/file.txt", get_home_dir())),
            path.unwrap()
        );
    }

    #[test]
    fn test_normalize_path_with_hidden_dir() {
        let path = normalize_path(PathBuf::from("~/.dotpatina/file.txt"));
        assert_eq!(
            PathBuf::from(format!("{}/.dotpatina/file.txt", get_home_dir())),
            path.unwrap()
        );
    }
}
