// src/models.rs
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use tokio::sync::Mutex;

use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug)]
pub struct MppsResponse {
    pub success: bool,
    pub message: String,
    pub result: String, // 可以用来存储可能返回的result UID
}




#[derive(Serialize, Deserialize, Debug)]
pub struct MppsEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>, // 使用 Option 以便在创建时可以为空
    #[serde(rename = "AccessionNumber")]
    pub accession_number: String,

    #[serde(rename = "RequestedProcedureDescription")]
    pub requested_procedure_description: String,

    #[serde(rename = "PatientName")]
    pub patient_name: String,

    #[serde(rename = "PatientID")]
    pub patient_id: String,

    #[serde(rename = "PatientBirthDate")]
    pub patient_birth_date: String,

    #[serde(rename = "PatientSex")]
    pub patient_sex: String,

    #[serde(rename = "Modality")]
    pub modality: String,

    #[serde(rename = "ScheduledStationAETitle")]
    pub scheduled_station_ae_title: String,

    #[serde(rename = "ScheduledProcedureStepStartDate")]
    pub scheduled_procedure_step_start_date: String,

    #[serde(rename = "ScheduledPerformingPhysicianName")]
    pub scheduled_performing_physician_name: String,

    #[serde(rename = "StudyInstanceUID")]
    pub study_instance_uid: String,

    pub status: Option<String>, // Optional to allow empty status initially

    #[serde(rename = "MppsInstanceUid")]
    pub mpps_instance_uid: Option<String>, 

    #[serde(rename = "SopInstanceUids")]
    pub sop_instance_uids: Option<String>,

    #[serde(rename = "DcmFile")]
    pub dcm_file: Option<String>,

    pub description: Option<String>, // Optional to allow empty status initially

}

#[derive(Debug, Serialize, Deserialize)]
pub struct MimEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>, // 使用 Option 以便在创建时可以为空
    pub name: String,      // 名称字段
    pub calling_ae_title: String, // 调用方 AE 标题
    pub ae_title: String,  // AE 标题
    pub ip: String,        // IP 地址
    pub port: String,      // 端口号
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hl7SettingEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>, // 使用 Option 以便在创建时可以为空
    pub name: String,      // 名称字段
    pub ip: String,        // IP 地址
    pub port: String,     // 端口号
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HL7MessageSetting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,  // 使用 Option 以便在创建时可以为空
    pub name: String,       // 名称字段
    pub message: String,    // HL7 报文字段
}


#[derive(Debug, Serialize, Deserialize)]
pub struct WorklistEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>, // 使用 Option 以便在创建时可以为空
    pub name: String,
    pub calling_ae_title: String,
    pub worklist_ae_title: String,
    pub worklist_ip: String,
    pub worklist_port: String,
    
    // 新增的 MPPS 相关字段
    pub mpps_calling_ae_title: String,
    pub mpps_ae_title: String,
    pub mpps_port: String,
}


pub struct DicomWorklistParams {
    pub calling_ae_title: String,
    pub ae_title: String,
    pub ae_address: String,
    pub ae_port: u16,
    pub c_find_rq_path: String,
    pub mr_modality_path: String,
}

pub struct DbState {
    pub db: Arc<Mutex<Surreal<Db>>>,
}

// 通用 API 响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    // 成功响应
    pub fn success(message: &str, data: Option<T>) -> Self {
        ApiResponse {
            success: true,
            message: message.to_string(),
            data,
            error: None,
        }
    }

    // 失败响应
    pub fn error(message: &str, error: Option<String>) -> Self {
        ApiResponse {
            success: false,
            message: message.to_string(),
            data: None,
            error,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PatientEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>, // 使用 Option 以便在创建时可以为空
    pub patient_name: String,      // 患者姓名
    pub patient_id: String,        // 患者ID
    pub patient_birth_date: String, // 出生日期
    pub patient_sex: String,        // 性别
    pub description: Option<String>, // Optional to allow empty status initially
    pub sop_instance_uids: Option<String>, // Optional to allow empty status initially
}
