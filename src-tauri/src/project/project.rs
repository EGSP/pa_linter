use std::{fs, path::Path};

use walkdir::WalkDir;

use crate::nodes::*;

pub struct Project {
    pub name: String,

    pub modinfo_path: String,
    pub arena_tree: ArenaTree,
}

impl Project {
    pub fn new(name: String, modinfo_path: String, arena_tree: ArenaTree) -> Self {
        Self {
            name,
            modinfo_path,
            arena_tree,
        }
    }
//
    pub fn is_project_folder(folder_path: &str) -> Result<bool, String> {
        let folder_path = Path::new(folder_path);
        if folder_path.is_file() {
            return Err(format!("{} is not a folder", folder_path.to_str().unwrap()));
        }

        Ok(folder_path.join("modinfo.json").exists())
    }

    pub fn try_initilize_project(project_path: &str) -> Result<Project, String> {
        let project_path = Path::new(project_path);

        // check folder is a project folder
        let is_project_folder = Self::is_project_folder(project_path.to_str().unwrap());
        if is_project_folder.is_err() {
            return Err(is_project_folder.unwrap_err());
        } else if !is_project_folder.unwrap() {
            return Err(format!(
                "{} is not a project folder",
                project_path.to_str().unwrap()
            ));
        }

        let modinfo_path = project_path.join("modinfo.json");

        let arena_tree = Self::scan_project_folder(project_path).unwrap();
        // TODO: заменить на название из modinfo.json
        let project_name = Self::get_path_name(project_path);

        Ok(Project::new(
            project_name,
            modinfo_path.to_str().unwrap().to_string(),
            arena_tree,
        ))
    }

    fn scan_project_folder(folder_path: &Path) -> Result<ArenaTree, String> {
        if !folder_path.join("modinfo.json").exists() {
            return Err(
                "Folder is not a project folder. Project folder must contain modinfo.json file"
                    .to_string(),
            );
        }
        let mut arena_tree = ArenaTree::new();

        // iterate directory
        Self::build_arena_tree(folder_path, &None, &mut arena_tree);
        Ok(arena_tree)
    }

    fn build_arena_tree(
        folder_path: &Path,
        previous_node: &Option<i32>,
        arena_tree: &mut ArenaTree,
    ) {
        let folder_node: Node = Node::new(Self::get_path_name(folder_path), String::from(""));
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
            let entry_path_ref = entry_path.as_path();

            println!("{} + {}", folder_path.to_str().unwrap(), entry_name);
            println!(" ");

            if entry.file_type().is_dir() {
                Self::build_arena_tree(entry_path_ref, &Some(folder_node_id), arena_tree);
            } else if entry.file_type().is_file() {
                let file_node = Node::new(entry_name.to_string(), String::from(""));
                arena_tree.add_node_to_parent_id(folder_node_id, file_node);
            } else {
                // do nothing
            }
        }
    }

    fn get_path_name(path: &Path) -> String {
        path.file_name().unwrap().to_str().unwrap().to_string()
    }

    fn get_modinfo_folder_name(modinfo_path: String) -> String {
        let path = Path::new(&modinfo_path);
        path.parent().unwrap().file_name().unwrap().to_str().unwrap().to_string()
    }

    fn get_modinfo_folder_path(modinfo_path: &String) -> String {
        let path = Path::new(&modinfo_path);
        path.parent().unwrap().to_str().unwrap().to_string()
    }

    pub fn build_node_path(&self, arena_tree: &ArenaTree,node_id: i32)-> String {
        let mut node = arena_tree.get_node_by_id(node_id).unwrap();

        let mut paths: Vec<String> = Vec::new();
        paths.push(node.value.clone());
    
        while node.parent.is_some() {
            node = arena_tree.get_node_by_id(node.parent.unwrap()).unwrap();
            paths.push(node.value.clone());
        }
    
        paths.reverse();
        // Убираю самую корневую папку
        if paths[0]==Self::get_modinfo_folder_name(self.modinfo_path.clone()) {
            paths.remove(0);
        }
        let mut path = paths.join("/");
        path = format!("/{}",path);
        path
    }

    pub fn get_node_absolute_path(&self, node_id: i32) -> String {
        let relative_path = self.build_node_path(&self.arena_tree, node_id);
        format!("{}{}", Self::get_modinfo_folder_path(&self.modinfo_path), relative_path)
    }

    pub fn read_json_node(&self, node_id: i32) -> Result<serde_json::Value, String> {
        let path = self.get_node_absolute_path(node_id);

        let content = fs::read_to_string(&path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        Ok(json)
    }

    pub fn find_file_by_relative_path(&self, relative_path: &String) -> Result<bool, String> {
        let path = format!("{}{}", Self::get_modinfo_folder_name(self.modinfo_path.clone()), relative_path);
        let path = Path::new(&path);
        if path.exists() {
            Ok(true)
        } else {
            Err(format!("File not found: {}", path.to_str().unwrap()))
        }
    }
}
