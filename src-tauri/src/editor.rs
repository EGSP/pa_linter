use std::{path::{Path, PathBuf}, process::Command};

use serde::{Deserialize, Serialize};

use crate::{directory_image::DirectoryImage, editor};

#[derive(Serialize,Deserialize,Debug)]
pub struct EditorEnvironment{
    pub editor_executable_path : String,
    pub editor_folder_path : String
}

impl EditorEnvironment{

    pub fn new(editor_executable_path : String, editor_folder_path : String) -> EditorEnvironment{
        EditorEnvironment{editor_executable_path, editor_folder_path}
    }
    
    pub fn get_workspace_folder(&self) -> Result<PathBuf,String>{
        let workspace_folder = PathBuf::from(&self.editor_folder_path).join(WORKSPACE_FOLDER_NAME);
        Ok(workspace_folder)
    }

    pub fn get_images_folder(&self) -> Result<PathBuf,String>{
        let images_folder = PathBuf::from(&self.editor_folder_path).join(IMAGES_FOLDER_NAME);
        Ok(images_folder)
    }

    pub fn get_directory_images(&self) -> Vec<DirectoryImage>
    {
        let mut directory_images : Vec<DirectoryImage> = Vec::new();
        // read images folder to find json files
        for entry in std::fs::read_dir(self.get_images_folder().unwrap()).unwrap() {
            let entry = entry.unwrap();
            if entry.file_name().to_str().unwrap().ends_with(".json"){
                let file_content = std::fs::read_to_string(entry.path()).unwrap();
                let directory_image = serde_json::from_str(&file_content).unwrap();
                directory_images.push(directory_image);
            }
        }

        directory_images
    }
}

const WORKSPACE_FOLDER_NAME : &str = "workspace";
const IMAGES_FOLDER_NAME : &str = "images";

pub fn try_ini_editor_environment() -> Result<EditorEnvironment,String>{
    let editor_executable_path = std::env::current_exe();
    if editor_executable_path.is_err(){
        return Err(String::from("Could not get editor executable path"));
    }
    let editor_executable_path = editor_executable_path.unwrap();

    let binding = std::env::current_exe().unwrap();
    let editor_folder_path = binding.parent();
    if editor_folder_path.is_none(){
        return Err(String::from("Could not get editor folder path"));
    }
    let editor_folder_path = editor_folder_path.unwrap();
    
    let workspace_folder = ini_workspace_folder(editor_folder_path);
    if workspace_folder.is_err(){
        return Err(String::from("Could not create workspace folder"));
    }
    let workspace_folder = workspace_folder.unwrap();

    let images_folder = ini_images_folder(&workspace_folder);
    if images_folder.is_err(){
        return Err(String::from("Could not create images folder"));
    }
    // let images_folder = images_folder.unwrap();


    Ok(EditorEnvironment::new(editor_executable_path.to_str().unwrap().to_string(), workspace_folder.to_str().unwrap().to_string()))
}

fn ini_workspace_folder(root_path: &Path) -> Result<PathBuf,String>{
   
    let workspace_folder = root_path.join(WORKSPACE_FOLDER_NAME);
    if workspace_folder.exists(){
        Ok(workspace_folder)
    }else{
        // create folder
        std::fs::create_dir(&workspace_folder);
        Ok(workspace_folder)
    }
}

fn ini_images_folder(root_path: &Path) -> Result<PathBuf,String>{
    let images_folder = root_path.join(IMAGES_FOLDER_NAME);
    if images_folder.exists(){
        Ok(images_folder)
    }else{
        // create folder
        std::fs::create_dir(&images_folder);
        Ok(images_folder)
    }
}

pub fn reveal_in_explorer(path: &Path)-> Result<(),String>{
    if !path.exists(){
        return Err(String::from("Path does not exist"));
    }

    let command = Command::new("explorer")
        .arg(path) // <- Specify the directory you'd like to open.
        .spawn()
        .unwrap();

    Ok(())
}

pub fn debug_editor_environment() -> Result<(),String>{
    println!("DEBUG EDITOR ENVIRONMENT:");
    let editor_env = try_ini_editor_environment();
    if editor_env.is_err(){
        return Err(editor_env.err().unwrap());
    }
    let editor_env = editor_env.unwrap();
    println!("EDITOR ENV: {:#?}", editor_env);

    let directory_images = editor_env.get_directory_images();
    println!("DIRECTORY IMAGES: ");
    for image in directory_images {
        println!("IMAGE name: {}; files: {}", image.name, image.files.len());
    }

    Ok(())
}