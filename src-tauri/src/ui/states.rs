use parking_lot::RwLock;

use crate::editor::editor_runtime::EditorRuntimeData;

pub struct EditorRuntimeState(pub RwLock<EditorRuntimeData>);