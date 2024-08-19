<template>
    <a-modal
      v-show="showAddPlatformLoginInfo"
      title="🤡添加情报平台信息🤡"
      :visible="AddPlatformLoginInfoVisible"
      :confirm-loading="confirmLoading"
      @ok="handleOk"
      @cancel="handleCancel"
    >
    <div class="row">
      <div class="column">
        <label>🤡平台账号</label>
        <a-input placeholder="username" allow-clear />
      </div>
    </div>
    <div class="row">
      <div class="column">
        <label>🤡平台密码</label>
        <a-input placeholder="password" allow-clear />
      </div>
    </div>
    </a-modal>
  </template>
  
  <script setup lang="ts">
  import { ref, onMounted } from 'vue'
  import { listen } from '@tauri-apps/api/event'
  
  // 定义响应式变量
  const showAddPlatformLoginInfo = ref(false)
  const AddPlatformLoginInfoVisible = ref(false)
  const confirmLoading = ref(false)

  
  onMounted(() => {
    listen('show_add_platform_login_info', () => {
        showAddPlatformLoginInfo.value = true
        AddPlatformLoginInfoVisible.value = true
        console.log('Tets connect show department...') // 调试日志
    })
  })
  
  function handleOk() {
    // ModalText.value = '---闪现---'
    confirmLoading.value = true
    setTimeout(() => {
      AddPlatformLoginInfoVisible.value = false
      confirmLoading.value = false
      showAddPlatformLoginInfo.value = false
    }, 0)
  }
  
  function handleCancel() {
    console.log('Clicked cancel button')
    AddPlatformLoginInfoVisible.value = false
    showAddPlatformLoginInfo.value = false
  }
  </script>
  