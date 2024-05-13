// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::{Map, Value};

mod nodes;
mod analyzer;

fn main() {
    // let binding = std::env::current_exe().unwrap();
    // let executable_directory = binding.parent().unwrap();

    // println!("executable directory: {:?}", executable_directory);

    // for entry in WalkDir::new(executable_directory) {
    //     let entry = entry.unwrap();
    //     if entry.file_type().is_dir() {
    //         println!("Directory: {}", entry.path().display());
    //     } else if entry.file_type().is_file() {
    //         println!("File: {}", entry.path().display());
    //     }else{
    //         println!("Other: {}", entry.path().display());
    //     }
    // }

    const test_json_path: &str = "c:/Workroot/softdev/pa_linter_test/orbital_fabrication_bot.json";

    let test_json_str = std::fs::read_to_string(test_json_path).unwrap();
    let test_json_value: Value = serde_json::from_str(&test_json_str).unwrap();

    let tips = analyzer::analyze_json(&test_json_value);
    println!("{:#?}", tips);
    

    // tauri::Builder::default()
    //     .invoke_handler(tauri::generate_handler![greet])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
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