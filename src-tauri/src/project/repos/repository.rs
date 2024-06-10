use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use walkdir::WalkDir;

use crate::{
    editor::editor::*,
    nodes::{ArenaTree, Node},
    quicks::{self, quick::get_path_name},
};

/// Репозиторий представляет собой папку с модом
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repository {
    /// Path to modinfo.json folder
    pub folder_path: String,
}

impl Repository {
    pub fn new(folder_path: String) -> Self {
        Self { folder_path }
    }

    pub fn get_modinfo_file_path(&self) -> String {
        self.folder_path.clone() + "/modinfo.json"
    }
}

const REPOSITORIES_FOLDER_NAME: &str = "repositories";

pub fn find_repositories(folder: &PathBuf) -> Vec<Repository> {
    let mut repositories: Vec<Repository> = Vec::new();

    for entry in WalkDir::new(folder).contents_first(true) {
        let entry = entry.unwrap();
        if entry.file_type().is_dir() && is_repository_folder(entry.path()) {
            let folder_path = entry.path().to_str().unwrap().to_string();
            let id = -1;
            repositories.push(Repository::new(folder_path));
        }
    }

    repositories
}

pub fn is_repository_folder(folder_path: &Path) -> bool {
    folder_path.join("modinfo.json").exists()
}

pub fn add_repository(repository_folder: &Path, editor: &EditorEnvironment) -> Result<(), String> {
    if (!repository_folder.exists()) {
        return Err(String::from("Repository folder does not exist"));
    }

    if (!is_repository_folder(repository_folder)) {
        return Err(String::from(
            "Folder is not a repository folder. Repository folder must contain modinfo.json file",
        ));
    }

    let repository = Repository::new(repository_folder.to_str().unwrap().to_string());
    let mut repositories = get_repositories(editor);
    repositories.push(repository);
    let file_content = serde_json::to_string(&repositories).unwrap();
    std::fs::write(
        editor
            .get_or_create_folder(REPOSITORIES_FOLDER_NAME)
            .unwrap()
            .join("repositories.json"),
        file_content,
    )
    .unwrap();

    Ok(())
}

pub fn remove_repository(
    repository_folder: &Path,
    editor: &EditorEnvironment,
) -> Result<(), String> {
    let mut repositories = get_repositories(editor);
    repositories.retain(|repository| repository.folder_path != repository_folder.to_str().unwrap());
    let file_content = serde_json::to_string(&repositories).unwrap();
    std::fs::write(
        editor
            .get_or_create_folder(REPOSITORIES_FOLDER_NAME)
            .unwrap()
            .join("repositories.json"),
        file_content,
    )
    .unwrap();
    Ok(())
}

pub fn get_repositories(editor: &EditorEnvironment) -> Vec<Repository> {
    let mut repositories: Vec<Repository> = Vec::new();
    let repositories_folder = editor.get_or_create_folder(REPOSITORIES_FOLDER_NAME);

    if repositories_folder.is_ok() {
        let repositories_config_file = repositories_folder
            .as_ref()
            .unwrap()
            .join("repositories.json");
        if repositories_config_file.exists() {
            // read json with repository objects
            let file_content = std::fs::read_to_string(repositories_config_file).unwrap();
            let repositories_config: Vec<Repository> = serde_json::from_str(&file_content).unwrap();
            repositories = repositories_config;
        } else {
            // create empty json file
            let file_content = serde_json::to_string(&repositories).unwrap();
            std::fs::write(repositories_config_file, file_content).unwrap();
        }
    }

    repositories
}

// 0000000000000000000000000000000000000000000000000000000000

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepositoryInfo {
    /// Path to modinfo.json folder
    pub folder_path: String,
    pub mod_identifier: String,
}

impl RepositoryInfo {
    pub fn new(folder_path: String, mod_identifier: String) -> Self {
        Self {
            folder_path,
            mod_identifier,
        }
    }
}

impl From<Repository> for RepositoryInfo {
    fn from(repository: Repository) -> RepositoryInfo {
        get_repository_info(&repository)
    }
}

fn get_repository_info(repository: &Repository) -> RepositoryInfo {
    // read modinfo.json and get "identifier" field
    let modinfo_file_path = repository.get_modinfo_file_path();
    let file_content = std::fs::read_to_string(modinfo_file_path).unwrap();
    let modinfo: Value = serde_json::from_str(&file_content).unwrap();
    RepositoryInfo::new(
        repository.folder_path.clone(),
        modinfo["identifier"].to_string().replace('"', ""),
    )
}
