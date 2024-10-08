<template>
  <div class="p-4">
    <!-- Inline Search Form -->
    <form class="flex items-center space-x-2 mb-4">
      <select
        v-model="tempSelectedOption"
        class="select select-bordered w-full max-w-xs"
      >
        <option disabled value="">Select an option</option>
        <option v-for="option in options" :key="option.id" :value="option.id">
          {{ option.name }}
        </option>
      </select>
      <button class="btn btn-primary" @click.prevent="handleSearch">
        Search
      </button>
      <!-- <button class="btn btn-secondary" @click.prevent="go_setting">
        Settings
      </button> -->
      <button class="btn btn-outline" @click.prevent="toggleFieldSelection">
        Select Fields
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

    <!-- Data Table -->
    <table class="table table-xs w-full">
      <thead>
        <tr>
          <th>#</th>
          <th v-for="field in selectedFields" :key="field">
            {{ allFields[field] }}
          </th>
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
        </tr>
        <tr v-if="data.length === 0">
          <td :colspan="selectedFields.length + 1" class="text-center">
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
import { useRouter } from "vue-router";

// 定义响应式数据
const tempSelectedOption = ref<string | null>(null); // 用于存储选择的id
const selectedOption = ref<string | null>(null); // 实际用于搜索的id
const data = ref<any[]>([]);
const options = ref<{ id: string; name: string }[]>([]); // 下拉选项的数组
const showFieldSelection = ref(false);

// 临时存储选中的字段
const tempSelectedFields = ref<string[]>([]);

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

const selectedFields = ref<string[]>([]);

// 页面加载时获取选项数据
onMounted(async () => {
  await fetchOptions(); // 页面加载时获取数据
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
          id: entry.id.id.String, // 假设数据有类似的结构
          name: entry.name,
        })) || [];
    } else {
      console.error(result.message);
    }
  } catch (error) {
    console.error("Failed to fetch options:", error);
  }
};

// 搜索方法，调用 Tauri 后端命令
const handleSearch = async () => {
  selectedOption.value = tempSelectedOption.value;
  if (!selectedOption.value) {
    return;
  }
  // 清空当前数据
  data.value = [];

  try {
    if (selectedOption.value) {
      // 调用 Tauri 命令，并传递所选项的id
      const result = await invoke<string>("search_worklist_data", {
        id: selectedOption.value,
      });

      // 解析返回的 JSON 字符串
      const parsedData = JSON.parse(result);
      console.log(parsedData);

      // 将解析后的数据放入 data 中
      data.value = parsedData;
    }
  } catch (error) {
    console.error("Failed to invoke search_data command:", error);
    // 这里你可以添加错误处理逻辑，比如显示错误消息
  }
};

// 处理字段选择逻辑
const toggleFieldSelection = () => {
  // 打开模态框时，临时存储当前的字段选择
  tempSelectedFields.value = [...selectedFields.value];
  showFieldSelection.value = !showFieldSelection.value;
};

const applyFieldSelection = () => {
  if (tempSelectedFields.value.length === 0) {
    // 默认选择所有字段
    selectedFields.value = Object.keys(allFields);
  } else {
    selectedFields.value = [...tempSelectedFields.value]; // 应用字段选择
  }
  toggleFieldSelection();
};

const router = useRouter();

const go_setting = () => {
  router.push({ path: "/dicom/setting" });
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
