from pydicom.uid import generate_uid
from pynetdicom import AE, debug_logger, build_context
from pynetdicom.sop_class import (
    ModalityWorklistInformationFind,
    ModalityPerformedProcedureStep
)
import json
from datetime import datetime
from pydicom import dcmread
import os
from collections import defaultdict
import copy
from pynetdicom.sop_class import uid_to_sop_class

from dataclasses import dataclass, asdict
from typing import List, Optional, Dict
from dataclasses import dataclass, fields
import ssl
from pathlib import Path

# 定义SopInstanceInfo类
@dataclass
class SopInstanceInfo:
    sop_instance_uid: str
    path: str
    SOPClassUID: str

# 定义SopInstanceUIDs类
@dataclass
class SopInstanceUIDs:
    series_instance_uid: str
    SOPClassUID: str
    sop_instance_infos: List[SopInstanceInfo]

# 定义MppsEntry类
@dataclass
class MppsEntry:
    AccessionNumber: str
    RequestedProcedureDescription: str
    PatientName: str
    PatientID: str
    PatientBirthDate: str
    PatientSex: str
    Modality: str
    ScheduledStationAETitle: str
    ScheduledProcedureStepStartDate: str
    ScheduledPerformingPhysicianName: str
    StudyInstanceUID: str
    status: Optional[str] = None
    MppsInstanceUid: Optional[str] = None
    SopInstanceUids: Optional[List[SopInstanceUIDs]] = None  # 可选字段
    DcmFile: Optional[str] = None  # 可选字段
    description: Optional[str] = None
    

    # 自定义的序列化方法
    def to_json(self):
        return json.dumps(asdict(self), indent=4)

    # 自定义的反序列化方法
    @staticmethod
    def from_json(json_str: str):
        data = json.loads(json_str)
        if 'id' in data:
            del data['id']
        # 反序列化 SopInstanceUids
        if data.get('SopInstanceUids'):
            sop_instance_uids = []
            
            # 首先，将 SopInstanceUids 从 JSON 字符串解析为 Python 对象
            sop_instance_uids_list = json.loads(data['SopInstanceUids'])
            
            for sop in sop_instance_uids_list:
                sop_instance_infos = [
                    SopInstanceInfo(**info) for info in sop['sop_instance_infos']
                ]
                sop_instance_uids.append(SopInstanceUIDs(
                    series_instance_uid=sop['series_instance_uid'],
                    SOPClassUID=sop['SOPClassUID'],
                    sop_instance_infos=sop_instance_infos
                ))
            data['SopInstanceUids'] = sop_instance_uids
        
        return MppsEntry(**data)

# 自定义处理 SopInstanceUids 字段的序列化和反序列化
def serialize_sop_instance_uids(sop_instance_uids: List[SopInstanceUIDs]) -> str:
    # 将 SopInstanceUIDs 列表序列化为 JSON 字符串
    return json.dumps([asdict(sop) for sop in sop_instance_uids])

def deserialize_sop_instance_uids(sop_instance_uids_str: str) -> List[SopInstanceUIDs]:
    # 将 JSON 字符串反序列化为 SopInstanceUIDs 列表
    sop_list = json.loads(sop_instance_uids_str)
    return [
        SopInstanceUIDs(
            series_instance_uid=sop['series_instance_uid'],
            SOPClassUID=sop['SOPClassUID'],
            sop_instance_infos=[SopInstanceInfo(**info) for info in sop['sop_instance_infos']]
        )
        for sop in sop_list
    ]

@dataclass
class WorklistEntry:
    name: str
    calling_ae_title: str
    worklist_ae_title: str
    worklist_ip: str
    worklist_port: str
    mpps_calling_ae_title: str
    mpps_ae_title: str
    mpps_port: str
    tlsEnabled: bool = False
    
@dataclass
class MimEntry:
    name: str
    calling_ae_title: str
    ae_title: str
    ip: str
    port: str
    tlsEnabled: bool = False

def json_to_dataclass(json_str: str, cls):
    data = json.loads(json_str)
    cls_fields = {f.name for f in fields(cls)}
    filtered_data = {k: v for k, v in data.items() if k in cls_fields}
    return cls(**filtered_data)

