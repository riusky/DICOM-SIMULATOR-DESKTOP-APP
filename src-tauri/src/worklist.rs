use crate::hl7_client::{Hl7Client, SendMethod};
use crate::models::ApiResponse;
use crate::models::DbState;
use crate::models::DicomWorklistParams;
use crate::models::HL7MessageSetting;
use crate::models::Hl7SettingEntry;
use crate::models::MimEntry;
use crate::models::MppsEntry;
use crate::models::MppsResponse;
use crate::models::WorklistEntry;
use crate::models::PatientEntry;
use crate::paths::AppPath;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::types::{IntoPyDict, PyBool, PyNone};
use std::fs;
use std::result;
use tauri::{command, AppHandle, Manager, State};

#[tauri::command]
pub async fn send_hl7_message(
    message: &str,
    server_address: &str,
    port: &str,
    method: &str,
) -> Result<String, String> {
    let result = Hl7Client::new(
        message.to_owned(),
        server_address.to_owned(),
        port.parse().unwrap(),
        match method {
            "TCP" => SendMethod::Tcp,
            "HTTP" => SendMethod::Http,
            _ => SendMethod::Tcp,
        },
    )
    .send_message()
    .await;
    match result {
        Ok(response) => Ok(response),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub async fn create_hl7_setting_entry(
    db_state: State<'_, DbState>,
    entry: Hl7SettingEntry,
) -> Result<ApiResponse<Hl7SettingEntry>, String> {
    let db = db_state.db.lock().await;

    let created: Vec<Hl7SettingEntry> = db
        .create("hl7_setting")
        .content(entry)
        .await
        .map_err(|e| format!("Failed to create HL7 setting entry: {}", e))?;

    let created_entry = created
        .into_iter()
        .next()
        .ok_or_else(|| "Failed to create HL7 setting entry, no entries returned".to_string())?;

    Ok(ApiResponse::success(
        "HL7 setting entry created successfully",
        Some(created_entry),
    ))
}

#[command]
pub async fn read_hl7_setting_entry(
    db_state: State<'_, DbState>,
    id: Option<String>, // 使用 Option 来允许 id 为 None
) -> Result<ApiResponse<Vec<Hl7SettingEntry>>, String> {
    let db = db_state.db.lock().await;

    if let Some(id) = id {
        // 如果提供了 id，则查询单个条目
        let entry: Option<Hl7SettingEntry> = db
            .select(("hl7_setting", &id))
            .await
            .map_err(|e| format!("Failed to read HL7 setting entry: {}", e))?;

        match entry {
            Some(e) => Ok(ApiResponse::success(
                "HL7 setting entry found",
                Some(vec![e]),
            )), // 返回单个条目作为列表
            None => Ok(ApiResponse::error("No HL7 setting entry found", None)),
        }
    } else {
        // 如果没有提供 id，则查询全部条目
        let entries: Vec<Hl7SettingEntry> = db
            .select("hl7_setting")
            .await
            .map_err(|e| format!("Failed to read HL7 setting entries: {}", e))?;

        if !entries.is_empty() {
            Ok(ApiResponse::success(
                "HL7 setting entries found",
                Some(entries),
            ))
        } else {
            Ok(ApiResponse::error("No HL7 setting entries found", None))
        }
    }
}

#[command]
pub async fn update_hl7_setting_entry(
    db_state: State<'_, DbState>,
    id: String,
    updated_entry: Hl7SettingEntry,
) -> Result<ApiResponse<Hl7SettingEntry>, String> {
    let db = db_state.db.lock().await;

    let updated: Option<Hl7SettingEntry> = db
        .update(("hl7_setting", &id))
        .content(updated_entry)
        .await
        .map_err(|e| format!("Failed to update HL7 setting entry: {}", e))?;

    match updated {
        Some(e) => Ok(ApiResponse::success(
            "HL7 setting entry updated successfully",
            Some(e),
        )),
        None => Ok(ApiResponse::error(
            "No HL7 setting entry found to update",
            None,
        )),
    }
}

#[command]
pub async fn delete_hl7_setting_entry(
    db_state: State<'_, DbState>,
    id: String,
) -> Result<ApiResponse<Hl7SettingEntry>, String> {
    let db = db_state.db.lock().await;

    let hl7_setting_entry: Option<Hl7SettingEntry> = db
        .delete(("hl7_setting", &id))
        .await
        .map_err(|e| format!("Failed to delete HL7 setting entry: {}", e))?;

    match hl7_setting_entry {
        Some(e) => Ok(ApiResponse::success(
            "HL7 setting entry deleted successfully",
            Some(e),
        )),
        None => Ok(ApiResponse::error(
            "No HL7 setting entry found to delete",
            None,
        )),
    }
}

#[command]
pub async fn create_mim_entry(
    db_state: State<'_, DbState>,
    entry: MimEntry,
) -> Result<ApiResponse<MimEntry>, String> {
    let db = db_state.db.lock().await;

    let created: Vec<MimEntry> = db
        .create("mim")
        .content(entry)
        .await
        .map_err(|e| format!("Failed to create mim entry: {}", e))?;

    let created_entry = created
        .into_iter()
        .next()
        .ok_or_else(|| "Failed to create mim entry, no entries returned".to_string())?;

    Ok(ApiResponse::success(
        "Mim entry created successfully",
        Some(created_entry),
    ))
}

#[command]
pub async fn read_mim_entry(
    db_state: State<'_, DbState>,
    id: Option<String>, // 使用 Option 来允许 id 为 None
) -> Result<ApiResponse<Vec<MimEntry>>, String> {
    let db = db_state.db.lock().await;

    if let Some(id) = id {
        // 如果提供了 id，则查询单个条目
        let entry: Option<MimEntry> = db
            .select(("mim", &id))
            .await
            .map_err(|e| format!("Failed to read mim entry: {}", e))?;

        match entry {
            Some(e) => Ok(ApiResponse::success("Mim entry found", Some(vec![e]))), // 返回单个条目作为列表
            None => Ok(ApiResponse::error("No mim entry found", None)),
        }
    } else {
        // 如果没有提供 id，则查询全部条目
        let entries: Vec<MimEntry> = db
            .select("mim")
            .await
            .map_err(|e| format!("Failed to read mim entries: {}", e))?;

        if !entries.is_empty() {
            Ok(ApiResponse::success("Mim entries found", Some(entries)))
        } else {
            Ok(ApiResponse::error("No mim entries found", None))
        }
    }
}

#[command]
pub async fn update_mim_entry(
    db_state: State<'_, DbState>,
    id: String,
    updated_entry: MimEntry,
) -> Result<ApiResponse<MimEntry>, String> {
    let db = db_state.db.lock().await;

    let updated: Option<MimEntry> = db
        .update(("mim", &id))
        .content(updated_entry)
        .await
        .map_err(|e| format!("Failed to update mim entry: {}", e))?;

    match updated {
        Some(e) => Ok(ApiResponse::success(
            "Mim entry updated successfully",
            Some(e),
        )),
        None => Ok(ApiResponse::error("No mim entry found to update", None)),
    }
}

#[command]
pub async fn delete_mim_entry(
    db_state: State<'_, DbState>,
    id: String,
) -> Result<ApiResponse<MimEntry>, String> {
    let db = db_state.db.lock().await;

    let mim_entry: Option<MimEntry> = db
        .delete(("mim", &id))
        .await
        .map_err(|e| format!("Failed to delete mim entry: {}", e))?;

    match mim_entry {
        Some(e) => Ok(ApiResponse::success(
            "Mim entry deleted successfully",
            Some(e),
        )),
        None => Ok(ApiResponse::error("No mim entry found to delete", None)),
    }
}

#[command]
pub async fn search_worklist_data(
    id: String,
    db_state: State<'_, DbState>,
    handle: AppHandle,
) -> Result<String, String> {
    let db = db_state.db.lock().await;

    // 查询单个条目
    let entry: Option<WorklistEntry> = db
        .select(("worklist", &id))
        .await
        .map_err(|e| format!("Failed to read worklist entry: {}", e))?;

    // 如果没有查询到结果，抛出异常
    let entry = entry.ok_or_else(|| format!("No worklist entry found with id: {}", id))?;

    // 从 entry 中构建 DicomWorklistParams
    let params = DicomWorklistParams {
        calling_ae_title: entry.calling_ae_title,
        ae_title: entry.worklist_ae_title,
        ae_address: entry.worklist_ip,
        ae_port: entry.worklist_port.parse().unwrap_or(106), // 假设端口为字符串，需要解析
        c_find_rq_path: AppPath::CFindRq
            .resolve(&handle)?
            .to_string_lossy()
            .into_owned(),
        mr_modality_path: AppPath::MrModality
            .resolve(&handle)?
            .to_string_lossy()
            .into_owned(),
    };

    let certs_path = AppPath::Certs
    .resolve(&handle)?
    .to_string_lossy()
    .into_owned();

    // 解析 Python 脚本路径
    let script_path = AppPath::PythonScript.resolve(&handle)?;

    // 执行 Python 脚本
    let result: Result<String, String> = Python::with_gil(|py| {
        let script_content = fs::read_to_string(&script_path)
            .map_err(|e| format!("Failed to read Python script file: {}", e))?;

        let module =
            PyModule::from_code_bound(py, &script_content, "dicom_utils.py", "dicom_utils")
                .map_err(|e| format!("Failed to load Python module: {}", e))?;

        let get_work_list = module
            .getattr("get_work_list_with_paths")
            .and_then(|f| {
              let tls_enabled_py = match entry.tls_enabled {
                Some(value) => value,
                None => false,
            };
                f.call1((
                    &params.calling_ae_title,
                    &params.ae_title,
                    &params.ae_address,
                    params.ae_port,
                    &params.c_find_rq_path,
                    &params.mr_modality_path,
                    tls_enabled_py,
                    certs_path,
                ))
            })
            .map_err(|e| format!("Failed to call Python function: {}", e))?;

        let work_list_result = get_work_list
            .extract::<String>()
            .map_err(|e| format!("Failed to extract Python function result: {}", e))?;

        Ok(work_list_result)
    });

    // 返回结果
    match result {
        Ok(work_list_message) => Ok(work_list_message),
        Err(e) => Err(format!("An error occurred: {}", e)),
    }
}

#[command]
pub async fn create_worklist_entry(
    db_state: State<'_, DbState>,
    entry: WorklistEntry,
) -> Result<ApiResponse<WorklistEntry>, String> {
    let db = db_state.db.lock().await;

    let created: Vec<WorklistEntry> = db
        .create("worklist")
        .content(entry)
        .await
        .map_err(|e| format!("Failed to create worklist entry: {}", e))?;

    let created_entry = created
        .into_iter()
        .next()
        .ok_or_else(|| "Failed to create worklist entry, no entries returned".to_string())?;

    Ok(ApiResponse::success(
        "Worklist entry created successfully",
        Some(created_entry),
    ))
}

#[command]
pub async fn read_worklist_entry(
    db_state: State<'_, DbState>,
    id: Option<String>, // 使用 Option 来允许 id 为 None
) -> Result<ApiResponse<Vec<WorklistEntry>>, String> {
    let db = db_state.db.lock().await;

    if let Some(id) = id {
        // 如果提供了 id，则查询单个条目
        let entry: Option<WorklistEntry> = db
            .select(("worklist", &id))
            .await
            .map_err(|e| format!("Failed to read worklist entry: {}", e))?;

        match entry {
            Some(e) => Ok(ApiResponse::success("Worklist entry found", Some(vec![e]))), // 返回单个条目作为列表
            None => Ok(ApiResponse::error("No worklist entry found", None)),
        }
    } else {
        // 如果没有提供 id，则查询全部条目
        let entries: Vec<WorklistEntry> = db
            .select("worklist")
            .await
            .map_err(|e| format!("Failed to read worklist entries: {}", e))?;

        if !entries.is_empty() {
            Ok(ApiResponse::success(
                "Worklist entries found",
                Some(entries),
            ))
        } else {
            Ok(ApiResponse::error("No worklist entries found", None))
        }
    }
}

#[command]
pub async fn update_worklist_entry(
    db_state: State<'_, DbState>,
    id: String,
    updated_entry: WorklistEntry,
) -> Result<ApiResponse<WorklistEntry>, String> {
    let db = db_state.db.lock().await;

    let updated: Option<WorklistEntry> = db
        .update(("worklist", &id))
        .content(updated_entry)
        .await
        .map_err(|e| format!("Failed to update worklist entry: {}", e))?;

    match updated {
        Some(e) => Ok(ApiResponse::success(
            "Worklist entry updated successfully",
            Some(e),
        )),
        None => Ok(ApiResponse::error(
            "No worklist entry found to update",
            None,
        )),
    }
}

#[command]
pub async fn delete_worklist_entry(
    db_state: State<'_, DbState>,
    id: String,
) -> Result<ApiResponse<WorklistEntry>, String> {
    let db = db_state.db.lock().await;

    let worklist_entry: Option<WorklistEntry> = db
        .delete(("worklist", &id))
        .await
        .map_err(|e| format!("Failed to delete worklist entry: {}", e))?;

    match worklist_entry {
        Some(e) => Ok(ApiResponse::success(
            "Worklist entry delete successfully",
            Some(e),
        )),
        None => Ok(ApiResponse::error(
            "No worklist entry found to delete",
            None,
        )),
    }
}

#[command]
pub async fn create_mpps_entry(
    db_state: State<'_, DbState>,
    selected_id: String,
    mut entry: MppsEntry,
    handle: AppHandle,
) -> Result<ApiResponse<MppsEntry>, String> {
    // 查询数据

    // 调用python代码
    let db = db_state.db.lock().await;

    // 查询单个条目
    let worklist: Option<WorklistEntry> = db
        .select(("worklist", &selected_id))
        .await
        .map_err(|e| format!("Failed to read worklist entry: {}", e))?;

    // 如果没有查询到结果，抛出异常
    let worklist =
        worklist.ok_or_else(|| format!("No worklist entry found with id: {}", selected_id))?;
    let worklist_json = serde_json::to_string(&worklist).unwrap();
    let mpps_json = serde_json::to_string(&entry).unwrap();
    // 解析 Python 脚本路径
    let script_path = AppPath::PythonScript.resolve(&handle)?;
    let inprogress_path_str = AppPath::InProgress
        .resolve(&handle)?
        .to_string_lossy()
        .into_owned();
    // 执行 Python 脚本
    let result: Result<MppsResponse, String> = Python::with_gil(|py| {
        let script_content = fs::read_to_string(&script_path)
            .map_err(|e| format!("Failed to read Python script file: {}", e))?;

        let module =
            PyModule::from_code_bound(py, &script_content, "dicom_utils.py", "dicom_utils")
                .map_err(|e| format!("Failed to load Python module: {}", e))?;

        let certs_path = AppPath::Certs
        .resolve(&handle)?
        .to_string_lossy()
        .into_owned();

        let send_mpps_in_progress = module
            .getattr("send_mpps_in_progress")
            .and_then(|f: Bound<'_, PyAny>| {
                f.call1((worklist_json, mpps_json, inprogress_path_str, false, certs_path))
            })
            .map_err(|e| format!("Failed to call Python function: {}", e))?;

        let result_send_mpps_in_progress = send_mpps_in_progress
            .extract::<String>()
            .map_err(|e| format!("Failed to extract Python function result: {}", e))?;

        // 解析返回的 JSON 字符串
        let response: MppsResponse = serde_json::from_str(&result_send_mpps_in_progress)
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        if response.success {
            Ok(response)
        } else {
            Err(format!("Failed: {}", response.message))
        }
    });
    if let Err(e) = result {
        return Err(e);
    }

    if let Ok(response) = result {
        entry.mpps_instance_uid = Some(response.result);
    }

    let created: Vec<MppsEntry> = db
        .create("mpps")
        .content(entry)
        .await
        .map_err(|e| format!("Failed to create mpps entry: {}", e))?;

    let result = created.into_iter().next().unwrap();

    Ok(ApiResponse::success(
        "mpps entry created successfully",
        Some(result),
    ))
}

#[command]
pub async fn get_base_dicom_dir(handle: AppHandle) -> Result<String, String> {
    let dicom_base_dir = AppPath::Dcm
        .resolve(&handle)?
        .to_string_lossy()
        .into_owned();
    return Ok(dicom_base_dir);
}


#[command]
pub async fn get_base_log_dir(handle: AppHandle) -> Result<String, String> {
    let dicom_base_dir = AppPath::Log
        .resolve(&handle)?
        .to_string_lossy()
        .into_owned();
    return Ok(dicom_base_dir);
}

#[command]
pub async fn update_mpps_entry(
    db_state: State<'_, DbState>,
    dcm_file: String,
    selected_id: String,
    id: String,
    status: String,
    description: Option<String>,
    handle: AppHandle,
) -> Result<ApiResponse<MppsEntry>, String> {
    let db = db_state.db.lock().await;

    // 查询单个条目
    let worklist: Option<WorklistEntry> = db
        .select(("worklist", &selected_id))
        .await
        .map_err(|e| format!("Failed to read worklist entry: {}", e))?;

    // 如果没有查询到结果，抛出异常
    let worklist =
        worklist.ok_or_else(|| format!("No worklist entry found with id: {}", selected_id))?;

    // 查询单个条目
    let updated_entry: Option<MppsEntry> = db
        .select(("mpps", &id))
        .await
        .map_err(|e| format!("Failed to read worklist entry: {}", e))?;
    // 如果没有查询到结果，抛出异常
    let mut updated_entry =
        updated_entry.ok_or_else(|| format!("No worklist entry found with id: {}", id))?;
    updated_entry.status = Some(status);
    updated_entry.description = description;

    let worklist_json = serde_json::to_string(&worklist).unwrap();
    let mpps_json = serde_json::to_string(&updated_entry).unwrap();

    // 解析 Python 脚本路径
    let script_path = AppPath::PythonScript.resolve(&handle)?;
    let completed_path_str = AppPath::Completed
        .resolve(&handle)?
        .to_string_lossy()
        .into_owned();
    // 执行 Python 脚本
    let result: Result<MppsResponse, String> = Python::with_gil(|py| {
        let script_content = fs::read_to_string(&script_path)
            .map_err(|e| format!("Failed to read Python script file: {}", e))?;

        let module =
            PyModule::from_code_bound(py, &script_content, "dicom_utils.py", "dicom_utils")
                .map_err(|e| format!("Failed to load Python module: {}", e))?;


        let certs_path = AppPath::Certs
        .resolve(&handle)?
        .to_string_lossy()
        .into_owned();
        let send_mpps_completed = module
            .getattr("send_mpps_completed")
            .and_then(|f: Bound<'_, PyAny>| {
                f.call1((
                    worklist_json,
                    mpps_json,
                    &dcm_file,
                    completed_path_str,
                    false,
                    certs_path,
                ))
            })
            .map_err(|e| format!("Failed to call Python function: {}", e))?;

        let result_send_mpps_completed = send_mpps_completed
            .extract::<String>()
            .map_err(|e| format!("Failed to extract Python function result: {}", e))?;

        // 解析返回的 JSON 字符串
        let response: MppsResponse = serde_json::from_str(&result_send_mpps_completed)
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        if response.success {
            Ok(response)
        } else {
            Err(format!("Failed: {}", response.message))
        }
    });

    if let Err(e) = result {
        return Err(e);
    }

    if let Ok(response) = result {
        updated_entry.sop_instance_uids = Some(response.result);
        updated_entry.dcm_file = Some(dcm_file);
    }

    let updated: Option<MppsEntry> = db
        .update(("mpps", &id))
        .content(updated_entry)
        .await
        .map_err(|e| format!("Failed to update mpps entry: {}", e))?;

    match updated {
        Some(e) => Ok(ApiResponse::success(
            "mpps entry updated successfully",
            Some(e),
        )),
        None => Ok(ApiResponse::error("No mpps entry found to update", None)),
    }
}

