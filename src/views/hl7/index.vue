<template>
  <div class="p-6 max-w-4xl mx-auto bg-base-100 rounded-xl shadow-md space-y-4">
    <form @submit.prevent="handleSubmit">
      <!-- 下拉选择 -->
      <div class="form-control">
        <label class="label">
          <span class="label-text">Select Protocol</span>
        </label>
        <select
          v-model="selectedProtocol"
          class="select select-bordered w-full"
        >
          <option disabled value="">Choose a protocol</option>
          <option
            v-for="protocol in protocols"
            :key="protocol.id"
            :value="protocol"
          >
            {{ formatProtocol(protocol) }}
          </option>
        </select>
      </div>

      <!-- 单选按钮: TCP/HTTP -->
      <div class="form-control">
        <label class="label">
          <span class="label-text">Connection Type</span>
        </label>
        <label class="cursor-pointer flex items-center space-x-4">
          <input
            v-model="connectionType"
            type="radio"
            name="connection"
            value="TCP"
            class="radio"
          />
          <span>TCP</span>
        </label>
        <label class="cursor-pointer flex items-center space-x-4">
          <input
            v-model="connectionType"
            type="radio"
            name="connection"
            value="HTTP"
            class="radio"
          />
          <span>HTTP</span>
        </label>
      </div>

      <!-- HL7 报文多行输入框和右侧下拉选择 -->
      <div class="form-control flex flex-row space-x-4">
        <div class="w-3/4">
          <label class="label">
            <span class="label-text">HL7 Message</span>
          </label>
          <textarea
            v-model="hl7Message"
            class="textarea textarea-bordered w-full h-24"
            placeholder="Enter HL7 message"
          />
        </div>
        <div class="w-1/4">
          <label class="label">
            <span class="label-text">Select HL7 Message</span>
          </label>
          <select
            v-model="selectedHL7Message"
            class="select select-bordered w-full"
          >
            <option disabled value="">Select a saved message</option>
            <option
              v-for="message in hl7Messages"
              :key="message.id"
              :value="message.message"
            >
              {{ message.name }}
            </option>
          </select>
        </div>
      </div>

      <!-- 提交按钮 -->
      <div class="form-control mt-4">
        <button type="submit" class="btn btn-primary w-full">Submit</button>
      </div>

      <!-- 响应结果的多行输入框 -->
      <div class="form-control mt-4">
        <label class="label">
          <span class="label-text">Response</span>
        </label>
        <textarea
          v-model="responseMessage"
          class="textarea textarea-bordered h-24"
          placeholder="Response will be displayed here"
          readonly
        />
      </div>
    </form>
  </div>
</template>

<script setup>
import { ref, onMounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";

const selectedProtocol = ref(""); // 下拉选择的协议
const connectionType = ref("TCP"); // 单选按钮的连接类型
const hl7Message = ref(""); // HL7 报文
const responseMessage = ref(""); // 响应信息
const protocols = ref([]); // 存储从后端获取的协议
const hl7Messages = ref([]); // 存储从后端获取的 HL7 消息
const selectedHL7Message = ref(""); // 选择的 HL7 消息

// 页面加载时调用获取数据
onMounted(async () => {
  try {
    // 获取协议数据
    const protocolResult = await invoke("read_hl7_setting_entry", { id: null });

    if (protocolResult && protocolResult.success) {
      protocols.value = protocolResult.data;
    } else {
      console.error("Failed to load protocols:", protocolResult.message);
    }

    // 获取 HL7 消息数据
    const hl7MessageResult = await invoke("read_hl7_message_setting", {
      id: null,
    });

    if (hl7MessageResult && hl7MessageResult.success) {
      hl7Messages.value = hl7MessageResult.data;
    } else {
      console.error("Failed to load HL7 messages:", hl7MessageResult.message);
    }
  } catch (error) {
    console.error("Error fetching data:", error);
  }
});

// 格式化协议的展示方式为 "name[ip:port]"
const formatProtocol = (protocol) => {
  return `${protocol.name} [${protocol.ip}:${protocol.port}]`;
};

// 监听选中的 HL7 消息并将其填入 HL7 报文
watch(selectedHL7Message, (newMessage) => {
  hl7Message.value = newMessage; // 当选择新消息时，更新 HL7 报文
});

// 处理表单提交
const handleSubmit = async () => {
  // 提交时准备好参数
  const formData = {
    message: hl7Message.value,
    server_address: selectedProtocol.value.ip, // 从 selectedProtocol 中提取 IP
    port: selectedProtocol.value.port, // 从 selectedProtocol 中提取端口
    method: connectionType.value, // TCP 或 HTTP
  };

  try {
    // 调用 tauri 后端的 send_hl7_message 方法
    const result = await invoke("send_hl7_message", {
      message: formData.message,
      serverAddress: formData.server_address,
      port: formData.port,
      method: formData.method,
    });

    // 处理响应
    responseMessage.value = result;
    console.log("Form Submitted:", formData);
  } catch (error) {
    // 处理错误
    responseMessage.value = `Failed to send message: ${error.message}`;
    console.error("Error:", error);
  }
};
</script>
