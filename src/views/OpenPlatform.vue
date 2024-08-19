<template>
  <a-float-button tooltip="点此返回，未提交的数据将被清空" :style="{
    right: '24px'
  }" @click="back_create_report">
    <template #icon>
      <LeftCircleFilled style="color:'red'" />
    </template>
  </a-float-button>
  <div class="page-header" style="background-color: #f5f5f5; padding: 2px">
    <a-page-header :ghost="ghost" sub-title="查询企业信息">
      <template #extra>
        <a-button type="primary">查询信息</a-button>
      </template>
      <a-descriptions size="small" :column="2">
        <a-descriptions-item label="企业名称">2017-01-10</a-descriptions-item>
        <a-descriptions-item label="企业地址">2017-10-10</a-descriptions-item>
      </a-descriptions>
    </a-page-header>
  </div>
  <div>
    <iframe :src="url" style="width: 100%; height:880px; border: none;">
    </iframe>
  </div>

</template>
<script lang="ts" setup>
import { ref } from 'vue';
import { LeftCircleFilled } from '@ant-design/icons-vue';
import { eventBus } from '../eventBus';

//打开平台tab
const back_create_report = () => {
  eventBus.emit('back-create-report');
};

const ghost = ref(false);
const url = 'http://10.20.240.60/intelligence';  // 目标网站的URL
</script>

<style scoped>
.page-header :deep(tr:last-child td) {
  padding-bottom: -10;
}
</style>
