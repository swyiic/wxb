<template>
  <!-- 强制不折叠的 Menubar -->
  <Menubar 
    :model="menuItems" 
    class="custom-menubar"
    :pt="{ root: { class: 'no-collapse' } }"
  />

  <!-- Tabs 主界面 -->
  <a-tabs v-model:activeKey="activeKey" style="margin-top: 10px;" centered>
    <a-tab-pane key="1" tab="创建报告">
      <CreateReport />
    </a-tab-pane>

    <!-- 关键：待提交报告 Tab -->
    <a-tab-pane key="2" tab="待提交报告">
      <PendingReportsWrapper 
      ref="reportsRef" 
      
      />
    </a-tab-pane>
  </a-tabs>

  <!-- 弹窗们 -->
  <AddVulnerability v-model:visible="showAddVuln" />
  <AddPlatformLoginInfo v-model:visible="showAddPlatformLogin" />
  <AboutDepartment v-model:visible="showAbout" />
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, watch } from 'vue'
import CreateReport from './CreateReport.vue'
import PendingReportsWrapper from './PendingReportsWrapper.vue'

import AddVulnerability from '../components/AddVulnerability.vue'
import AddPlatformLoginInfo from '../components/AddPlatformLoginInfo.vue'
import AboutDepartment from '../components/AboutDepartment.vue'

import Menubar from 'primevue/menubar'
import { message } from "ant-design-vue"
import { eventBus } from '../eventBus'

// ==================== 弹窗控制 ====================
const showAddVuln = ref(false)
const showAddPlatformLogin = ref(false)
const showAbout = ref(false)

// ==================== Menubar 菜单（平铺美观）===================
const menuItems = ref([
  { label: '新增漏洞', icon: 'pi pi-plus-circle', command: () => { showAddVuln.value = true } },
  { label: '平台登陆配置', icon: 'pi pi-sign-in', command: () => { showAddPlatformLogin.value = true } },
  { label: '关于工具', icon: 'pi pi-info-circle', command: () => { showAbout.value = true } },
  { separator: true },
  { label: '使用说明', icon: 'pi pi-question-circle', command: () => { message.info('请参考情报平台操作手册') } },
  { label: '检查更新', icon: 'pi pi-refresh', command: () => { message.success('当前已是最新版本') } },
])

// ==================== Tab 控制 + 自动刷新 ====================
const activeKey = ref('1')  // 默认创建报告
const reportsRef = ref<any>(null)

// 每次切换到“待提交报告”Tab，自动刷新列表
watch(activeKey, async (newKey) => {
  if (newKey === '2') {
    await nextTick()
    reportsRef.value?.refresh?.()
  }
})

// 全局事件：从创建页跳转到待提交报告
const show_reports = () => {
  activeKey.value = '2'
}
const back_create_report = () => {
  activeKey.value = '1'
}

onMounted(() => {
  eventBus.on('show-reports', show_reports)
  eventBus.on('back-create-report', back_create_report)
})

onUnmounted(() => {
  eventBus.off('show-reports', show_reports)
  eventBus.off('back-create-report', back_create_report)
})
</script>

<style scoped>
.custom-menubar {
  border: none !important;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  padding: 0 20px;
  justify-content: center;
}

/* 强制不折叠菜单 */
:deep(.no-collapse .p-menubar-button) {
  display: none !important;
}
:deep(.no-collapse .p-menubar-root-list) {
  flex-wrap: nowrap !important;
}

/* 美化菜单项 */
:deep(.p-menuitem-link:hover) {
  background: rgba(64, 158, 255, 0.15) !important;
  border-radius: 6px;
}
</style>