#[command]
pub async fn read_mpps_entry(
    db_state: State<'_, DbState>,
    id: Option<String>, // 使用 Option 来允许 id 为 None
) -> Result<ApiResponse<Vec<MppsEntry>>, String> {
    let db = db_state.db.lock().await;

    if let Some(id) = id {
        // 如果提供了 id，则查询单个条目
        let entry: Option<MppsEntry> = db
            .select(("mpps", &id))
            .await
            .map_err(|e| format!("Failed to read mpps entry: {}", e))?;

        match entry {
            Some(e) => Ok(ApiResponse::success("mpps entry found", Some(vec![e]))), // 返回单个条目作为列表
            None => Ok(ApiResponse::error("No mpps entry found", None)),
        }
    } else {
        // 如果没有提供 id，则查询全部条目
        let entries: Vec<MppsEntry> = db
            .select("mpps")
            .await
            .map_err(|e| format!("Failed to read mpps entries: {}", e))?;

        if !entries.is_empty() {
            Ok(ApiResponse::success("mpps entries found", Some(entries)))
        } else {
            Ok(ApiResponse::error("No mpps entries found", None))
        }
    }
}

#[command]
pub async fn delete_mpps_entry(
    db_state: State<'_, DbState>,
    id: String,
) -> Result<ApiResponse<Vec<MppsEntry>>, String> {
    let db = db_state.db.lock().await; 
    if id == "all" {
        // 查询 status 为 COMPLETED 的条目
        let mut complete_entries = db
            .query("SELECT * FROM mpps WHERE status = 'COMPLETED'")
            .await
            .map_err(|e| format!("Failed to retrieve complete mpps entries: {}", e))?;

          // 提取结果集
          let result: Vec<MppsEntry> = complete_entries.take(0).map_err(|e| format!("Failed to extract mpps entries: {}", e))?;

          // 删除查询出来的每一条记录
          for entry in &result {
              if let Some(ref id) = entry.id {
                let tb: String = id.tb.clone();
                let raw = &id.id.to_raw();
                let _: Option<MppsEntry> = db.delete((tb, raw))
                      .await
                      .map_err(|e| format!("Failed to delete mpps entry with id {}: {}", id, e))?;
              } else {
                  return Err("Missing ID for an entry, cannot delete".to_string());
              }
          }

          Ok(ApiResponse::success(
              "All complete mpps entries deleted successfully",
              Some(result),
          ))
    } else {
        let worklist_entry: Option<MppsEntry> = db
            .delete(("mpps", &id))
            .await
            .map_err(|e| format!("Failed to delete mpps entry: {}", e))?;
        
        match worklist_entry {
            Some(e) => Ok(ApiResponse::success(
                "mpps entry deleted successfully",
                Some(vec![e]),
            )),
            None => Ok(ApiResponse::error("No mpps entry found to delete", None)),
        }
    }
}

