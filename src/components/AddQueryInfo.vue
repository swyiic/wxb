<template>
  <a-modal
    v-show="showAddQueryInfo"
    title="🔎添加爱企查信息🔍"
    :visible="AddQueryInfoisible"
    :confirm-loading="confirmLoading"
    @cancel="handleCancel"
    :footer="null"
  >
    <a-form :model="modelRef">
      <a-form-item v-bind="validateInfos.aqc_cookie">
        <label>😬爱企查Cookie</label>
        <a-textarea
          placeholder="Step1.登录百度爱企查
                        Step2.打开开发者工具，点击任意功能
                        Step3.选择任意ajax请求后，复制全部Cookies
                        "
          v-model:value="modelRef.aqc_cookie"
          :rows="10"
          allow-clear
        />
      </a-form-item>
      <div style="text-align: right; margin-top: 16px">
        <a-button @click="handleCancel" style="margin-right: 8px"
          >取消</a-button
        >
        <a-button type="primary" @click="handleSave">保存</a-button>
      </div>
    </a-form>
  </a-modal>
</template>

<script setup lang="ts">
import { ref, onMounted, reactive } from "vue";
import { notification, Form } from "ant-design-vue";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
// import { readText } from "@tauri-apps/plugin-clipboard-manager";

// 定义响应式变量
const showAddQueryInfo = ref(false);
const AddQueryInfoisible = ref(false);
const confirmLoading = ref(false);

// 表单模型
const modelRef = reactive({
  aqc_cookie: "",
});

// 表单验证规则和方法
const useForm = Form.useForm;
const { validate, validateInfos, resetFields } = useForm(
  modelRef,
  reactive({
    aqc_cookie: [
      {
        required: true,
        message: "请输入爱企查Cookie",
      },
    ],
  })
);

// 监听事件以显示模态框
onMounted(() => {
  listen("show_add_query_info", () => {
    showAddQueryInfo.value = true;
    AddQueryInfoisible.value = true;
  });

  // 监听来自后端的 "paste-from-clipboard" 事件
  // listen("paste-from-clipboard", async () => {
  //   try {
  //     const content = await readText(); // 从剪贴板读取数据
  //     if (content) {
  //       modelRef.aqc_cookie = content; // 将粘贴的内容赋值给输入框
  //       console.log("粘贴的内容:", modelRef.aqc_cookie);
  //       notification.success({ message: "已从剪贴板粘贴内容！" });
  //     } else {
  //       console.log("剪贴板内容为空");
  //     }
  //   } catch (error) {
  //     console.error("读取剪贴板失败:", error);
  //     notification.error({ message: "读取剪贴板失败！" });
  //   }
  // });
});

// 通知方法
const openNotificationWithIcon = (
  type: "success" | "info" | "warning" | "error",
  message: string
) => {
  notification[type]({
    message: message,
  });
};

// 重置表单
const resetForm = () => {
  resetFields();
  modelRef.aqc_cookie = "";
};

// 保存按钮的处理逻辑
const handleSave = async () => {
  try {
    await validate();
    console.log("粘贴的内容:", modelRef.aqc_cookie);

    // 确保数据传递到后端之前没有被修改
    await invoke("add_query_info", { aqcCookie: modelRef.aqc_cookie });
    debugger;
    openNotificationWithIcon("success", "添加成功");

    // 重置表单和状态
    resetForm();
    AddQueryInfoisible.value = false;
    showAddQueryInfo.value = false;
  } catch (err) {
    console.error("验证失败或数据传输失败:", err);
  } finally {
    confirmLoading.value = false; // 确保加载状态被重置
  }
};

// 取消按钮的处理逻辑
function handleCancel() {
  AddQueryInfoisible.value = false;
  showAddQueryInfo.value = false;
}
</script>
