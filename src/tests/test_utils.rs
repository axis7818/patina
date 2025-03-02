use std::{fs, path::PathBuf};

use uuid::Uuid;

/// TmpTestDir is a helper struct for creating a temporary directory for testing.
pub struct TmpTestDir {
    pub path: PathBuf,
}

impl TmpTestDir {
    /// Create a new temporary directory with the given name.
    pub fn new() -> TmpTestDir {
        let path = std::env::temp_dir()
            .join("dotpatina-tests")
            .join(Uuid::new_v4().to_string());
        fs::create_dir_all(&path).unwrap();
        TmpTestDir { path }
    }

    /// Write a file to the temporary directory and return the full PathBuf
    pub fn write_file(&self, file_name: &str, contents: &str) -> PathBuf {
        let full_path = self.path.join(file_name);
        fs::write(&full_path, contents).unwrap();
        full_path
    }
}

impl Drop for TmpTestDir {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).unwrap();
    }
}