#[command]
pub async fn send_to_pacs(
    db_state: State<'_, DbState>,
    id: String,
    current_id: String,
    handle: AppHandle,
) -> Result<ApiResponse<String>, String> {
    let db = db_state.db.lock().await;

    let mim_entry: Option<MimEntry> = db
        .select(("mim", &id))
        .await
        .map_err(|e| format!("Failed to read MimEntry entry: {}", e))?;

    let mim_entry = mim_entry.ok_or_else(|| format!("No MimEntry entry found with id: {}", id))?;

    let mpps_entry: Option<MppsEntry> = db
        .select(("mpps", &current_id))
        .await
        .map_err(|e| format!("Failed to read MppsEntry entry: {}", e))?;

    let mpps_entry =
        mpps_entry.ok_or_else(|| format!("No MppsEntry entry found with id: {}", current_id))?;

    let script_path = AppPath::PythonScript.resolve(&handle)?;

    let mim_entry_json = serde_json::to_string(&mim_entry).unwrap();
    let mpps_entry_json = serde_json::to_string(&mpps_entry).unwrap();

    let result: Result<String, String> = Python::with_gil(|py| {
        let script_content = fs::read_to_string(&script_path)
            .map_err(|e| format!("Failed to read Python script file: {}", e))?;

        let module =
            PyModule::from_code_bound(py, &script_content, "dicom_utils.py", "dicom_utils")
                .map_err(|e| format!("Failed to load Python module: {}", e))?;
        let certs_path = AppPath::Certs
        .resolve(&handle)?
        .to_string_lossy()
        .into_owned();
        let get_work_list = module
            .getattr("send_c_store_requests")
            .and_then(|f| f.call1((mpps_entry_json, mim_entry_json, certs_path)))
            .map_err(|e| format!("Failed to call Python function: {}", e))?;

        let work_list_result = get_work_list
            .extract::<String>()
            .map_err(|e| format!("Failed to extract Python function result: {}", e))?;

        // 解析返回的 JSON 字符串
        let response: MppsResponse = serde_json::from_str(&work_list_result)
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        if response.success {
            Ok(response.message)
        } else {
            Err(format!("Failed: {}", response.message))
        }
    });
    if let Err(e) = result {
        return Err(e);
    }

    Ok(ApiResponse::success("message", None))
}

