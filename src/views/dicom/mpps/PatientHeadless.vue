<template>
  <!-- 全局 Loading 遮罩 -->
  <div
    v-if="loading"
    class="absolute inset-0 bg-white bg-opacity-75 flex items-center justify-center z-50"
  >
    <button class="btn btn-lg loading">Loading...</button>
  </div>

  <div v-if="showDescriptionModal" class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg">Override Series Description</h3>
      <textarea
        v-model="descriptionText"
        class="textarea textarea-bordered w-full"
        placeholder="Enter new series description if you want to override it, or leave empty to use original series description in dicom files "
      />
      <div class="flex items-center mt-4">
        <span class="mr-2">Generate SeriesInstanceUID</span>
        <input
          v-model="generateUID"
          type="checkbox"
          class="toggle toggle-primary"
        />
      </div>
      <div class="modal-action">
        <button class="btn" @click="submitDescription">Submit</button>
        <button class="btn btn-secondary" @click="closeDescriptionModal">
          Cancel
        </button>
      </div>
    </div>
  </div>

  <!-- Store PACS Modal -->
  <div v-if="showPACSModal" class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg">Select PACS</h3>
      <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4">
        <div
          v-for="mim in mimEntries"
          :key="mim.id"
          class="border rounded p-2 cursor-pointer transition duration-200 ease-in-out transform hover:scale-105 hover:bg-base-100 hover:border-green-500 hover:shadow-lg"
          @click="selectPACS(mim.id)"
        >
          {{ mim.name }}
        </div>
      </div>
      <div class="modal-action">
        <button class="btn btn-secondary" @click="togglePACSModal">
          Close
        </button>
      </div>
    </div>
  </div>
  <!-- Error Modal -->
  <div v-if="showErrorModal" class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg text-red-600">Error</h3>
      <p>{{ errorMessage }}</p>
      <div class="modal-action">
        <button class="btn" @click="showErrorModal = false">Close</button>
      </div>
    </div>
  </div>
  <div class="p-4">
    <!-- Inline Search Form -->
    <form class="flex items-center space-x-2 mb-4">
      <button class="btn btn-primary" @click.prevent="handleSearch">
        Search
      </button>
      <button class="btn btn-secondary" @click.prevent="toggleAddModal">
        Add
      </button>
      <button class="btn btn-outline" @click.prevent="toggleFieldSelection">
        Select Fields
      </button>
    </form>

    <!-- Add Data Modal -->
    <div v-if="showAddModal" class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg">Add New Patient Entry</h3>
        <form @submit.prevent="handleAdd">
          <!-- Form fields for creating new entry -->
          <div class="form-control">
            <label class="label">Patient Name</label>
            <input
              v-model="newEntry.patient_name"
              type="text"
              class="input input-bordered"
              required
            />
          </div>
          <div class="form-control">
            <label class="label">Patient ID</label>
            <input
              v-model="newEntry.patient_id"
              type="text"
              class="input input-bordered"
              required
            />
          </div>
          <div class="form-control">
            <label class="label">Patient Birth Date</label>
            <input
              v-model="newEntry.patient_birth_date"
              type="date"
              class="input input-bordered"
              required
            />
          </div>
          <div class="form-control">
            <label class="label">Patient Sex</label>
            <div class="flex space-x-4">
              <label class="cursor-pointer flex items-center">
                <input
                  v-model="newEntry.patient_sex"
                  type="radio"
                  value="M"
                  class="radio radio-primary"
                />
                <span class="ml-2">MAN</span>
              </label>
              <label class="cursor-pointer flex items-center">
                <input
                  v-model="newEntry.patient_sex"
                  type="radio"
                  value="F"
                  class="radio radio-primary"
                />
                <span class="ml-2">FEMALE</span>
              </label>
            </div>
          </div>

          <!-- Modal Actions -->
          <div class="modal-action">
            <button type="submit" class="btn btn-primary">Submit</button>
            <button
              type="button"
              class="btn btn-secondary"
              @click.prevent="toggleAddModal"
            >
              Close
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- Field Selection Modal -->
    <div v-if="showFieldSelection" class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg">Select Fields to Display</h3>
        <div class="form-control">
          <label
            v-for="(label, field) in allFields"
            :key="field"
            class="cursor-pointer label"
          >
            <span class="label-text">{{ label }}</span>
            <input
              v-model="tempSelectedFields"
              type="checkbox"
              class="checkbox"
              :value="field"
            />
          </label>
        </div>
        <div class="modal-action">
          <button class="btn btn-primary" @click.prevent="applyFieldSelection">
            Apply
          </button>
          <button
            class="btn btn-secondary"
            @click.prevent="toggleFieldSelection"
          >
            Close
          </button>
        </div>
      </div>
    </div>

    <!-- Data Table -->
    <table class="table table-xs w-full">
      <thead>
        <tr>
          <th>#</th>
          <th v-for="field in selectedFields" :key="field">
            {{ allFields[field] }}
          </th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="(item, index) in data"
          :key="index"
          class="hover:bg-base-100"
        >
          <td>{{ index + 1 }}</td>
          <td v-for="field in selectedFields" :key="field">
            {{ item[field] }}
          </td>
          <td>
            <button
              class="btn btn-sm btn-warning mr-1"
              @click="handleUpdate(item)"
            >
              Update
            </button>
            <button
              class="btn btn-sm btn-danger mr-1"
              @click="handleDelete(item.id)"
            >
              Delete
            </button>
            <button
              class="btn btn-sm btn-info mr-1"
              @click="openDescriptionModal(item, 'dir')"
            >
              Send Directory
            </button>
            <button
              class="btn btn-sm btn-success mr-1"
              @click="openDescriptionModal(item, 'file')"
            >
              Send File
            </button>
          </td>
        </tr>
        <tr v-if="data.length === 0">
          <td :colspan="selectedFields.length + 2" class="text-center">
            No results found
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
const dicomDir = ref<string | null>(null);
const showPACSModal = ref(false); // 控制 STORE PACS 模态框显示
const mimEntries = ref<any[]>([]); // 存储 MimEntry 数据
const currentId = ref<string | null>(null); // 用于存储当前触发弹框的 ID
const currentDir = ref<string | null>(null); // 用于存储当前触发弹框的 ID
const loading = ref(false);
const showDescriptionModal = ref(false); // 控制描述弹窗的显示
const descriptionText = ref(""); // 存储输入的描述文本
const descriptionId = ref(""); // 存储输入的描述文本
const descriptionOption = ref(""); // 存储输入的描述文本
const generateUID = ref(false);

