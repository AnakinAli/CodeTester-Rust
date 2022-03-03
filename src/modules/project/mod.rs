use walkdir::WalkDir;
use std::fs::metadata;

/// Samples input extension
pub const PYTHON_EXTENSION_FILE: &str = ".py";

#[derive(Debug)]
pub struct Project {
    pub path: String,
}

impl Project {
    ///
    /// Locates the student's project
    /// Returns Path
    ///
    pub fn locate_code_path(student_project_general_path: String) -> Project {
        let mut path: String = "".to_string();

        for entry in WalkDir::new(student_project_general_path).into_iter().filter_map(|e| e.ok()) {

            let entry_path: String = (entry.path().display()).to_string();
            let md = metadata(&entry_path).unwrap();

            if md.is_file() && entry_path.ends_with(&PYTHON_EXTENSION_FILE) {
                // Check if ends with .py
                    path = entry_path;
            }
        }

        return Project {
            path
        };
    }
}