# Utility function to establish association
def establish_association(calling_ae_title, ae_title, ae_address, ae_port, context, debug=False, tls_enabled_py=False, certs_path=None):
    if debug:
        debug_logger()
    certs_path = Path(certs_path)
    # 使用 Path 对象拼接路径
    cafile = certs_path / 'ca.pem'
    certfile = certs_path / 'tls.pem'
    keyfile = certs_path / 'tls.key'
    print('======================')
    print('======================')
    print('======================')
    print(tls_enabled_py)
    print(certs_path)
    print(cafile)
    print(certfile)
    print(keyfile)
    print('======================')
    print('======================')
    # Create the SSLContext, your requirements may vary
    # ssl_cx = ssl.create_default_context(ssl.Purpose.CLIENT_AUTH, cafile='server.crt')
    # ssl_cx.verify_mode = ssl.CERT_REQUIRED
    # ssl_cx.load_cert_chain(certfile='client.crt', keyfile='client.key')
    ae = AE(calling_ae_title)
    ae.add_requested_context(context)
    if tls_enabled_py:
        ssl_cx = ssl.create_default_context(ssl.Purpose.SERVER_AUTH, cafile=cafile)
        ssl_cx.check_hostname = False
        ssl_cx.verify_mode = ssl.CERT_NONE
        ssl_cx.verify_mode = ssl.CERT_REQUIRED
        ssl_cx.load_cert_chain(certfile=certfile, keyfile=keyfile)
        assoc = ae.associate(ae_address, ae_port, ae_title=ae_title, tls_args=(ssl_cx, None))
    else:
        assoc = ae.associate(ae_address, ae_port, ae_title=ae_title)
    
    if not assoc.is_established:
        print('Association rejected, aborted or never connected')
        return None
    
    return assoc

# Utility function to send C-FIND request
def send_c_find(assoc, ds):
    responses = assoc.send_c_find(ds, ModalityWorklistInformationFind, msg_id=99)
    result = []

    for status, identifier in responses:
        if status:
            print('C-FIND query status: 0x{0:04x}'.format(status.Status))
        if identifier:
            result.append(identifier.to_json())
        else:
            print('Connection timed out, was aborted or received invalid response')

    return result

# Function to get work list
def get_work_list_with_paths(calling_ae_title, ae_title, ae_address, ae_port, c_find_rq_path, mr_modality_path, tls_enabled_py, certs_path, debug=False):
    debug_logger()
    ds1 = dcmread(c_find_rq_path)
    ds2 = dcmread(mr_modality_path)
    assoc = establish_association(calling_ae_title, ae_title, ae_address, ae_port, ModalityWorklistInformationFind, debug, tls_enabled_py, certs_path)
    if assoc is None:
        return []
    result2 = send_c_find(assoc, ds1)
    result1 = send_c_find(assoc, ds2)
    assoc.release()
    result1_extracted = [extract_fields(item) for item in result1]
    accession_numbers_in_result1 = {item['AccessionNumber'] for item in result1_extracted}
    filtered_result2 = [extract_fields(item) for item in result2 if extract_fields(item)['AccessionNumber'] not in accession_numbers_in_result1]
    return json.dumps(result1_extracted + filtered_result2)

# Function to extract fields from DICOM data
def extract_fields(data):
    dicom_data = json.loads(data)
    result = {
        "AccessionNumber": dicom_data.get("00080050", {}).get("Value", [""])[0],
        "RequestedProcedureDescription": dicom_data.get("00401001", {}).get("Value", [""])[0],
        "PatientName": dicom_data.get("00100010", {}).get("Value", [{}])[0].get("Alphabetic", ""),
        "PatientID": dicom_data.get("00100020", {}).get("Value", [""])[0],
        "PatientBirthDate": dicom_data.get("00100030", {}).get("Value", [""])[0],
        "PatientSex": dicom_data.get("00100040", {}).get("Value", [""])[0],
        'Modality': dicom_data.get("00400100", {}).get("Value", [""])[0].get("00080060", {}).get("Value", [""])[0],
        'ScheduledStationAETitle': dicom_data.get("00400100", {}).get("Value", [""])[0].get("00400001", {}).get("Value", [""])[0],
        'ScheduledProcedureStepStartDate': dicom_data.get("00400100", {}).get("Value", [""])[0].get("00400002", {}).get("Value", [""])[0],
        'ScheduledPerformingPhysicianName': dicom_data.get("00400100", {}).get("Value", [""])[0].get("00400006", {}).get("Value", [""])[0].get("Alphabetic", ""),
        'StudyInstanceUID': dicom_data.get("0020000D", {}).get("Value", [""])[0],
    }
    return result