// 打开描述弹窗
const openDescriptionModal = (item: PatientEntry, option: string) => {
  descriptionText.value = ""; // 重置描述文本
  showDescriptionModal.value = true;
  descriptionId.value = item.id.id.String;
  descriptionOption.value = option;
};

// 关闭描述弹窗
const closeDescriptionModal = () => {
  showDescriptionModal.value = false;
};

const submitDescription = () => {
  closeDescriptionModal();
  if (descriptionOption.value == "dir") {
    sendDirectory();
  } else {
    sendFile();
  }
};

// 选择 PACS 项目后的处理逻辑
const selectPACS = async (id: { tb: string; id: { String: string } }) => {
  console.log("Selected PACS:", id);
  togglePACSModal(); // 关闭模态框
  loading.value = true;
  console.log({
    dcmFile: currentDir.value,
    id: currentId.value,
    selectedId: id.id.String,
    description: descriptionText.value,
    generate: generateUID.value,
  });
  try {
    await invoke("send_cstore_headless", {
      dcmFile: currentDir.value,
      id: currentId.value,
      selectedId: id.id.String,
      description: descriptionText.value,
      generate: generateUID.value,
    });
  } catch (error) {
    console.log(error);
    showError("Failed to update status:", error);
  } finally {
    loading.value = false;
  }
};

interface MimEntry {
  id?: {
    tb: string;
    id: {
      String: string;
    };
  };
  name: string;
  calling_ae_title: string;
  ae_title: string;
  ip: string;
  port: string;
}

// 获取 MimEntry 数据
const fetchMimEntries = async () => {
  try {
    const result = await invoke<ApiResponse<MimEntry[]>>("read_mim_entry", {
      id: null,
    });
    if (result.success) {
      mimEntries.value = result.data || [];
    } else {
      showError(result.message, "");
    }
  } catch (error) {
    showError("Failed to fetch mim entries:", error);
  }
};

// 页面加载时获取选项数据
onMounted(async () => {
  await getDicomDir();
});

const getDicomDir = async () => {
  try {
    const result = await invoke<string>("get_base_dicom_dir");
    dicomDir.value = result;
  } catch (error) {
    showError("Failed to get DICOM directory:", error);
  }
};

const showErrorModal = ref(false);
const errorMessage = ref("");

// 显示错误弹框的函数
const showError = (message: string, error) => {
  errorMessage.value = message + error;
  showErrorModal.value = true;
};

const openFileSelector = async (directory) => {
  try {
    // 打开文件选择器对话框
    const file = await open({
      multiple: false, // 是否允许多选
      directory: directory, // 是否选择目录，如果选择文件，设为 false
      defaultPath: dicomDir.value,
    });

    if (file) {
      // 在这里处理选中的文件路径
      return file;
    } else {
      return false;
    }
  } catch (error) {
    showError("Failed to open file selector", error);
    return false;
  }
};