#[command]
pub async fn create_hl7_message_setting(
    db_state: State<'_, DbState>,
    entry: HL7MessageSetting,
) -> Result<ApiResponse<HL7MessageSetting>, String> {
    let db = db_state.db.lock().await;

    let created: Vec<HL7MessageSetting> = db
        .create("hl7_message_setting")
        .content(entry)
        .await
        .map_err(|e| format!("Failed to create HL7 message setting entry: {}", e))?;

    let created_entry = created.into_iter().next().ok_or_else(|| {
        "Failed to create HL7 message setting entry, no entries returned".to_string()
    })?;

    Ok(ApiResponse::success(
        "HL7 message setting entry created successfully",
        Some(created_entry),
    ))
}

#[command]
pub async fn read_hl7_message_setting(
    db_state: State<'_, DbState>,
    id: Option<String>, // 使用 Option 来允许 id 为 None
) -> Result<ApiResponse<Vec<HL7MessageSetting>>, String> {
    let db = db_state.db.lock().await;

    if let Some(id) = id {
        // 如果提供了 id，则查询单个条目
        let entry: Option<HL7MessageSetting> = db
            .select(("hl7_message_setting", &id))
            .await
            .map_err(|e| format!("Failed to read HL7 message setting entry: {}", e))?;

        match entry {
            Some(e) => Ok(ApiResponse::success(
                "HL7 message setting entry found",
                Some(vec![e]),
            )), // 返回单个条目作为列表
            None => Ok(ApiResponse::error(
                "No HL7 message setting entry found",
                None,
            )),
        }
    } else {
        // 如果没有提供 id，则查询全部条目
        let entries: Vec<HL7MessageSetting> = db
            .select("hl7_message_setting")
            .await
            .map_err(|e| format!("Failed to read HL7 message setting entries: {}", e))?;

        if !entries.is_empty() {
            Ok(ApiResponse::success(
                "HL7 message setting entries found",
                Some(entries),
            ))
        } else {
            Ok(ApiResponse::error(
                "No HL7 message setting entries found",
                None,
            ))
        }
    }
}