# Function to build attribute list for N-CREATE
def build_attr_list_in_progress(data, PerformedProcedureStepStatus,path):
    ct_study_uid = data.get('StudyInstanceUID', '')
    ds = dcmread(path)
    # ds.ScheduledStepAttributesSequence = [Dataset()]
    step_seq = ds.ScheduledStepAttributesSequence
    ct_study_uid = data.get('StudyInstanceUID', '')
    step_seq[0].StudyInstanceUID = ct_study_uid
    step_seq[0].ReferencedStudySequence[0].SpecificCharacterSet = 'ISO_IR 100'
    # del step_seq[0].ReferencedStudySequence[0].ReferencedSOPInstanceUID
    step_seq[0].AccessionNumber = data.get('AccessionNumber', '')
    step_seq[0].RequestedProcedureID = data.get('RequestedProcedureDescription', '')
    step_seq[0].RequestedProcedureDescription = data.get('RequestedProcedureDescription', '')
    step_seq[0].ScheduledProcedureStepID = data.get('RequestedProcedureDescription', '')
    step_seq[0].ScheduledProcedureStepDescription = data.get('RequestedProcedureDescription', '')
    step_seq[0].ScheduledProcedureProtocolCodeSequence = []
    ds.PatientName = data.get('PatientName', '')
    ds.PatientID = data.get('PatientID', '')
    ds.PatientBirthDate = data.get('PatientBirthDate', '')
    ds.PatientSex = data.get('PatientSex', '')
    ds.ReferencedPatientSequence = []
    ds.PerformedProcedureStepID = 'PPS ID ' + data.get('AccessionNumber', '')
    ds.PerformedStationAETitle = data.get('ScheduledStationAETitle', '')
    ds.PerformedStationName = data.get('ScheduledPerformingPhysicianName', '')
    ds.PerformedLocation = data.get('ScheduledPerformingPhysicianName', '')
    ds.PerformedProcedureStepStartDate = data.get('ScheduledProcedureStepStartDate', '')
    now = datetime.now() 
    time_int_str = now.strftime("%H%M%S")
    ds.PerformedProcedureStepStartTime = time_int_str
    ds.PerformedProcedureStepStatus = PerformedProcedureStepStatus
    ds.PerformedProcedureStepDescription = 'description'
    ds.PerformedProcedureTypeDescription = 'type'
    ds.PerformedProcedureCodeSequence = []
    ds.PerformedProcedureStepEndDate = None
    ds.PerformedProcedureStepEndTime = None
    ds.Modality = data.get('Modality', '')
    ds.StudyID = data.get('AccessionNumber', '')
    ds.PerformedProtocolCodeSequence = []
    ds.PerformedSeriesSequence = []
    ds.PerformedProcedureStepDiscontinuationReasonCodeSequence = []
    return ds
def build_attr_list_discontinued(data, PerformedProcedureStepStatus):
    ds = dcmread('./system_data/message/mpps-discontinued.dcm')
    # ds.ScheduledStepAttributesSequence = [Dataset()]
    step_seq = ds.PerformedSeriesSequence
    step_seq[0].SeriesInstanceUID = generate_uid()
    step_seq[0].ReferencedImageSequence = []
    now = datetime.now()
    time_int_str = now.strftime("%H%M%S")
    date_int_str = now.strftime('%Y%m%d')
    ds.PerformedProcedureStepStatus = PerformedProcedureStepStatus
    ds.PerformedProcedureStepEndDate = date_int_str
    ds.PerformedProcedureStepEndTime = time_int_str
    return ds

# Function to send N-CREATE request
def send_mpps_in_progress(worklist_json, mpps_json, path, debug=False, certs_path=None):
    debug_logger()
    print(worklist_json)
    try:
        mpps_entry = json_to_dataclass(mpps_json, MppsEntry)

        worklist_entry = json_to_dataclass(worklist_json, WorklistEntry)
        print(worklist_entry)
        assoc = establish_association(
            worklist_entry.mpps_calling_ae_title,
            worklist_entry.mpps_ae_title,
            worklist_entry.worklist_ip,
            int(worklist_entry.mpps_port),
            ModalityPerformedProcedureStep,
            debug,
            worklist_entry.tlsEnabled,
            certs_path
        )
        if assoc is None:
            return json.dumps({"success": False, "message": "Failed to establish association"})

        result = mpps_entry.MppsInstanceUid or generate_uid()
        ds = build_attr_list_in_progress({
            'AccessionNumber': mpps_entry.AccessionNumber,
            'RequestedProcedureDescription': mpps_entry.RequestedProcedureDescription,
            'PatientName': mpps_entry.PatientName,
            'PatientID': mpps_entry.PatientID,
            'PatientBirthDate': mpps_entry.PatientBirthDate,
            'PatientSex': mpps_entry.PatientSex,
            'Modality': mpps_entry.Modality,
            'ScheduledStationAETitle': worklist_entry.calling_ae_title,
            'ScheduledProcedureStepStartDate': mpps_entry.ScheduledProcedureStepStartDate,
            'ScheduledPerformingPhysicianName': worklist_entry.name,
            'StudyInstanceUID': mpps_entry.StudyInstanceUID,
            'status': mpps_entry.status
        }, "IN_PROGRESS", path)

        status, attr_list = assoc.send_n_create(ds, ModalityPerformedProcedureStep, result)

        if status:
            message = 'N-CREATE request status: 0x{0:04x}'.format(status.Status)
            print(message)
            response = {"success": True, "message": message, "result": result}
        else:
            message = 'Connection timed out, was aborted, or received invalid response'
            print(message)
            response = {"success": False, "message": message}

        assoc.release()
        return json.dumps(response)

    except Exception as e:
        # 捕获所有异常并返回异常信息
        return json.dumps({"success": False, "message": str(e)})

