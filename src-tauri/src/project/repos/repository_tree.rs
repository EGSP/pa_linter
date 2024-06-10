use std::{collections::HashMap, path::Path};

use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use super::repository::{Repository, RepositoryInfo};

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Copy, Clone)]
pub struct EntryID(i32);

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct RelativePath(String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepositoryTreeEntry {
    pub id: EntryID,

    /// full path with disk and extensions
    pub path: String,
    pub parent: Option<EntryID>,
    pub children: Vec<EntryID>,
}

impl RepositoryTreeEntry {
    pub fn new(id: EntryID, parent: Option<EntryID>, path: String) -> Self {
        Self {
            id,
            path,
            parent,
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryTree {
    pub entries: Vec<RepositoryTreeEntry>,
    pub last_generated_id: EntryID,

    pub repository_info: RepositoryInfo,
}

impl RepositoryTree {
    pub fn new(repository_info: RepositoryInfo) -> Self {
        Self {
            entries: Vec::new(),
            last_generated_id: EntryID(0),
            repository_info,
        }
    }

    fn generate_id(&mut self) -> EntryID {
        self.last_generated_id.0 += 1;
        self.last_generated_id
    }

    pub fn add_entry(&mut self, entry: &mut RepositoryTreeEntry) {
        let id = self.generate_id();
        entry.id = id;
        self.entries.push(entry.clone());
        // add entry to parent children
        if let Some(parent_id) = entry.parent {
            self.entries
                .iter_mut()
                .find(|entry| entry.id == parent_id)
                .unwrap()
                .children
                .push(id);
        }
    }
}

pub fn build_repository_tree(repository: &Repository) -> RepositoryTree {
    let mut repository_tree = RepositoryTree::new(RepositoryInfo::from(repository.clone()));
    build_repository_tree_recursive(
        None,
        Path::new(&repository.folder_path),
        &mut repository_tree,
    );

    repository_tree
}

fn build_repository_tree_recursive(
    parent_id: Option<EntryID>,
    tree_entry_path: &Path,
    repository_tree: &mut RepositoryTree,
) {
    let mut tree_entry = RepositoryTreeEntry::new(
        EntryID(-1),
        parent_id,
        tree_entry_path.to_str().unwrap().to_string(),
    );

    repository_tree.add_entry(&mut tree_entry);

    // min_depth = 1 - чтобы не учитывать уже переданного родителя
    // max_depth = 1 - потому что функция build_repository_tree_recursive рекурсивная и сама ходит по папкам. Иначе walkdir будет для каждого вызова функции обходить все дерево
    for entry in WalkDir::new(tree_entry_path.to_str().unwrap())
        .min_depth(1)
        .max_depth(1)
    {
        if (entry.is_err()) {
            continue;
        }

        let entry = entry.unwrap();
        let entry_path = tree_entry_path.join(entry.file_name());

        if entry.file_type().is_dir() {
            build_repository_tree_recursive(
                Some(tree_entry.id),
                entry_path.as_path(),
                repository_tree,
            );
        } else if entry.file_type().is_file() {
            let mut file_tree_entry = RepositoryTreeEntry::new(
                EntryID(-1),
                Some(tree_entry.id),
                entry_path.to_str().unwrap().to_string(),
            );
            repository_tree.add_entry(&mut file_tree_entry);
        } else {
            // do nothing
        }
    }
}

pub fn get_entry_relative_path(
    repository_tree: &RepositoryTree,
    repository_tree_entry: &RepositoryTreeEntry,
) -> RelativePath {
    let root_path = repository_tree.repository_info.folder_path.clone();
    let absolute_entry_path = repository_tree_entry.path.clone();

    RelativePath(
        absolute_entry_path
            .strip_prefix(&root_path)
            .unwrap()
            .to_string(),
    )
}

fn find_repository_entry(
    search_relative_path: &RelativePath,
    repository_trees: &Vec<RepositoryTree>,
) -> Option<(RepositoryInfo, RepositoryTreeEntry)> {
    for repository_tree in repository_trees {
        
        for entry in &repository_tree.entries {
            let entry_relative_path = get_entry_relative_path(repository_tree, entry);
            if (entry_relative_path == *search_relative_path) {
                return Some((repository_tree.repository_info.clone(), entry.clone()));
            }
        }
    }

    None
}
