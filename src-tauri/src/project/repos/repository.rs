use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::{
    editor::EditorEnvironment, nodes::{ArenaTree, Node}, quicks::{self, quick::get_path_name}
};

/// Репозиторий представляет собой папку с модом
#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    /// Path to modinfo.json folder
    pub folder_path: String,
}

impl Repository {
    pub fn new( folder_path: String) -> Self {
        Self {  folder_path }
    }

    pub fn get_modinfo_file_path(&self) -> String {
        self.folder_path.clone() + "/modinfo.json"
    }
}

const REPOSITORIES_FOLDER_NAME : &str = "repositories";

pub struct RepositoryComputed {
    pub repository: Repository,
    pub tree: Option<ArenaTree>,
}

impl RepositoryComputed {
    pub fn new(repository: Repository, tree: Option<ArenaTree>) -> Self {
        Self { repository, tree }
    }

    pub fn ini(&mut self) -> Result<(), String> {

        let mut tree = self.tree.as_mut().unwrap();
        Self::build_arena_tree(
            &PathBuf::from(&self.repository.folder_path),
            &None,
            &mut tree,
        );
        Ok(())
    }

    fn build_arena_tree(
        folder_path: &PathBuf,
        previous_node: &Option<i32>,
        arena_tree: &mut ArenaTree,
    ) {
        let folder_node: Node = Node::new(get_path_name(folder_path), String::from(""));
        let folder_node_id: i32;

        if previous_node.is_none() {
            if arena_tree.get_root_node().is_none() {
                folder_node_id = arena_tree.add_root_node(folder_node);
            } else {
                // корневые ноды можно определять по наличию родителя - можно сделать когда-нибудь.
                // folder_node_id = arena_tree.add_node(folder_node);
                panic!("Root node already exists in arena tree");
            }
        } else {
            folder_node_id = arena_tree.add_node_to_parent_id(previous_node.unwrap(), folder_node);
        }

        // iterate directory entries and build arena tree
        for entry in WalkDir::new(folder_path.to_str().unwrap())
            .min_depth(1)
            .max_depth(1)
        {
            let entry = entry.unwrap();
            let entry_name = entry.file_name().to_str().unwrap();
            let entry_path = folder_path.join(entry_name);

            println!("{} + {}", folder_path.to_str().unwrap(), entry_name);
            println!(" ");

            if entry.file_type().is_dir() {
                Self::build_arena_tree(&entry_path, &Some(folder_node_id), arena_tree);
            } else if entry.file_type().is_file() {
                let file_node = Node::new(entry_name.to_string(), String::from(""));
                arena_tree.add_node_to_parent_id(folder_node_id, file_node);
            } else {
                // do nothing
            }
        }
    }
}

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

pub fn add_repository(repository_folder: &Path, editor:&EditorEnvironment) -> Result<(), String> {
    if(!repository_folder.exists()){
        return Err(String::from("Repository folder does not exist"));
    }

    if(!is_repository_folder(repository_folder)){
        return Err(String::from("Folder is not a repository folder. Repository folder must contain modinfo.json file"));
    }

    let repository = Repository::new(repository_folder.to_str().unwrap().to_string());
    let mut repositories = get_repositories(editor);
    repositories.push(repository);
    let file_content = serde_json::to_string(&repositories).unwrap();
    std::fs::write(editor.get_or_create_folder(REPOSITORIES_FOLDER_NAME).unwrap().join("repositories.json"), file_content).unwrap();

    Ok(())
}

pub fn remove_repository(repository_folder: &Path, editor:&EditorEnvironment) -> Result<(), String> 
{
    let mut repositories = get_repositories(editor);
    repositories.retain(|repository| repository.folder_path != repository_folder.to_str().unwrap());
    let file_content = serde_json::to_string(&repositories).unwrap();
    std::fs::write(editor.get_or_create_folder(REPOSITORIES_FOLDER_NAME).unwrap().join("repositories.json"), file_content).unwrap();
    Ok(())
}

pub fn get_repositories(editor:&EditorEnvironment) -> Vec<Repository>{

    let mut repositories: Vec<Repository> = Vec::new();
    let repositories_folder = editor.get_or_create_folder(REPOSITORIES_FOLDER_NAME);

    if repositories_folder.is_ok() {
        let repositories_config_file = repositories_folder.as_ref().unwrap().join("repositories.json");
        if repositories_config_file.exists() {
            // read json with repository objects
            let file_content = std::fs::read_to_string(repositories_config_file).unwrap();
            let repositories_config: Vec<Repository> = serde_json::from_str(&file_content).unwrap();
            repositories = repositories_config;
        }else{
            // create empty json file
            let file_content = serde_json::to_string(&repositories).unwrap();
            std::fs::write(repositories_config_file, file_content).unwrap();
        }
    }

    repositories
}