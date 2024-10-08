<template>
  <div class="p-4">
    <!-- 全局 Loading 遮罩 -->
    <div
      v-if="loading"
      class="absolute inset-0 bg-base-100 bg-opacity-75 flex items-center justify-center z-50"
    >
      <button class="btn btn-lg loading">Loading...</button>
    </div>

    <!-- 确认删除模态框 -->
    <div v-if="showConfirmDeleteAll" class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg">Confirm Deletion</h3>
        <p>Are you sure you want to delete all completed entries?</p>
        <div class="modal-action">
          <button class="btn btn-error" @click="confirmDeleteAll">
            Yes, Delete All
          </button>
          <button
            class="btn btn-secondary"
            @click="showConfirmDeleteAll = false"
          >
            Cancel
          </button>
        </div>
      </div>
    </div>

    <div v-if="showDescriptionModal" class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg">Override Series Description</h3>
        <textarea
          v-model="descriptionText"
          class="textarea textarea-bordered w-full"
          placeholder="Enter new series description if you want to override it, or leave empty to use original series description in dicom files "
        />
        <div class="modal-action">
          <button class="btn" @click="submitDescription">Submit</button>
          <button class="btn btn-secondary" @click="closeDescriptionModal">
            Cancel
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

    <!-- Inline Search Form -->
    <form class="flex items-center space-x-2 mb-4">
      <select
        v-model="tempSelectedOption"
        class="select select-bordered w-full max-w-xs"
      >
        <option disabled value="">Select an option</option>
        <option v-for="option in options" :key="option.id" :value="option.id">
          {{ `${option.name} [${option.ip}:${option.port}]` }}
        </option>
      </select>
      <button class="btn btn-primary" @click.prevent="handleSearch">
        Search
      </button>
      <button class="btn btn-outline" @click.prevent="toggleFieldSelection">
        Select Fields
      </button>

      <button
        class="btn btn-error"
        @click.prevent="showConfirmDeleteAll = true"
      >
        Delete All
      </button>
    </form>

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
          <button class="btn" @click.prevent="applyFieldSelection">
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

    <div id="table-container" class="overflow-x-auto" style="overflow-y: auto">
      <table class="table table-xs w-full">
        <thead>
          <tr>
            <th class="sticky top-0 bg-base-100">#</th>
            <th
              v-for="field in selectedFields"
              :key="field"
              class="sticky top-0 bg-base-100"
            >
              {{ allFields[field] }}
            </th>
            <th class="sticky top-0 bg-base-100">Current Status</th>
            <th class="sticky top-0 bg-base-100">Actions</th>
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
            <td>{{ item.status || "IDLE" }}</td>
            <td>
              <!-- 使用 flex 布局来对齐按钮 -->
              <div class="flex items-center space-x-1">
                <button
                  v-if="!item.status || item.status === 'IDLE'"
                  class="btn btn-xs btn-primary"
                  @click="createStatus(item, 'IN PROGRESS')"
                >
                  PROGRESS
                </button>
                <template v-else-if="item.status === 'IN PROGRESS'">
                  <button
                    class="btn btn-xs btn-success"
                    @click="openDescriptionModal(item.id)"
                  >
                    COMPLETED
                  </button>
                  <button
                    class="btn btn-xs btn-error"
                    @click="deletempps(item.id)"
                  >
                    DELETE
                  </button>
                </template>
                <template v-else-if="item.status === 'COMPLETED'">
                  <button
                    class="btn btn-xs btn-warning"
                    @click="showStorePACSModal(item.id, 'COMPLETED')"
                  >
                    SEND PACS
                  </button>
                  <button
                    v-if="item['Modality'] === 'MR'"
                    class="btn btn-xs btn-success"
                    @click="showStorePACSModal(item.id, 'RTSS')"
                  >
                    SEND RT-S
                  </button>
                  <button
                    class="btn btn-xs btn-error"
                    @click="deletempps(item.id)"
                  >
                    DELETE
                  </button>
                </template>
              </div>
            </td>
          </tr>
          <tr v-if="data.length === 0">
            <td :colspan="selectedFields.length + 3" class="text-center">
              No results found
            </td>
          </tr>
        </tbody>
      </table>
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
  </div>
</template>

<script lang="ts">
function adjustTableHeight() {
  var container = document.getElementById("table-container");
  if (container) {
    var height = window.innerHeight - 300; // 计算出要减去的高度
    container.style.maxHeight = height + "px";
  }
}
adjustTableHeight();
// 调用该函数并监听窗口resize事件
window.addEventListener("load", adjustTableHeight);
window.addEventListener("resize", adjustTableHeight);
</script>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useRouter } from "vue-router";
import { open } from "@tauri-apps/plugin-dialog";
const loading = ref(false);

const adjustTableHeight = () => {
  var container = document.getElementById("table-container");
  if (container) {
    var height = window.innerHeight - 300;
    container.style.maxHeight = height + "px";
  }
};
const showConfirmDeleteAll = ref(false);

