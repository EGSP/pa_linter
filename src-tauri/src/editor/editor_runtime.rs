use crate::{directory_image::DirectoryImage, project::repos::repository_tree::RepositoryTree, EditorEnvironment};

pub struct EditorRuntimeData{
    pub editor_env: EditorEnvironment,
    pub repository_trees: Vec<RepositoryTree>,
    pub directory_images: Vec<DirectoryImage>,
}