<template>
  <!-- <div class="image-uploader" contenteditable="true" @input="handleInput" @paste="handlePaste" @drop="handleDrop"
    @dragover.prevent>
  </div> -->
  <div ref="assistUploader" class="image-uploader" contenteditable="true" @input="handleInput" @paste="handlePaste"
    @drop="handleDrop" @dragover.prevent>
  </div>
</template>

<script setup lang="ts">
import { defineEmits, ref } from 'vue';

const assistUploader = ref<HTMLElement | null>(null);

// 暴露给父组件的方法，用于重置内容
const resetContent = () => {
  if (assistUploader.value) {
    assistUploader.value.innerHTML = '';
    emit('updateAssistContent', ''); // 通知父组件内容已被重置
  }
};
// 通过 `defineExpose` 暴露方法
defineExpose({
  resetContent,
});


const emit = defineEmits(['updateAssistContent']);
const handleInput = () => {
  const uploader = document.querySelector('.image-uploader') as HTMLElement;
  const content = uploader.innerHTML;
  emit('updateAssistContent', content);
};

const handlePaste = (event: ClipboardEvent) => {
  event.preventDefault();

  const clipboardData = event.clipboardData;
  if (clipboardData) {
    // 处理图片
    const items = clipboardData.items;
    if (items) {
      for (const item of items) {
        if (item.kind === 'file' && item.type.startsWith('image/')) {
          const file = item.getAsFile();
          if (file) {
            const reader = new FileReader();
            reader.onload = (e) => {
              insertImage(e.target?.result as string);
              handleInput();  // 在插入图片后调用 handleInput
            };
            reader.readAsDataURL(file);
          }
        }
      }
    }
  }
};

const handleDrop = (event: DragEvent) => {
  event.preventDefault();

  const dataTransfer = event.dataTransfer;
  if (dataTransfer) {
    const items = dataTransfer.items;
    if (items) {
      for (const item of items) {
        if (item.kind === 'file' && item.type.startsWith('image/')) {
          const file = item.getAsFile();
          if (file) {
            const reader = new FileReader();
            reader.onload = (e) => {
              insertImage(e.target?.result as string);
              handleInput();  // 在插入图片后调用 handleInput
            };
            reader.readAsDataURL(file);
          }
        }
      }
    }
  }
};

const insertImage = (src: string) => {
  const img = document.createElement('img');
  img.src = src;
  img.style.maxWidth = '85%';
  img.style.height = 'auto';
  const uploader = document.querySelector('.image-uploader') as HTMLElement;
  uploader.appendChild(img);
  moveCursorToEnd(uploader);
};

// const insertText = (text: string) => {
//   const uploader = document.querySelector('.image-uploader') as HTMLElement;
//   const textNode = document.createTextNode(text);
//   uploader.appendChild(textNode);
//   moveCursorToEnd(uploader);
// };

const moveCursorToEnd = (element: HTMLElement) => {
  const range = document.createRange();
  const sel = window.getSelection();
  range.selectNodeContents(element);
  range.collapse(false);
  sel?.removeAllRanges();
  sel?.addRange(range);
};
</script>

<style scoped>
.image-uploader {
  border: 1px solid #ccc;
  padding: 5px;
  min-height: 200px;
  width: 100%;
  overflow: auto;
  white-space: pre-wrap;
  background-color: #fff;
  box-sizing: border-box;
}

.image-uploader img {
  max-width: 85%;
  height: auto;
}
</style>