#[command]
pub async fn update_hl7_message_setting(
    db_state: State<'_, DbState>,
    id: String,
    updated_entry: HL7MessageSetting,
) -> Result<ApiResponse<HL7MessageSetting>, String> {
    let db = db_state.db.lock().await;

    let updated: Option<HL7MessageSetting> = db
        .update(("hl7_message_setting", &id))
        .content(updated_entry)
        .await
        .map_err(|e| format!("Failed to update HL7 message setting entry: {}", e))?;

    match updated {
        Some(e) => Ok(ApiResponse::success(
            "HL7 message setting entry updated successfully",
            Some(e),
        )),
        None => Ok(ApiResponse::error(
            "No HL7 message setting entry found to update",
            None,
        )),
    }
}

#[command]
pub async fn delete_hl7_message_setting(
    db_state: State<'_, DbState>,
    id: String,
) -> Result<ApiResponse<HL7MessageSetting>, String> {
    let db = db_state.db.lock().await;

    let hl7_message_setting_entry: Option<HL7MessageSetting> = db
        .delete(("hl7_message_setting", &id))
        .await
        .map_err(|e| format!("Failed to delete HL7 message setting entry: {}", e))?;

    match hl7_message_setting_entry {
        Some(e) => Ok(ApiResponse::success(
            "HL7 message setting entry deleted successfully",
            Some(e),
        )),
        None => Ok(ApiResponse::error(
            "No HL7 message setting entry found to delete",
            None,
        )),
    }
}

