<template>
  <a-modal
    :open="modalVisible"
    title="🔐 手动粘贴 SRC 平台请求头"
    width="960px"
    centered
    destroy-on-close
    :mask-closable="false"
    @cancel="close"
  >
    <div class="modal-body">
      <a-textarea
        v-model:value="headersText"
        placeholder="请粘贴完整 Request Headers
        打开浏览器，进入 SRC 平台，按 F12 打开开发者工具，切换到 Network（网络）标签页，刷新页面，点击任意一个请求，在 Headers（请求头）部分找到 Request Headers，将其完整内容复制粘贴到此处。
        "
        :rows="22"
      />
    </div>

    <template #footer>
      <a-space>
        <a-button html-type="button" @click="close">取消</a-button>
        <a-button
          html-type="button"
          type="primary"
          :loading="saving"
          @click="parseAndSave"
        >
          {{ saving ? '保存中...' : '解析并保存' }}
        </a-button>
      </a-space>
    </template>
  </a-modal>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { message } from 'ant-design-vue'

const props = defineProps<{ visible: boolean }>()
const emit = defineEmits(['update:visible'])

const modalVisible = computed({
  get: () => props.visible,
  set: v => emit('update:visible', v)
})

const headersText = ref('')
const saving = ref(false)

const close = () => {
  modalVisible.value = false
}

const parseAndSave = async () => {
  if (!headersText.value.trim()) {
    message.warning('请先粘贴请求头')
    return
  }

  console.log("[UI] parseAndSave clicked, headersText length =", headersText.value.length)
  if (headersText.value.length > 1000) {
    console.log("[UI] headers_text (first 1000 chars):", headersText.value.slice(0, 1000))
  } else {
    console.log("[UI] headers_text (full):", headersText.value)
  }

  saving.value = true

  try {
    const result = await invoke<string>('save_auth_from_headers', {
      headersText: headersText.value
    })
    console.log("[UI] invoke result:", result)
    message.success(result || '保存成功')
    modalVisible.value = false
  } catch (err: any) {
    // 完整捕获错误，避免冒泡
    console.error("[UI] invoke error (raw):", err)
    try {
      console.error("[UI] invoke error (stringified):", JSON.stringify(err, Object.getOwnPropertyNames(err), 2))
    } catch {}

    const friendly = (err && (err.message || err.toString())) || '保存失败'
    message.error(friendly)
  } finally {
    saving.value = false
  }
}
</script>

<style scoped>
.modal-body {
  margin-bottom: 16px;
}
</style>