def send_mpps_discontinued(calling_ae_title, ae_title, ae_address, ae_port, data, debug=False):
    assoc = establish_association(calling_ae_title, ae_title, ae_address, ae_port, ModalityPerformedProcedureStep, debug)
    if assoc is None:
        return None
    result = None
    if data.get('mpps_instance_uid'):
        result = data.get('mpps_instance_uid')
    else:
        result = generate_uid()
    
    ds = build_attr_list_discontinued(data['data'], data['currentState'])
    status, attr_list = assoc.send_n_set(ds, ModalityPerformedProcedureStep, result)
    
    if status:
        print('N-CREATE request status: 0x{0:04x}'.format(status.Status))
    else:
        print('Connection timed out, was aborted or received invalid response')
        result = None
    
    assoc.release()
    return result

import json

def send_mpps_completed(worklist_json, mpps_json, dcmFile, path, debug=False, certs_path=None):
    debug_logger()
    try:
        mpps_entry = MppsEntry.from_json(mpps_json)
        worklist_entry = json_to_dataclass(worklist_json, WorklistEntry)
        result = collect_dcm_files(dcmFile)
        mpps_entry.SopInstanceUids = result
        mpps_entry.DcmFile = path

        if mpps_entry.SopInstanceUids:
            ds = build_mod_list(mpps_entry, path)
            status = send_n_set(mpps_entry, worklist_entry, ds, certs_path)
            if status and status.Status == 0x0000:
                return json.dumps({
                    "success": True,
                    "message": "MPPS completed successfully.",
                    "result": serialize_sop_instance_uids(mpps_entry.SopInstanceUids)
                })
            else:
                return json.dumps({
                    "success": False,
                    "message": f"MPPS failed with status: 0x{status.Status:04x}",
                    "result": ""
                })
        else:
            return json.dumps({
                "success": False,
                "message": "No SOP Instance UIDs found.",
                "result": ""
            })
    except Exception as e:
        return json.dumps({
            "success": False,
            "message": f"Error occurred: {str(e)}",
            "result": ""
        })

# Function to send N-SET request
def send_n_set(mpps_entry: MppsEntry, worklist_entry: WorklistEntry, ds, certs_path=None):
    try:
        assoc = establish_association(
            worklist_entry.mpps_calling_ae_title,
            worklist_entry.mpps_ae_title,
            worklist_entry.worklist_ip,
            int(worklist_entry.mpps_port),
            ModalityPerformedProcedureStep,
            False,
            worklist_entry.tlsEnabled,
            certs_path
        )

        if assoc is None:
            return None

        # Send the N-SET request for the series
        status, attr_list = assoc.send_n_set(
            ds,
            ModalityPerformedProcedureStep,
            mpps_entry.MppsInstanceUid
        )

        assoc.release()
        return status

    except Exception as e:
        print(f"Error during N-SET request: {str(e)}")
        return None

# Function to build attribute list for N-SET
def build_mod_list(mpps_entry: MppsEntry, path: str):
    sop_instance_info = mpps_entry.SopInstanceUids
    ds = dcmread(path)
    now = datetime.now()
    
    # 设置结束日期和时间
    ds.PerformedProcedureStepEndDate = now.strftime('%Y%m%d')
    ds.PerformedProcedureStepEndTime = now.strftime('%H%M')
    
    performedSeriesSequenceTemplate = ds.PerformedSeriesSequence[0]
    referencedImageSequenceTemplate = ds.PerformedSeriesSequence[0].ReferencedImageSequence[0]
    ds.PerformedSeriesSequence = []
    
    # 遍历 sop_instance_info 列表
    for series_info in sop_instance_info:
        performedSeriesSequence = copy.deepcopy(performedSeriesSequenceTemplate)
        
        # 使用点符号访问属性，而不是使用下标
        performedSeriesSequence.SeriesInstanceUID = series_info.series_instance_uid
        performedSeriesSequence.PerformingPhysicianName = mpps_entry.ScheduledPerformingPhysicianName
        if mpps_entry.description is not None:
          # performedSeriesSequence.OperatorsName = mpps_entry.description
          performedSeriesSequence.SeriesDescription = mpps_entry.description
        performedSeriesSequence.ReferencedImageSequence = []
        
        # 遍历 sop_instance_infos 列表
        for instance in series_info.sop_instance_infos:
            referencedImageSequence = copy.deepcopy(referencedImageSequenceTemplate)
            # 使用点符号访问属性，而不是下标
            referencedImageSequence.ReferencedSOPClassUID = series_info.SOPClassUID
            referencedImageSequence.ReferencedSOPInstanceUID = instance.sop_instance_uid
            if performedSeriesSequence.OperatorsName == 'iRT DICOM Device Simulator':
                temp_ds = dcmread(instance.path)
                if temp_ds.SeriesDescription:
                    performedSeriesSequence.OperatorsName = temp_ds.SeriesDescription
                    performedSeriesSequence.SeriesDescription = temp_ds.SeriesDescription
            
            performedSeriesSequence.ReferencedImageSequence.append(referencedImageSequence)
        
        ds.PerformedSeriesSequence.append(performedSeriesSequence)

    return ds