const confirmDeleteAll = async () => {
  showConfirmDeleteAll.value = false; // 关闭确认模态框
  await deleteAll(); // 调用删除所有的方法
};

const showDescriptionModal = ref(false); // 控制描述弹窗的显示
const descriptionText = ref(""); // 存储输入的描述文本
const descriptionId = ref(""); // 存储输入的描述文本

// 打开描述弹窗
const openDescriptionModal = (id: { tb: string; id: { String: string } }) => {
  descriptionText.value = ""; // 重置描述文本
  showDescriptionModal.value = true;
  descriptionId.value = id.id.String;
};

// 关闭描述弹窗
const closeDescriptionModal = () => {
  showDescriptionModal.value = false;
};

const submitDescription = () => {
  closeDescriptionModal();
  updateStatus(descriptionId.value, "COMPLETED");
};

const openFileSelector = async () => {
  try {
    // 打开文件选择器对话框
    const file = await open({
      multiple: false,
      directory: true,
      defaultPath: dicomDir.value,
    });

    if (file) {
      console.log("Selected file: ", file);
      return file;
    } else {
      return false;
    }
  } catch (error) {
    showError("Failed to open file selector", error);
    return false;
  }
};

const openFileSelectorFile = async () => {
  try {
    // 打开文件选择器对话框
    const file = await open({
      multiple: false, // 是否允许多选
      directory: false, // 是否选择目录，如果选择文件，设为 false
      defaultPath: dicomDir.value,
    });

    if (file) {
      console.log("Selected file: ", file);
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

// 定义响应式数据
const tempSelectedOption = ref<string | null>(null);
const selectedOption = ref<string | null>(null);
const data = ref<any[]>([]);
const options = ref<{ id: string; name: string; ip: string; port: string }[]>(
  [],
);
const showFieldSelection = ref(false);
const tempSelectedFields = ref<string[]>([]);
const showPACSModal = ref(false); // 控制 STORE PACS 模态框显示
const mimEntries = ref<any[]>([]); // 存储 MimEntry 数据
const selectedFields = ref<string[]>([]);
const dicomDir = ref<string | null>(null);
const currentId = ref<string | null>(null); // 用于存储当前触发弹框的 ID
const currentOption = ref<string | null>(null); // 用于存储当前触发弹框的 ID
const showErrorModal = ref(false);
const errorMessage = ref("");

// 显示错误弹框的函数
const showError = (message: string, error) => {
  errorMessage.value = message + error;
  showErrorModal.value = true;
};
const allFields = {
  AccessionNumber: "Accession Number",
  RequestedProcedureDescription: "Procedure Description",
  PatientName: "Patient Name",
  PatientID: "Patient ID",
  PatientBirthDate: "Birth Date",
  PatientSex: "Sex",
  Modality: "Modality",
  ScheduledStationAETitle: "Station AE Title",
  ScheduledProcedureStepStartDate: "Procedure Step Start Date",
  ScheduledPerformingPhysicianName: "Physician Name",
  StudyInstanceUID: "Study Instance UID",
};

// 页面加载时获取选项数据
onMounted(async () => {
  window.addEventListener("load", adjustTableHeight);
  window.addEventListener("resize", adjustTableHeight);
  adjustTableHeight();
  await fetchOptions();
  await getDicomDir();
});

// 获取选项数据并格式化为 options
const fetchOptions = async () => {
  try {
    const result = await invoke<ApiResponse<WorklistEntry[]>>(
      "read_worklist_entry",
      { id: null },
    );
    if (result.success) {
      options.value =
        result.data?.map((entry) => ({
          id: entry.id.id.String,
          name: entry.name,
          ip: entry.worklist_ip,
          port: entry.worklist_port,
        })) || [];
      if (options.value.length > 0) {
        tempSelectedOption.value = options.value[0].id;
        handleSearch();
      }
    } else {
      showError(result.message, "");
    }
  } catch (error) {
    showError("Failed to fetch options:", error);
  }
};

const getDicomDir = async () => {
  try {
    const result = await invoke<string>("get_base_dicom_dir");
    dicomDir.value = result;
  } catch (error) {
    showError("Failed to get DICOM directory:", error);
  }
};

// 搜索方法，调用 Tauri 后端命令
const handleSearch = async () => {
  selectedOption.value = tempSelectedOption.value;
  if (!selectedOption.value) {
    return;
  }

  data.value = [];
  loading.value = true;
  try {
    if (selectedOption.value) {
      const result = await invoke<string>("search_worklist_data", {
        id: selectedOption.value,
      });
      const parsedData = JSON.parse(result);
      // searchStatus
      const resultStatus = await invoke<ApiResponse<MppsRecordEntry[]>>(
        "read_mpps_entry",
        {
          id: undefined,
        },
      );

      if (resultStatus.success) {
        // 遍历 resultStatus.data 数组，并处理每个 MppsRecordEntry
        // resultStatus

        // 对 resultStatus 按照 AccessionNumber 升序排序
        resultStatus.data.sort((a, b) => {
          // 使用 parseInt 确保按数值大小排序
          return parseInt(a.AccessionNumber) - parseInt(b.AccessionNumber);
        });
        resultStatus.data.forEach((mppsData) => {
          // 解构对象，获取除 id 外的其他字段
          const {
            // id, // 跳过 id 字段
            ...otherFields
          } = mppsData;

          // 如果 parsedData 是数组，则将 otherFields 添加到数组
          if (Array.isArray(parsedData)) {
            parsedData.push(otherFields);
          } else {
            // 如果 parsedData 不是数组，直接合并字段到 parsedData 对象中
            Object.assign(parsedData, otherFields);
          }
        });
        // 更新 data
        data.value = parsedData;
      } else {
        // showError(resultStatus.message, "");
        data.value = parsedData;
      }
    }
  } catch (error) {
    showError("Failed to invoke search_data command:", error);
  } finally {
    loading.value = false;
  }
};

// 处理字段选择逻辑
const toggleFieldSelection = () => {
  tempSelectedFields.value = [...selectedFields.value];
  showFieldSelection.value = !showFieldSelection.value;
};

const deleteAll = async () => {
  // 删除所有的已完成的数据
  loading.value = true;
  try {
    await invoke("delete_mpps_entry", {
      id: "all",
    });
    // 重新加载数据
    handleSearch();
  } catch (error) {
    showError("Failed to update status:", error);
  } finally {
    loading.value = false;
  }
};

// 处理 STORE PACS 模态框显示
const showStorePACSModal = async (
  id: {
    tb: string;
    id: { String: string };
  },
  option: string,
) => {
  currentId.value = id.id.String;
  currentOption.value = option;
  await fetchMimEntries();
  togglePACSModal();
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

// 切换 PACS 模态框
const togglePACSModal = () => {
  showPACSModal.value = !showPACSModal.value;
};

// 选择 PACS 项目后的处理逻辑
const selectPACS = (id: { tb: string; id: { String: string } }) => {
  console.log("Selected PACS:", id);
  if (currentOption.value == "COMPLETED") {
    sendToPACS(currentId.value, id.id.String);
  } else if (currentOption.value == "RTSS") {
    sendRTS(currentId.value, id.id.String);
  }
  togglePACSModal(); // 关闭模态框
};

const applyFieldSelection = () => {
  if (tempSelectedFields.value.length === 0) {
    selectedFields.value = Object.keys(allFields);
  } else {
    selectedFields.value = [...tempSelectedFields.value];
  }
  toggleFieldSelection();
};

const router = useRouter();

const go_setting = () => {
  router.push({ path: "/dicom/setting" });
};

// 更新状态方法
const updateStatus = async (id: string, newStatus) => {
  const file = await openFileSelector();
  if (!file) {
    return;
  }
  loading.value = true;
  try {
    await invoke("update_mpps_entry", {
      selectedId: selectedOption.value,
      dcmFile: file,
      id: id,
      status: newStatus,
      description: descriptionText.value,
    });
    handleSearch();
  } catch (error) {
    console.log(error);
    showError("Failed to update status:", error);
  } finally {
    loading.value = false;
  }
};

// 更新状态方法
const sendRTS = async (id: string, mimId: string) => {
  const file = await openFileSelectorFile();
  if (!file) {
    return;
  }
  loading.value = true;
  try {
    await invoke("send_rt_s", {
      selectedId: mimId,
      dcmFile: file,
      id: id,
    });
    handleSearch();
  } catch (error) {
    showError("Failed to update status:", error);
  } finally {
    loading.value = false;
  }
};
// 更新状态方法
const createStatus = async (item, newStatus) => {
  loading.value = true;
  try {
    await invoke("create_mpps_entry", {
      selectedId: selectedOption.value,
      entry: {
        ...item,
        status: "IN PROGRESS",
      },
    });
    // 重新加载数据
    handleSearch();
  } catch (error) {
    showError("Failed to update status:", error);
  } finally {
    loading.value = false;
  }
};

// 更新状态方法
const deletempps = async (id: { tb: string; id: { String: string } }) => {
  loading.value = true;
  try {
    await invoke("delete_mpps_entry", {
      id: id.id.String,
    });
    // 重新加载数据
    handleSearch();
  } catch (error) {
    showError("Failed to update status:", error);
  } finally {
    loading.value = false;
  }
};

// 发送到PACS的方法
const sendToPACS = async (currentId, id) => {
  loading.value = true;
  try {
    const pacs_result = await invoke("send_to_pacs", {
      id: id,
      currentId: currentId,
    });
    console.log(pacs_result);
    handleSearch();
  } catch (error) {
    showError("Failed to send to PACS:", error);
  } finally {
    loading.value = false;
  }
};

// 页面加载时默认选择字段
selectedFields.value = Object.keys(allFields).filter(
  (field) =>
    ![
      "ScheduledPerformingPhysicianName",
      "StudyInstanceUID",
      "RequestedProcedureDescription",
    ].includes(field),
);
</script>
