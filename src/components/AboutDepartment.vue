<template>
  <a-modal
    v-show="showAbout"
    title="关于工具"
    :visible="visible"
    :confirm-loading="confirmLoading"
    @ok="handleOk"
    @cancel="handleCancel"
  >
    <p>{{ ModalText }}</p>
  </a-modal>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { listen } from '@tauri-apps/api/event'

// 定义响应式变量
const showAbout = ref(false)
const visible = ref(false)
const confirmLoading = ref(false)
const ModalText = ref('上海事业部交付部❤业务支撑部❤网信办项目专用')

onMounted(() => {
  listen('show_about_department', () => {
    showAbout.value = true
    visible.value = true
    console.log('Tets connect show department...') // 调试日志
  })
})

function handleOk() {
  // ModalText.value = '---闪现---'
  confirmLoading.value = true
  setTimeout(() => {
    visible.value = false
    confirmLoading.value = false
    showAbout.value = false
  }, 0)
}

function handleCancel() {
  console.log('Clicked cancel button')
  visible.value = false
  showAbout.value = false
}
</script>