def collect_dcm_files(path: str) -> List[SopInstanceUIDs]:
    sop_instance_uids = []

    if not os.path.exists(path):
        print("Path does not exist.")
        return sop_instance_uids

    for root, dirs, files in os.walk(path):
        for file in files:
            if file.endswith(".dcm"):
                ds = dcmread(os.path.join(root, file))
                if 'SOPClassUID' in ds:
                    SOPClassUID = ds.SOPClassUID
                    sop_instance_uid = {
                        "sop_instance_uid": generate_uid(),
                        "path": os.path.join(root, file),
                        "SOPClassUID": SOPClassUID,
                        "root_path": root
                    }
                    sop_instance_uids.append(sop_instance_uid)
    
    if sop_instance_uids:
        categorized_data = defaultdict(list)

        # 分类数据
        for item in sop_instance_uids:
            categorized_data[item['root_path']].append(SopInstanceInfo(
                sop_instance_uid=item['sop_instance_uid'],
                path=item['path'],
                SOPClassUID=item['SOPClassUID']
            ))
        
        # 生成 List[SopInstanceUIDs]
        result = [
            SopInstanceUIDs(
                series_instance_uid=generate_uid(),
                SOPClassUID=sop_instance_infos[0].SOPClassUID,
                sop_instance_infos=sop_instance_infos
            )
            for sop_instance_infos in categorized_data.values()
        ]
        return result

    return []


# Function to send C-STORE requests
def send_c_store_requests(mpps_entry, mim_entry,certs_path=''):
    debug_logger()
    print(mpps_entry)
    try:
        mpps_entry = MppsEntry.from_json(mpps_entry)
        mim_entry = json_to_dataclass(mim_entry, MimEntry)
        calling_ae_title = mim_entry.calling_ae_title
        pacs_ae_title = mim_entry.ae_title
        ip = mim_entry.ip
        port = int(mim_entry.port)
        # Initialise the Application Entity
        ae = AE(ae_title=calling_ae_title)
        patient_data = {
            "PatientName": mpps_entry.PatientName,
            "PatientID": mpps_entry.PatientID,
            "PatientBirthDate": mpps_entry.PatientBirthDate,
            "PatientSex": mpps_entry.PatientSex,
            "StudyInstanceUID": mpps_entry.StudyInstanceUID,
        }

        # Ensure SopInstanceUids is provided
        if not mpps_entry.SopInstanceUids:
            return json.dumps({
                "success": False,
                "message": "No SOP Instance UIDs available for this MPPS entry.",
                "result": ""
            })
        # Loop through the SopInstanceUids in MppsEntry
        for sop_instance_uid_data in mpps_entry.SopInstanceUids:
            sop_class_uid = sop_instance_uid_data.SOPClassUID
            sop_class = uid_to_sop_class(sop_class_uid)
            
            if sop_class is None:
                print(f"Unsupported SOP Class UID: {sop_class_uid}")
                continue
            
            # Create a presentation context for the SOP Class
            tem_ds = dcmread(sop_instance_uid_data.sop_instance_infos[0].path)
            context = build_context(sop_class, tem_ds.file_meta.TransferSyntaxUID)
            # Associate with the peer AE
            if mim_entry.tlsEnabled:
                certs_path = Path(certs_path)
                # 使用 Path 对象拼接路径
                cafile = certs_path / 'ca.pem'
                certfile = certs_path / 'tls.pem'
                keyfile = certs_path / 'tls.key'
                ssl_cx = ssl.create_default_context(ssl.Purpose.SERVER_AUTH, cafile=cafile)
                ssl_cx.check_hostname = False
                ssl_cx.verify_mode = ssl.CERT_NONE
                ssl_cx.verify_mode = ssl.CERT_REQUIRED
                ssl_cx.load_cert_chain(certfile=certfile, keyfile=keyfile)
                assoc = ae.associate(ip, port, contexts=[context], ae_title=pacs_ae_title, tls_args=(ssl_cx, None))
            else:
                assoc = ae.associate(ip, port, contexts=[context], ae_title=pacs_ae_title)
            
            if assoc.is_established:
                for sop_instance_info in sop_instance_uid_data.sop_instance_infos:
                    file_path = sop_instance_info.path
                    ds = dcmread(file_path)
                    
                    # Update DICOM dataset with new patient and SOP instance information
                    now = datetime.now()
                    date_int_str = now.strftime('%Y%m%d')
                    time_int_str = now.strftime("%H%M%S")
                    
                    ds.InstanceCreationDate = date_int_str
                    ds.InstanceCreationTime = time_int_str
                    ds.SOPInstanceUID = sop_instance_info.sop_instance_uid
                    ds.PatientName = patient_data["PatientName"]
                    ds.PatientID = patient_data["PatientID"]
                    ds.PatientBirthDate = patient_data["PatientBirthDate"]
                    ds.PatientSex = patient_data["PatientSex"]
                    ds.StudyInstanceUID = patient_data["StudyInstanceUID"]
                    ds.SeriesInstanceUID = sop_instance_uid_data.series_instance_uid
                    ds.SeriesDescription = mpps_entry.description
                    
                    # Send the C-STORE request
                    status = assoc.send_c_store(ds)
                    
                    # Check the status of the storage request
                    if status and status.Status == 0x0000:  # Success status code
                        print(f'C-STORE request status for {file_path}: SUCCESS')
                    else:
                        print(f'C-STORE request status for {status.Status}')
                        print(f'Connection timed out, was aborted, or received invalid response for {file_path}')
                        return json.dumps({
                            "success": False,
                            "message": f"Failed to store file {file_path}",
                            "result": ""
                        })
                
                # Release association
                assoc.release()
            else:
                print(f'Association rejected, aborted, or never connected for SOP Class UID: {sop_class_uid} on {ip}:{port}')
                return json.dumps({
                    "success": False,
                    "message": f"Association rejected for SOP Class UID: {sop_class_uid}",
                    "result": ""
                })
        
        # If all operations are successful
        return json.dumps({
            "success": True,
            "message": "C-STORE requests completed successfully.",
            "result": mpps_entry.MppsInstanceUid if mpps_entry.MppsInstanceUid else ""
        })

    except Exception as e:
        # Handle any exceptions and return an error response
        print(f"An error occurred: {str(e)}")
        return json.dumps({
            "success": False,
            "message": f"An error occurred: {str(e)}",
            "result": ""
        })
        
