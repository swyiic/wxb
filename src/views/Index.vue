<template>
  <a-tabs v-model:activeKey="activeKey" style="margin-top: -20px;display: flex; " default-active-key="1" centered>
    <a-tab-pane key="1" style="min-width: 800px;" tab="创建报告">
      <CreateReport />
    </a-tab-pane>
    <a-tab-pane key="2" style="min-width: 800px;" tab="情报平台" disabled>
      <OpenPlatform />
    </a-tab-pane>
  </a-tabs>
</template>

<script lang="ts" setup>
import OpenPlatform from './OpenPlatform.vue';
import CreateReport from './CreateReport.vue';
import { ref, onMounted, onUnmounted } from 'vue';
import { eventBus } from '../eventBus';

const activeKey = ref('1');
//打开情报平台
const open_platform = () => {
  activeKey.value = '2';
};
onMounted(() => {
  eventBus.on('open-platform', open_platform);
});
onUnmounted(() => {
  eventBus.off('open-platform', open_platform);
});
//返回报告页面
const back_create_report = () => {
  activeKey.value = '1';
};
onMounted(() => {
  eventBus.on('back-create-report', back_create_report);
});
onUnmounted(() => {
  eventBus.off('back-create-report', back_create_report);
});

</script>
