// main.rs or wherever your main logic resides
mod models;
mod hl7_client;
mod paths;
mod utils;
mod worklist; // Add this line to include the paths module

use models::DbState;
use paths::AppPath;
use std::sync::Arc;

use surrealdb::engine::local::File;

use surrealdb::Surreal;
use tauri::{command, AppHandle, Manager, State};
use tokio::sync::Mutex;
use utils::set_python_env; // Import the AppPath enum
use tauri_plugin_fs::FsExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                if let Err(e) = async_init_db(app_handle).await {
                    eprintln!("Failed to initialize database: {}", e);
                }
            });

            set_python_env(&app.handle())
                .map_err(|e| format!("Failed to set Python environment: {}", e))?;

                      // allowed the given directory
          let scope = app.fs_scope();
          scope.allow_directory("/log", false);

            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            worklist::search_worklist_data,
            worklist::create_worklist_entry,
            worklist::read_worklist_entry,
            worklist::update_worklist_entry,
            worklist::delete_worklist_entry,
            worklist::create_mpps_entry,
            worklist::update_mpps_entry,
            worklist::delete_mpps_entry,
            worklist::read_mpps_entry,
            worklist::get_base_dicom_dir,
            worklist::delete_mim_entry,
            worklist::update_mim_entry,
            worklist::read_mim_entry,
            worklist::create_mim_entry,
            worklist::send_to_pacs,
            worklist::create_hl7_setting_entry,
            worklist::read_hl7_setting_entry,
            worklist::update_hl7_setting_entry,
            worklist::delete_hl7_setting_entry,
            worklist::send_hl7_message,
            worklist::create_hl7_message_setting,
            worklist::read_hl7_message_setting,
            worklist::update_hl7_message_setting,
            worklist::delete_hl7_message_setting,
            worklist::send_rt_s,
            worklist::get_base_log_dir,
            worklist::read_log_file,
            worklist::create_patient_entry,
            worklist::read_patient_entry,
            worklist::update_patient_entry,
            worklist::delete_patient_entry,
            worklist::send_cstore_headless,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

async fn async_init_db(app_handle: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let database_folder = AppPath::Database.resolve(&app_handle)?;
    let db_path = std::path::Path::new(&database_folder);

    let db_path_str = db_path.to_str().ok_or_else(|| "Invalid database path")?;

    // Check if the path starts with \\? and remove it if necessary
    let db_path_str = if db_path_str.starts_with(r"\\?\") {
        &db_path_str[4..] // Remove the first 4 characters (i.e., \\?\)
    } else {
        db_path_str
    };

    let db = Surreal::new::<File>(db_path_str)
        .await
        .map_err(|e| format!("Failed to create SurrealDB instance: {}", e))?;
    db.use_ns("test").use_db("test").await?;
    let db_state = DbState {
        db: Arc::new(Mutex::new(db)),
    };
    app_handle.manage(db_state);
    Ok(())
}
