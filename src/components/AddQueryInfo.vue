<template>
  <a-modal v-show="showAddQueryInfo" title="🔎添加爱企查信息🔍" :visible="AddQueryInfoisible" :confirm-loading="confirmLoading"
    @cancel="handleCancel" :footer="null">
    <a-form :model="modelRef">
      <a-form-item v-bind="validateInfos.aqc_cookie">
        <label>😬爱企查Cookie</label>
        <a-textarea placeholder="
        Step1.登录百度爱企查
        Step2.打开开发者工具，点击任意功能
        Step3.选择任意ajax请求后，复制全部Cookies
        保存按钮没反应就是没填
        " v-model:value="modelRef.aqc_cookie" :rows="10" allow-clear />
      </a-form-item>
      <div style="text-align: right; margin-top: 16px;">
        <a-button @click="handleCancel" style="margin-right: 8px;">取消</a-button>
        <a-button type="primary" @click="handleSave">保存</a-button>
      </div>
    </a-form>
  </a-modal>
</template>

<script setup lang="ts">
import { ref, onMounted, reactive } from 'vue'
import { notification, Form } from 'ant-design-vue';
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri';

// 定义响应式变量
const showAddQueryInfo = ref(false)
const AddQueryInfoisible = ref(false)
const confirmLoading = ref(false)

onMounted(() => {
  listen('show_add_query_info', () => {
    showAddQueryInfo.value = true
    AddQueryInfoisible.value = true
  })
})
const openNotificationWithIcon = (type: 'success' | 'info' | 'warning' | 'error') => {
  notification[type]({
    message: '添加成功'
  });
};
const useForm = Form.useForm;
const modelRef = reactive({
  aqc_cookie: '',
});
const { validate, validateInfos, resetFields } = useForm(
  modelRef,
  reactive({
    aqc_cookie: [
      {
        required: true,
        message: '请输入爱企查Cookie',
      },
    ]
  }),
);
const resetForm = () => {
  resetFields();
  modelRef.aqc_cookie = '';
};
const handleSave = async () => {
  await validate().then(() => {
    openNotificationWithIcon('success');
    confirmLoading.value = true
    AddQueryInfoisible.value = false
    confirmLoading.value = false
    showAddQueryInfo.value = false
  }).catch(err => {
    console.log('验证失败:', err);
  });
  await invoke('add_query_info', {
    aqcCookie: modelRef.aqc_cookie,
  });
  resetForm()
}

function handleCancel() {
  AddQueryInfoisible.value = false
  showAddQueryInfo.value = false
}
</script>