def send_rt_s(mim_entry, mpps_entry, dcmFile, debug=False):
    debug_logger()
    try:
        mpps_entry = MppsEntry.from_json(mpps_entry)
        mim_entry = json_to_dataclass(mim_entry, MimEntry)
        calling_ae_title = mim_entry.calling_ae_title
        pacs_ae_title = mim_entry.ae_title
        ip = mim_entry.ip
        port = int(mim_entry.port)
        # Initialise the Application Entity
        ae = AE(ae_title=calling_ae_title)
        patient_data = {
            "PatientName": mpps_entry.PatientName,
            "PatientID": mpps_entry.PatientID,
            "PatientBirthDate": mpps_entry.PatientBirthDate,
            "PatientSex": mpps_entry.PatientSex,
            "StudyInstanceUID": mpps_entry.StudyInstanceUID,
        }

        # Ensure SopInstanceUids is provided
        if not mpps_entry.SopInstanceUids:
            return json.dumps({
                "success": False,
                "message": "No SOP Instance UIDs available for this MPPS entry.",
                "result": ""
            })    
        ds = dcmread(dcmFile)
        now = datetime.now()
        date_int_str = now.strftime('%Y%m%d')
        time_int_str = now.strftime("%H%M%S")
        
        ds.InstanceCreationDate = date_int_str
        ds.InstanceCreationTime = time_int_str
        ds.SOPInstanceUID = generate_uid()
        ds.PatientName = patient_data["PatientName"]
        ds.PatientID = patient_data["PatientID"]
        ds.PatientBirthDate = patient_data["PatientBirthDate"]
        ds.PatientSex = patient_data["PatientSex"]
        ds.StudyInstanceUID = patient_data["StudyInstanceUID"]
        ds.SeriesInstanceUID = generate_uid()
        frameOfReferenceUID = generate_uid()
        ds.ReferencedFrameOfReferenceSequence[0].FrameOfReferenceUID = frameOfReferenceUID
        ds.StructureSetROISequence[0].ReferencedFrameOfReferenceUID = frameOfReferenceUID
        ds.StructureSetROISequence[1].ReferencedFrameOfReferenceUID = frameOfReferenceUID
        ds.StructureSetROISequence[2].ReferencedFrameOfReferenceUID = frameOfReferenceUID
        ds.ReferencedFrameOfReferenceSequence[0].RTReferencedStudySequence[0].ReferencedSOPInstanceUID = patient_data["StudyInstanceUID"]
        ds.ReferencedFrameOfReferenceSequence[0].RTReferencedStudySequence[0].RTReferencedSeriesSequence[0].SeriesInstanceUID = generate_uid()
        sop_instance_uid_list = [
            "1.2.840.113619.2.374.2807.4219983.23592.1454490194.266",
            "1.2.840.113619.2.374.2807.4219983.23592.1454490194.264",
            "1.2.840.113619.2.374.2807.4219983.23592.1454490194.261"
        ]
        update_result = update_contour_image_sequence(ds,mpps_entry,sop_instance_uid_list)
        # 判断 update_contour_image_sequence 调用是否成功
        if not update_result.get("success"):
            return json.dumps({
                "success": False,
                "message": f"Failed to update contour image sequence: {update_result.get('message')}",
                "result": ""
            })
        context = build_context(ds.SOPClassUID,ds.file_meta.TransferSyntaxUID)
        # Associate with the peer AE
        assoc = ae.associate(ip, port, contexts=[context], ae_title=pacs_ae_title)
        status = assoc.send_c_store(ds)
        assoc.release()
        if status.Status != 0x0000:
            return json.dumps({
                "success": False,
                "message": f"C-STORE request failed with status {status.Status}",
                "result": ""
            })

        return json.dumps({
            "success": True,
            "message": "C-STORE requests completed successfully.",
            "result": ""
        })

    except Exception as e:
        # Handle any exceptions and return an error response
        print(f"An error occurred: {str(e)}")
        return json.dumps({
            "success": False,
            "message": f"An error occurred: {str(e)}",
            "result": ""
        })
        
        
