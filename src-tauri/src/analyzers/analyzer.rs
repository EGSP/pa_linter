use crate::{editor::{editor::EditorEnvironment, editor_runtime::EditorRuntimeData}, project::repos::repository::{Repository, RepositoryInfo}};

pub struct FileAnalysisResult {
    pub file_path: String,

    pub repository_info: RepositoryInfo,

    pub messages: Vec<AnalysisMessage>
}

pub enum AnalysisMessage{
    Warning(String),
    Suggestion(String)
}



pub fn analyze_repositories(editor_runtime_data: &EditorRuntimeData) -> Vec<FileAnalysisResult> {
    let mut results:Vec<FileAnalysisResult> = Vec::new();

}

