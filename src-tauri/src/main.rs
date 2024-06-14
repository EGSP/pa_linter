// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::panic;
use std::{cell::OnceCell, path::{Path, PathBuf}, process::Command, sync::OnceLock};

use analyzers::analyzer::FileAnalysisResult;
use directory_image::{get_directory_images, save_directory_image};
use editor::{editor::*, editor_runtime::EditorRuntimeData};
use nodes::{ArenaTree, Node, NodeId};
use parking_lot::RwLock;
use project::repos::{self, repository::{self, add_repository, remove_repository, Repository, RepositoryInfo}, repository_tree::{build_repository_tree, RepositoryTree}};
use rand::Rng;
use tauri::{api::file, State};
use ui::states::EditorRuntimeState;
use walkdir::{DirEntry, WalkDir};

use crate::{
    directory_image::{take_directory_image, DirectoryImage},
    project::project::Project,
    project::repos::repository::{get_repositories},
    logs::logbox::Log
};

mod analyzer;
mod analyzers;
mod directory_image;
mod nodes;
mod project;
mod editor;
mod ui;
mod quicks;
mod logs;

static PROJECT: OnceLock<Project> = OnceLock::new();
static EDITOR_ENVIRONMENT: OnceLock<EditorEnvironment> = OnceLock::new();

fn main() {
    let editor_env = try_ini_editor_environment();
    if editor_env.is_err() {
        panic!("{}", editor_env.err().unwrap());
    }
    let _ = EDITOR_ENVIRONMENT.set(editor_env.clone().unwrap());
    let editor_runtime_data = EditorRuntimeData::new(editor_env.unwrap());

    // let project = Project::try_initilize_project(PROJECT_TEST_FOLDER_PATH);
    // if project.is_err() {
    //     println!("{}", project.as_ref().err().unwrap());
    //     //panic!("{}", project.err().unwrap());
    // }
    // let _ = PROJECT.set(project.unwrap());
    
    // tauri::api::dialog::FileDialogBuilder::new()
    // .set_title("Choose folder to scan for repositories/mods")
    // .pick_folder(|folder_path| {
    //     if folder_path.is_none() {
    //         return;
    //     }
    //     let repositories = repository::find_repositories(&folder_path.unwrap());
    //     println!("{:?}", repositories);
    // });
    let devtools = devtools::init();
    tauri::Builder::default()
        .plugin(devtools)
        .manage(EditorRuntimeState(RwLock::new(editor_runtime_data)))
        .invoke_handler(tauri::generate_handler![
            c_take_directory_image,
            c_get_directory_images,
            c_get_repositories,
            c_add_repository,
            c_remove_repository,
            c_get_project_trees,
            c_reveal_in_explorer,
            c_reveal_workspace_folder,
            c_analyze_repositories
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn c_reveal_in_explorer(path: &str) {
    let _ = reveal_in_explorer(Path::new(path));
}

#[tauri::command]
fn c_reveal_workspace_folder(){
    let editor_env = EDITOR_ENVIRONMENT.get().unwrap();
    let _ = reveal_in_explorer(editor_env.get_workspace_folder().unwrap().as_path());
}

#[tauri::command]
fn c_get_project_trees(state: State<EditorRuntimeState>) -> Vec<RepositoryTree> {
    let repositories = get_repositories(&EDITOR_ENVIRONMENT.get().unwrap());

    let mut trees:Vec<RepositoryTree> = Vec::new();
    for repository in repositories {
        let tree = build_repository_tree(&repository);
        trees.push(tree);
    }

    let mut editor_runtime_data = state.0.write();
    editor_runtime_data.repository_trees = trees.clone();

    trees
}

#[tauri::command]
fn c_take_directory_image(folder_path: &Path) {
    let image = take_directory_image(folder_path.to_str().unwrap());
    save_directory_image(&image, EDITOR_ENVIRONMENT.get().unwrap());
}

#[tauri::command]
fn c_get_directory_images(state: State<EditorRuntimeState>) -> Vec<DirectoryImage> {
    let images =get_directory_images(EDITOR_ENVIRONMENT.get().unwrap());
    
    let mut editer_runtime_data = state.0.write();
    editer_runtime_data.directory_images = images.clone();
    images
}

#[tauri::command]
fn c_get_repositories() -> Vec<RepositoryInfo> {
    let repositories = get_repositories(&EDITOR_ENVIRONMENT.get().unwrap())
    .into_iter()
    .map(|repository| RepositoryInfo::from(repository))
    .collect();
    repositories
}

#[tauri::command]
fn c_add_repository(folder_path: &Path) {
    let editor_env = EDITOR_ENVIRONMENT.get().unwrap();
    let _ = add_repository(folder_path,&editor_env);
}

#[tauri::command]
fn c_remove_repository(repository_folder: &Path) {
    let editor_env = EDITOR_ENVIRONMENT.get().unwrap();
    let _ = remove_repository(repository_folder, &editor_env);
}

#[tauri::command]
fn c_analyze_repositories(state: State<EditorRuntimeState>) -> Vec<FileAnalysisResult> {
    let mut editor_runtime_data = state.0.read();

    let results = analyzers::analyzer::analyze_repositories(&mut editor_runtime_data);
    results
}


// найти свойства в json файлах со строковыми значениями.
// если значение это относительный путь, то нужно проверить его на корректность.

// проверка на корректность состоит из двух этапов:
// 1) проверить формат строки
// [строка должна начинаться на '/' и все символы '\' должны быть заменены на '/']
// 2) проверить наличие строки в справочнике, в котором хранится снимок файловой структуры
// если строка не прошла проверку,
// то нужно показать пользователю файл и название свойства

// справочник содержит снимок файловой структуры и название справочника
// снимок файловой структуры состоит из узлов
// узел хранит строковое значение, ссылку на родительский узел, список альтернативных узлов
// и список дочерних узлов,
// также узел хранит название своего справочника
// альтернативный узел - это узел, который имеет то же значение, но принадлежит другому справочнику

// если справочников несколько, то их нужно объединить под одним именем,
// объединить структуры, используя функционал альтернативных узлов

// перебор вглубь идет по ткому алгоритму:
// 1) у нас есть значение следующего узла
// 2) смотрим есть ли дочерний узел с таким значением в текущем узле
// 3) если нет, то продолжаем перебор в альтернативных узлах текущего узла
// 4) если нигде не нашли, то - нигде не нашли

// [альтернативные узлы позволяют по своей природе сохранять
// и держать всю исходную информацию, без потери её во время слияния деревьев]

// В программе есть:
// - справочники оригинальных структур
// - справочники рабочих структур

// справочники рабочих структур нужно обновлять при изменении файловой структуры или
// содержимого файлов.

// если какой-то файл изменяется или удаляется, то
// файл в рабочем справочнике нужно обновить.

// ДОПОЛНИТЕЛЬНО:
// узлы справочников могут хранить ссылки друг на друга, если
// свойства в файлах ссылаются на другие файлы. Но думаю это лучше делать
// отдельной мета-структурой.

// Добавить авто-фикс проблем с путями.
