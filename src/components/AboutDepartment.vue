<template>
  <a-modal
    :open="visible"
    title="关于工具"
    centered
    width="500px"
    :footer="null"
    @cancel="handleClose"
  >
    <div style="text-align: center; padding: 20px 0; font-size: 16px; line-height: 2;">
      <p>上海事业部交付部 ❤ 业务支撑部 ❤ 网信办项目专用</p>
      <p style="margin-top: 20px; color: #888;">
        版本：v1.0.0<br />
        开发时间：2023<br />
        如有问题自己解决
      </p>
    </div>
  </a-modal>
</template>

<script setup lang="ts">
import { watch } from 'vue'

// 使用 v-model 的标准写法
const props = defineProps<{
  visible: boolean
}>()

const emit = defineEmits<{
  (e: 'update:visible', value: boolean): void
}>()

// 同步父组件的 visible
const handleClose = () => {
  emit('update:visible', false)
}

watch(() => props.visible, (val) => {
  if (!val) {
    // 可以在这里做一些清理工作
  }
})
</script>

<style scoped>
/* 如需额外样式可自行添加 */
</style>