def update_contour_image_sequence(ds, mpps_entry, sop_instance_uid_list):
    # Ensure SopInstanceUids exists and has elements
    if not mpps_entry.SopInstanceUids:
        return {
            "success": False,
            "message": "No SopInstanceUids data available."
        }

    # Check if there's only one element in SopInstanceUids
    sop_instance_uid_data = mpps_entry.SopInstanceUids[0] if len(mpps_entry.SopInstanceUids) == 1 else None

    # Get contour image sequence from DICOM dataset
    try:
        contour_image_sequence = ds.ReferencedFrameOfReferenceSequence[0].RTReferencedStudySequence[0].RTReferencedSeriesSequence[0].ContourImageSequence
        
        if not contour_image_sequence:
            return {
                "success": False,
                "message": "No ContourImageSequence data available."
            }

        # If SopInstanceUids has only one entry, proceed to update ReferencedSOPInstanceUID
        if sop_instance_uid_data:
            sop_instance_infos = sop_instance_uid_data.sop_instance_infos

            # Check if the lengths of sop_instance_infos and contour_image_sequence are equal
            # if len(sop_instance_infos) != len(contour_image_sequence):
            #     return {
            #         "success": False,
            #         "message": f"Error: The length of sop_instance_infos ({len(sop_instance_infos)}) "
            #     }

            # Iterate over the sop_instance_infos and update or add to contour_image_sequence
            for index, sop_info in enumerate(sop_instance_infos):
                # Check if the ReferencedSOPInstanceUID is in the provided list
                if contour_image_sequence[index].ReferencedSOPInstanceUID in sop_instance_uid_list:
                    # Find the index 'i' in the sop_instance_uid_list
                    i = sop_instance_uid_list.index(contour_image_sequence[index].ReferencedSOPInstanceUID)

                    # Ensure that the ds.ROIContourSequence has enough entries
                    if len(ds.ROIContourSequence) > i:
                        # Replace the ReferencedSOPInstanceUID in the ROIContourSequence
                        ds.ROIContourSequence[i].ContourSequence[0].ContourImageSequence[0].ReferencedSOPInstanceUID = sop_info.sop_instance_uid
                    else:
                        return {
                            "success": False,
                            "message": f"Error: ROIContourSequence does not have enough elements for index {i}."
                        }

                # Replace existing ReferencedSOPInstanceUID in ContourImageSequence
                contour_image_sequence[index].ReferencedSOPInstanceUID = sop_info.sop_instance_uid

            return {
                "success": True,
                "message": "ContourImageSequence and ROIContourSequence updated successfully."
            }
        else:
            return {
                "success": False,
                "message": "More than one SopInstanceUids entry found, skipping SOP Instance replacement."
            }
    except AttributeError as e:
        return {
            "success": False,
            "message": f"An error occurred while accessing ContourImageSequence: {str(e)}"
        }

