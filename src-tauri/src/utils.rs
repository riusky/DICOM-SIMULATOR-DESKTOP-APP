use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::env;
use std::io;
use std::path::PathBuf;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};
use winreg::enums::*;
use winreg::RegKey;

/// 添加指定路径到当前用户的 Path 环境变量
///
/// # Arguments
///
/// * `new_path` - 要添加到 Path 的新路径
///
/// # Returns
///
/// * `Ok(())` - 成功添加路径
/// * `Err(io::Error)` - 出现错误
pub fn add_path_to_env(new_path: &str) -> io::Result<()> {
    // 打开当前用户的环境变量注册表键
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;

    // 获取当前的 Path 环境变量
    let current_path: String = env.get_value("Path").unwrap_or_default();

    // 输出当前的 Path 值，用于调试
    println!("Current Path: {}", current_path);

    // 检查新路径是否已经存在
    if !current_path.contains(new_path) {
        // 如果不存在，则将新路径添加到 Path 中
        let new_path_value = format!("{};{}", current_path, new_path);
        env.set_value("Path", &new_path_value)?;

        println!("Successfully added to Path: {}", new_path);
    } else {
        println!("Path already contains: {}", new_path);
    }

    Ok(())
}

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
