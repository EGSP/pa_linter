use crate::{directory_image::DirectoryImage, project::repos::repository_tree::RepositoryTree, EditorEnvironment};

pub struct EditorRuntimeData{
    pub editor_env: EditorEnvironment,
    pub repository_trees: Vec<RepositoryTree>,
    pub directory_images: Vec<DirectoryImage>,
}

impl EditorRuntimeData {
    pub fn new(editor_env: EditorEnvironment) -> EditorRuntimeData {
        EditorRuntimeData {
            editor_env,
            repository_trees: Vec::new(),
            directory_images: Vec::new(),
        }
    }
}