interface PatientEntry {
  id?: {
    tb: string;
    id: {
      String: string;
    };
  };
  patient_name: string;
  patient_id: string;
  patient_birth_date: string;
  patient_sex: string;
}

const data = ref<PatientEntry[]>([]);
const showFieldSelection = ref(false);
const showAddModal = ref(false);

const newEntry = ref<PatientEntry>({
  id: undefined,
  patient_name: "",
  patient_id: "",
  patient_birth_date: "",
  patient_sex: "M", // 默认值设为 "M"
});

// 临时存储选中字段
const tempSelectedFields = ref<string[]>([]);

const allFields = {
  id: "ID",
  patient_name: "Patient Name",
  patient_id: "Patient ID",
  patient_birth_date: "Patient Birth Date",
  patient_sex: "Patient Sex",
};

// 处理字段选择
const selectedFields = ref<string[]>(Object.keys(allFields));

// 打开或关闭字段选择模态框
const toggleFieldSelection = () => {
  tempSelectedFields.value = [...selectedFields.value];
  showFieldSelection.value = !showFieldSelection.value;
};

// 应用字段选择
const applyFieldSelection = () => {
  selectedFields.value = [...tempSelectedFields.value];
  toggleFieldSelection();
};

// 搜索方法，调用 Tauri 后端命令
const handleSearch = async () => {
  data.value = [];
  try {
    const result = await invoke<ApiResponse<PatientEntry[]>>(
      "read_patient_entry",
      { id: null },
    );
    if (result.success) {
      data.value = result.data || [];
    } else {
      console.error(result.message);
    }
  } catch (error) {
    console.error("Failed to fetch patient entries:", error);
  }
};

// 新增或更新条目的方法
const handleAdd = async () => {
  try {
    if (newEntry.value.id) {
      const result = await invoke<ApiResponse<PatientEntry>>(
        "update_patient_entry",
        {
          id: newEntry.value.id.id.String,
          updatedEntry: {
            patient_name: newEntry.value.patient_name,
            patient_id: newEntry.value.patient_id,
            patient_birth_date: newEntry.value.patient_birth_date,
            patient_sex: newEntry.value.patient_sex,
          },
        },
      );
      if (result.success) {
        clearForm();
        toggleAddModal();
        handleSearch();
      } else {
        console.error(result.message);
      }
    } else {
      const result = await invoke<ApiResponse<PatientEntry>>(
        "create_patient_entry",
        {
          entry: {
            patient_name: newEntry.value.patient_name,
            patient_id: newEntry.value.patient_id,
            patient_birth_date: newEntry.value.patient_birth_date,
            patient_sex: newEntry.value.patient_sex,
          },
        },
      );
      if (result.success) {
        clearForm();
        toggleAddModal();
        handleSearch();
      } else {
        console.error(result.message);
      }
    }
  } catch (error) {
    console.error("Failed to add or update patient entry:", error);
  }
};

// 更新条目的方法
const handleUpdate = (item: PatientEntry) => {
  newEntry.value = { ...item };
  toggleAddModal();
};

// 删除条目的方法
const handleDelete = async (id: { tb: string; id: { String: string } }) => {
  try {
    const result = await invoke<ApiResponse<null>>("delete_patient_entry", {
      id: id.id.String,
    });

    if (result.success) {
      handleSearch();
    } else {
      console.error(result.message);
    }
  } catch (error) {
    console.error("Failed to delete patient entry:", error);
  }
};

// 发送目录的逻辑
const sendDirectory = async () => {
  const dir = await openFileSelector(true);
  // 选择PACS
  if (dir) {
    currentDir.value = dir;
  } else {
    return;
  }
  currentId.value = descriptionId.value;
  await fetchMimEntries();
  togglePACSModal();
};

// 切换 PACS 模态框
const togglePACSModal = () => {
  showPACSModal.value = !showPACSModal.value;
};

// 发送文件的逻辑
const sendFile = async () => {
  const dir = await openFileSelector(false);
  // 选择PACS
  if (dir) {
    currentDir.value = dir;
  } else {
    return;
  }
  currentId.value = descriptionId.value;
  await fetchMimEntries();
  togglePACSModal();
};

// 处理添加模态框显示，同时清空表单
const toggleAddModal = () => {
  showAddModal.value = !showAddModal.value;
  if (!showAddModal.value) clearForm();
};

// 清空表单
const clearForm = () => {
  newEntry.value = {
    id: undefined,
    patient_name: "",
    patient_id: "",
    patient_birth_date: "",
    patient_sex: "M", // 默认值设为 "M"
  };
};

handleSearch();

// 页面加载时默认选择字段
selectedFields.value = Object.keys(allFields).filter(
  (field) => !["id"].includes(field),
);
</script>
