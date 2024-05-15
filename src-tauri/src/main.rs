// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use nodes::ArenaTree;
use serde_json::{Map, Value};

mod nodes;
mod analyzer;

fn main() {

    const test_folder_path: &str = "c:/Workroot/softdev/pa_linter_test";
    
    let results = analyzer::analyze_folder(test_folder_path);
    for result in results {
        println!("{:#?}", result);
    }

    const tree_test_folder_path: &str = "c:/Workroot/softdev/pa_linter_test_tree/";

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, analyze_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn analyze_folder(folder_path: &str) -> Vec<analyzer::AnalysisResult> {
    analyzer::analyze_folder(folder_path)
}

// fn scan_project_folder(folder_path: &str) -> Result<ArenaTree, String> {
//     // if folder contains modinfo.json file, then it is a project folder
//     if(!std::path::Path::new(folder_path).join("modinfo.json").exists()) {
//         return Err("Folder is not a project folder. Project folder must contain modinfo.json file".to_string());   
//     }

//     let mut arena_tree = ArenaTree::new();

//     // add root node
//     let root_node = nodes::Node::new(String::from("root"), String::from(""));

//     // for entry in WalkDir::new(folder_path).contents_first(true) {
//     //     let entry = entry.unwrap();

//     // }
// }

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