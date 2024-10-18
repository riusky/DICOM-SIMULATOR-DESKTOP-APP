use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::env;
use std::io;
use std::path::PathBuf;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};

/// Resolves a path using Tauri's `BaseDirectory::Resource`.
/// Returns a `PathBuf` to allow more flexibility with file paths.
pub fn resolve_path(handle: &AppHandle, path: &str) -> Result<PathBuf, String> {
    handle
        .path()
        .resolve(path, BaseDirectory::Resource)
        .map_err(|e| format!("Failed to resolve path '{}': {}", path, e))
}

/// Sets the Python environment by resolving the path to the Python environment
/// directory and setting the `PYTHONPATH` environment variable.
pub fn set_python_env(handle: &AppHandle) -> Result<(), String> {
    // let python_env_path = resolve_path(handle, "resources/python-env/Python312")?;

    // // Convert the PathBuf to a string for setting the environment variable
    // let python_env_str = python_env_path
    //     .to_str()
    //     .ok_or_else(|| format!("Failed to convert path to string: {:?}", python_env_path))?;
    // let python_env_str = if python_env_str.starts_with(r"\\?\") {
    //     &python_env_str[4..] // Remove the first 4 characters (i.e., \\?\)
    // } else {
    //     python_env_str
    // };
    // let _ = add_path_to_env(python_env_str);

    // Prepare Python for use in a multi-threaded context
    pyo3::prepare_freethreaded_python();

    Ok(())
}