#[command]
pub async fn send_rt_s(
    db_state: State<'_, DbState>,
    dcm_file: String,
    selected_id: String,
    id: String,
    handle: AppHandle,
) -> Result<ApiResponse<MppsEntry>, String> {
    let db = db_state.db.lock().await;

    let mim_entry: Option<MimEntry> = db
        .select(("mim", &selected_id))
        .await
        .map_err(|e| format!("Failed to read MimEntry entry: {}", e))?;

    let mim_entry = mim_entry.ok_or_else(|| format!("No MimEntry entry found with id: {}", selected_id))?;

    // 查询单个条目
    let mpps_entry: Option<MppsEntry> = db
        .select(("mpps", &id))
        .await
        .map_err(|e| format!("Failed to read worklist entry: {}", e))?;
    // 如果没有查询到结果，抛出异常
    let mpps_entry =
        mpps_entry.ok_or_else(|| format!("No worklist entry found with id: {}", id))?;

    let mim_entry_json = serde_json::to_string(&mim_entry).unwrap();
    let mpps_json = serde_json::to_string(&mpps_entry).unwrap();

    // 解析 Python 脚本路径
    let script_path = AppPath::PythonScript.resolve(&handle)?;
    // 执行 Python 脚本
    let result: Result<MppsResponse, String> = Python::with_gil(|py| {
        let script_content = fs::read_to_string(&script_path)
            .map_err(|e| format!("Failed to read Python script file: {}", e))?;

        let module =
            PyModule::from_code_bound(py, &script_content, "dicom_utils.py", "dicom_utils")
                .map_err(|e| format!("Failed to load Python module: {}", e))?;

        let send_rt_s = module
            .getattr("send_rt_s")
            .and_then(|f: Bound<'_, PyAny>| f.call1((mim_entry_json, mpps_json, &dcm_file, false)))
            .map_err(|e| format!("Failed to call Python function: {}", e))?;

        let result_send_rt_s = send_rt_s
            .extract::<String>()
            .map_err(|e| format!("Failed to extract Python function result: {}", e))?;

        // 解析返回的 JSON 字符串
        let response: MppsResponse = serde_json::from_str(&result_send_rt_s)
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        if response.success {
            Ok(response)
        } else {
            Err(format!("Failed: {}", response.message))
        }
    });

    if let Err(e) = result {
        return Err(e);
    }

    Ok(ApiResponse::success(
        "mpps entry updated successfully",
        None,
    ))
}


#[command]
pub async fn read_log_file(handle: AppHandle) -> Result<String, String> {
  let dicom_base_dir = AppPath::Log
  .resolve(&handle)?
  .to_string_lossy()
  .into_owned();
    let log_file_path = format!(r#"{}\app.log"#,dicom_base_dir);

    let log_file_path = if log_file_path.starts_with(r"\\?\") {
        &log_file_path[4..] // Remove the first 4 characters (i.e., \\?\)
    } else {
        &log_file_path
    };
    
    // 读取文件内容
    match fs::read_to_string(log_file_path) {
        Ok(content) => Ok(content),
        Err(err) => Err(format!("Failed to read log file: {}", dicom_base_dir)),
    }
}

#[command]
pub async fn create_patient_entry(
    db_state: State<'_, DbState>,
    entry: PatientEntry,
) -> Result<ApiResponse<PatientEntry>, String> {
    let db = db_state.db.lock().await;

    let created: Vec<PatientEntry> = db
        .create("patient")
        .content(entry)
        .await
        .map_err(|e| format!("Failed to create patient entry: {}", e))?;

    let created_entry = created
        .into_iter()
        .next()
        .ok_or_else(|| "Failed to create patient entry, no entries returned".to_string())?;

    Ok(ApiResponse::success(
        "Patient entry created successfully",
        Some(created_entry),
    ))
}



#[command]
pub async fn read_patient_entry(
    db_state: State<'_, DbState>,
    id: Option<String>, // 使用 Option 允许 id 为 None
) -> Result<ApiResponse<Vec<PatientEntry>>, String> {
    let db = db_state.db.lock().await;

    if let Some(id) = id {
        // 如果提供了 id，则查询单个条目
        let entry: Option<PatientEntry> = db
            .select(("patient", &id))
            .await
            .map_err(|e| format!("Failed to read patient entry: {}", e))?;

        match entry {
            Some(e) => Ok(ApiResponse::success(
                "Patient entry found",
                Some(vec![e]),
            )),
            None => Ok(ApiResponse::error("No patient entry found", None)),
        }
    } else {
        // 如果没有提供 id，则查询全部条目
        let entries: Vec<PatientEntry> = db
            .select("patient")
            .await
            .map_err(|e| format!("Failed to read patient entries: {}", e))?;

        if !entries.is_empty() {
            Ok(ApiResponse::success(
                "Patient entries found",
                Some(entries),
            ))
        } else {
            Ok(ApiResponse::error("No patient entries found", None))
        }
    }
}


#[command]
pub async fn update_patient_entry(
    db_state: State<'_, DbState>,
    id: String,
    updated_entry: PatientEntry,
) -> Result<ApiResponse<PatientEntry>, String> {
    let db = db_state.db.lock().await;

    let updated: Option<PatientEntry> = db
        .update(("patient", &id))
        .content(updated_entry)
        .await
        .map_err(|e| format!("Failed to update patient entry: {}", e))?;

    match updated {
        Some(e) => Ok(ApiResponse::success(
            "Patient entry updated successfully",
            Some(e),
        )),
        None => Ok(ApiResponse::error(
            "No patient entry found to update",
            None,
        )),
    }
}


#[command]
pub async fn delete_patient_entry(
    db_state: State<'_, DbState>,
    id: String,
) -> Result<ApiResponse<PatientEntry>, String> {
    let db = db_state.db.lock().await;

    let patient_entry: Option<PatientEntry> = db
        .delete(("patient", &id))
        .await
        .map_err(|e| format!("Failed to delete patient entry: {}", e))?;

    match patient_entry {
        Some(e) => Ok(ApiResponse::success(
            "Patient entry deleted successfully",
            Some(e),
        )),
        None => Ok(ApiResponse::error(
            "No patient entry found to delete",
            None,
        )),
    }
}


#[command]
pub async fn send_cstore_headless(
    db_state: State<'_, DbState>,
    dcm_file: String,
    selected_id: String,
    id: String,
    description: Option<String>,
    generate: Option<bool>,
    handle: AppHandle,
) -> Result<ApiResponse<PatientEntry>, String> {
    let db = db_state.db.lock().await;

    let mim_entry: Option<MimEntry> = db
        .select(("mim", &selected_id))
        .await
        .map_err(|e| format!("Failed to read MimEntry entry: {}", e))?;

    let mim_entry = mim_entry.ok_or_else(|| format!("No MimEntry entry found with id: {}", selected_id))?;

    // 查询单个条目
    let mpps_entry: Option<PatientEntry> = db
        .select(("patient", &id))
        .await
        .map_err(|e| format!("Failed to read worklist entry: {}", e))?;
    // 如果没有查询到结果，抛出异常
    let mut mpps_entry =
        mpps_entry.ok_or_else(|| format!("No worklist entry found with id: {}", id))?;
    mpps_entry.description = description;
    mpps_entry.generate = generate;

    let mim_entry_json = serde_json::to_string(&mim_entry).unwrap();
    let mpps_json = serde_json::to_string(&mpps_entry).unwrap();

    // 解析 Python 脚本路径
    let script_path = AppPath::PythonScript.resolve(&handle)?;
    // 执行 Python 脚本
    let result: Result<MppsResponse, String> = Python::with_gil(|py| {
        let script_content = fs::read_to_string(&script_path)
            .map_err(|e| format!("Failed to read Python script file: {}", e))?;

        let module =
            PyModule::from_code_bound(py, &script_content, "dicom_utils.py", "dicom_utils")
                .map_err(|e| format!("Failed to load Python module: {}", e))?;

        let send_rt_s = module
            .getattr("send_cstore_headless")
            .and_then(|f: Bound<'_, PyAny>| f.call1((mim_entry_json, mpps_json, &dcm_file, false)))
            .map_err(|e| format!("Failed to call Python function: {}", e))?;

        let result_send_rt_s = send_rt_s
            .extract::<String>()
            .map_err(|e| format!("Failed to extract Python function result: {}", e))?;

        // 解析返回的 JSON 字符串
        let response: MppsResponse = serde_json::from_str(&result_send_rt_s)
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        if response.success {
            Ok(response)
        } else {
            Err(format!("Failed: {}", response.message))
        }
    });

    if let Err(e) = result {
        return Err(e);
    }
    if let Ok(response) = result {
      mpps_entry.sop_instance_uids = Some(response.result);
    }

    let updated: Option<PatientEntry> = db
        .update(("patient", &id))
        .content(mpps_entry)
        .await
        .map_err(|e| format!("Failed to update patient entry: {}", e))?;

    match updated {
        Some(e) => Ok(ApiResponse::success(
            "patient entry updated successfully",
            Some(e),
        )),
        None => Ok(ApiResponse::error("No patient entry found to update", None)),
    }
}