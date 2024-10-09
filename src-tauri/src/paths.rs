// paths.rs
use tauri::path::BaseDirectory;
use tauri::AppHandle;
use std::path::PathBuf;
use crate::utils::resolve_path;

pub enum AppPath {
    Database,
    Dcm,
    CFindRq,
    MrModality,
    PythonScript,
    Certs,
    InProgress,
    DisContinued,
    Completed,
    Log
}

impl AppPath {
    pub fn resolve(&self, handle: &AppHandle) -> Result<PathBuf, String> {
        match self {
            AppPath::Log => resolve_path(handle, "log"),
            AppPath::Database => resolve_path(handle, "resources/database"),
            AppPath::Dcm => resolve_path(handle, "resources/dicom"),
            AppPath::CFindRq => resolve_path(handle, "resources/dcm/message/C-FIND-RQ.dcm"),
            AppPath::MrModality => resolve_path(handle, "resources/dcm/message/MR_Modality.dcm"),
            AppPath::InProgress => resolve_path(handle, "resources/dcm/message/mpps-inprogress.dcm"),
            AppPath::DisContinued => resolve_path(handle, "resources/dcm/message/mpps-discontinued.dcm"),
            AppPath::Completed => resolve_path(handle, "resources/dcm/message/mpps-completed.dcm"),
            AppPath::PythonScript => resolve_path(handle, "resources/python-script/dicom_utils.py"),
            AppPath::Certs => resolve_path(handle, "resources/certs"),
        }
        .map_err(|e| format!("Failed to resolve path: {}", e))
    }
}
