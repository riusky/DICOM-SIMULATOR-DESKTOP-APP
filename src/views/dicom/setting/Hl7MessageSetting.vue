<template>
  <div class="p-4">
    <!-- Inline Search Form -->
    <form class="flex items-center space-x-2 mb-4">
      <button class="btn btn-primary" @click.prevent="handleSearch">
        {{ t("from.search") }}
      </button>
      <button class="btn btn-secondary" @click.prevent="toggleAddModal">
        {{ t("from.add") }}
      </button>
      <button class="btn btn-outline" @click.prevent="toggleFieldSelection">
        {{ t("from.showfields") }}
      </button>
    </form>

    <!-- Add Data Modal -->
    <div
      v-if="showAddModal"
      class="modal modal-open"
      @click.self="toggleAddModal"
    >
      <div class="modal-box max-w-3xl">
        <h3 class="font-bold text-lg">{{ t("from.AddHL7Message") }}</h3>
        <form @submit.prevent="handleAdd">
          <!-- Form fields for creating new entry -->
          <div class="form-control flex flex-row mb-1 items-center">
            <label class="label w-1/5">{{ t("from.hl7name") }}</label>
            <input
              v-model="newEntry.name"
              type="text"
              class="input input-bordered w-full"
              required
            />
          </div>
          <div class="form-control flex flex-row mb-1 items-center">
            <label class="label w-1/5">{{ t("from.hl7message") }}</label>
            <textarea
              v-model="newEntry.message"
              class="textarea textarea-bordered w-full"
              required
            />
          </div>

          <!-- Modal Actions -->
          <div class="modal-action">
            <button type="submit" class="btn btn-primary">
              {{ t("from.Submit") }}
            </button>
            <button
              type="button"
              class="btn btn-secondary"
              @click.prevent="toggleAddModal"
            >
              {{ t("from.Close") }}
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- Field Selection Modal -->
    <div v-if="showFieldSelection" class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg">{{ t("from.Display") }}</h3>
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
            {{ t("from.Apply") }}
          </button>
          <button
            class="btn btn-secondary"
            @click.prevent="toggleFieldSelection"
          >
            {{ t("from.Close") }}
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
          <th>{{ t("from.Actions") }}</th>
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
            <button class="btn btn-sm btn-warning" @click="handleUpdate(item)">
              {{ t("from.Update") }}
            </button>
            <button
              class="btn btn-sm btn-danger"
              @click="handleDelete(item.id)"
            >
              {{ t("from.Delete") }}
            </button>
          </td>
        </tr>
        <tr v-if="data.length === 0">
          <td :colspan="selectedFields.length + 2" class="text-center">
            {{ t("from.results") }}
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useTranslationLang } from "@/layout/hooks/useTranslationLang";
const { t } = useTranslationLang();
// 定义 HL7MessageSetting 的接口
interface HL7MessageSetting {
  id?: {
    tb: string;
    id: {
      String: string;
    };
  };
  name: string;
  message: string;
}

const data = ref<HL7MessageSetting[]>([]);
const showFieldSelection = ref(false);
const showAddModal = ref(false);

const newEntry = ref<HL7MessageSetting>({
  id: undefined,
  name: "",
  message: "",
});

// 临时存储选中字段
const tempSelectedFields = ref<string[]>([]);

const allFields = {
  name: "Name",
  message: "HL7 Message",
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
    const result = await invoke<ApiResponse<HL7MessageSetting[]>>(
      "read_hl7_message_setting",
      {
        id: null,
      },
    );
    if (result.success) {
      data.value = result.data || [];
    } else {
      console.error(result.message);
    }
  } catch (error) {
    console.error("Failed to fetch HL7 message settings:", error);
  }
};

// 新增或更新条目的方法
const handleAdd = async () => {
  try {
    if (newEntry.value.id) {
      const result = await invoke<ApiResponse<HL7MessageSetting>>(
        "update_hl7_message_setting",
        {
          id: newEntry.value.id.id.String,
          updatedEntry: {
            name: newEntry.value.name,
            message: newEntry.value.message,
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
      const result = await invoke<ApiResponse<HL7MessageSetting>>(
        "create_hl7_message_setting",
        {
          entry: newEntry.value,
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
    console.error("Failed to add or update HL7 message setting:", error);
  }
};

// 更新条目的方法
const handleUpdate = (item: HL7MessageSetting) => {
  newEntry.value = { ...item };
  toggleAddModal();
};

// 删除条目的方法
const handleDelete = async (id: { tb: string; id: { String: string } }) => {
  try {
    const result = await invoke<ApiResponse<null>>(
      "delete_hl7_message_setting",
      {
        id: id.id.String,
      },
    );

    if (result.success) {
      handleSearch();
    } else {
      console.error(result.message);
    }
  } catch (error) {
    console.error("Failed to delete HL7 message setting:", error);
  }
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
    name: "",
    message: "",
  };
};

// 页面加载时默认选择字段
selectedFields.value = ["name", "message"];
handleSearch();
</script>
