use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

// Actually folder with modinfo.json
#[derive(Serialize,Deserialize,Debug)]
pub struct Repository{
    /// Used by external modules
    pub id: i32,

    /// Path to modinfo.json folder
    pub folder_path: String
}

impl Repository {
    pub fn new(id: i32 ,folder_path: String) -> Self {
        Self {
            id,
            folder_path
        }
    }

    pub fn get_modinfo_file_path(&self) -> String {
        self.folder_path.clone() + "/modinfo.json"
    }
}


pub fn find_repositories(folder:&PathBuf) -> Vec<Repository> {
    let mut repositories: Vec<Repository> = Vec::new();

    for entry in WalkDir::new(folder).contents_first(true) {
        let entry = entry.unwrap();
        if entry.file_type().is_dir() && is_repository_folder(entry.path()) {
            let folder_path = entry.path().to_str().unwrap().to_string();
            let id = -1;
            repositories.push(Repository::new(id, folder_path));
        }
    }

    repositories
}

pub fn is_repository_folder(folder_path: &Path) -> bool {
    folder_path.join("modinfo.json").exists()
}