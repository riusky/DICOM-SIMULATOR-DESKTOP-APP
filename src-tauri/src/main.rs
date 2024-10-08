// Prevents additional console window on Windows in release, DO NOT REMOVE!!

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use std::env;
// mod utils;
// use utils::add_path_to_env; 
#[tokio::main]
async fn main(){
    // 获取当前工作目录
    // let current_exe = env::current_exe().expect("Failed to get current executable path");
    // let current_dir = current_exe
    //     .parent()
    //     .expect("Failed to get parent directory");
    // let python_env_path = current_dir
    //     .join("resources")
    //     .join("python-env")
    //     .join("Python312");
    // // 将 PathBuf 转换为字符串
    // let python_env_str = python_env_path
    //     .to_str()
    //     .expect("Failed to convert path to string");
    //   let _ = add_path_to_env(python_env_str);
    dicom_desktop_lib::run().await.expect("Failed to run the application")
}
