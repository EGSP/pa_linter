use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::{editor::{editor::EditorEnvironment, editor_runtime::EditorRuntimeData}, project::repos::repository::{Repository, RepositoryInfo}};

use super::json_analyzer::JsonAnalyzeTask;


#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct FileAnalysisResult {
    pub file_path: String,

    pub repository_info: RepositoryInfo,

    pub messages: Vec<Hint>
}

#[derive(Serialize,Deserialize,Debug,Clone)]
#[serde(rename_all = "lowercase", tag = "hint_type")]
pub enum Hint{
    JSON {
        category: Category,
        property_name: String,
        property_value: String,
        message: String
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
#[serde(rename_all = "lowercase")]
pub enum Category{
    Warning,
    Suggestion
}


pub fn analyze_repositories(editor_runtime_data: &EditorRuntimeData) -> Vec<FileAnalysisResult> {
    let mut results:Vec<FileAnalysisResult> = Vec::new();

    let TIMECHECK = std::time::Instant::now();
    let json_analyze_task = JsonAnalyzeTask::new(editor_runtime_data);
    results.append(&mut json_analyze_task.run());

    let ELAPSED = TIMECHECK.elapsed().as_secs_f32();
    println!("{} {}: execution time {} sec","FUNCTION".on_bright_black(), "json analyzer".blue(), ELAPSED.to_string());
    results
}