def send_cstore_headless(mim_entry, mpps_entry, dcmFile, debug=False):
    # 初始化调试记录器（假设有一个调试日志工具）
    debug_logger()
    print(mpps_entry)
    try:
        # 将 JSON 字符串直接转换为字典
        mpps_entry = json.loads(mpps_entry)
        mim_entry = json.loads(mim_entry)

        # 从 mim_entry 字典中获取相应的字段
        calling_ae_title = mim_entry.get("calling_ae_title")
        pacs_ae_title = mim_entry.get("ae_title")
        ip = mim_entry.get("ip")
        port = int(mim_entry.get("port"))

        # Initialise the Application Entity
        ae = AE(ae_title=calling_ae_title)

        # 创建患者数据字典，直接从 mpps_entry 获取字段
        patient_data = {
            "PatientName": mpps_entry.get("patient_name"),
            "PatientID": mpps_entry.get("patient_id"),
            "PatientBirthDate": mpps_entry.get("patient_birth_date"),
            "PatientSex": mpps_entry.get("patient_sex"),
            "StudyInstanceUID": generate_uid(),
            "Description": mpps_entry.get("description"),
            "Generate": mpps_entry.get("generate"),
        }

        def process_directory(directory, series_instance_uid):
            result = None
            for root, dirs, files in os.walk(directory):
                dcm_files = [os.path.join(root, f) for f in files if f.endswith('.dcm')]
                # series_instance_uid = generate_uid()

                for dcm_file in dcm_files:
                    result = process_dicom_file(ae, dcm_file, patient_data, ip, port, pacs_ae_title, series_instance_uid)
                    if not result["success"]:
                        return result
            return result
        if patient_data['Generate']:
            series_instance_uid = generate_uid()
        else:
            series_instance_uid = None
        # 判断 dcmFile 是文件还是目录
        if os.path.isdir(dcmFile):
            for current_dir, _, _ in os.walk(dcmFile):
                result = process_directory(current_dir,series_instance_uid)
                if result and not result["success"]:
                    return json.dumps(result)
                series_instance_uid = result['result']
        else:
            # 如果是单个文件，执行处理
            
            result = process_dicom_file(ae, dcmFile, patient_data, ip, port, pacs_ae_title, mpps_entry.get("sop_instance_uids"))
            series_instance_uid = result['result']
            if not result["success"]:
                return json.dumps(result)

        return json.dumps({
            "success": True,
            "message": "C-STORE requests completed successfully.",
            "result": series_instance_uid
        })

    except Exception as e:
        return json.dumps({
            "success": False,
            "message": f"Error during C-STORE: {str(e)}",
            "result": series_instance_uid
        })

def process_dicom_file(ae, dcm_file, patient_data, ip, port, pacs_ae_title, series_instance_uid):
    try:
        # 读取 DICOM 文件
        ds = dcmread(dcm_file)
        now = datetime.now()
        date_int_str = now.strftime('%Y%m%d')
        time_int_str = now.strftime("%H%M%S")

        # 更新 DICOM 文件中的字段
        ds.InstanceCreationDate = date_int_str
        ds.InstanceCreationTime = time_int_str
        ds.SOPInstanceUID = generate_uid()
        ds.PatientName = patient_data["PatientName"]
        ds.PatientID = patient_data["PatientID"]
        ds.PatientBirthDate = patient_data["PatientBirthDate"]
        ds.PatientSex = patient_data["PatientSex"]
        if patient_data['Generate']:
            ds.StudyInstanceUID = patient_data["StudyInstanceUID"]
            # ds.SeriesInstanceUID = series_instance_uid
            ds.SeriesInstanceUID = generate_uid()
            if 'ReferencedFrameOfReferenceSequence' in ds:
                ds.ReferencedFrameOfReferenceSequence[0].RTReferencedStudySequence[0].RTReferencedSeriesSequence[0].SeriesInstanceUID = series_instance_uid
        else:
            series_instance_uid = ds.SeriesInstanceUID
        if patient_data.get("Description"):
            ds.SeriesDescription = patient_data.get('Description')
        
        # 构建 DICOM 传输上下文
        context = build_context(ds.SOPClassUID,ds.file_meta.TransferSyntaxUID)

        # 关联 AE 并发送 C-STORE 请求
        assoc = ae.associate(ip, port, contexts=[context], ae_title=pacs_ae_title)
        status = assoc.send_c_store(ds)
        assoc.release()

        # 检查 C-STORE 请求状态
        if status.Status != 0x0000:
            return {
                "success": False,
                "message": f"C-STORE request failed with status {status.Status}",
                "result": series_instance_uid
            }

        return {
            "success": True,
            "message": "C-STORE request completed successfully.",
            "result": series_instance_uid
        }

    except Exception as e:
        return {
            "success": False,
            "message": f"Error processing DICOM file {dcm_file}: {str(e)}",
            "result": series_instance_uid
        }