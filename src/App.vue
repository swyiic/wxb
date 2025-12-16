<template>
  <div class="container">
    <Index />
    <AboutDepartment v-model:visible="showAbout" />
    <AddVulnerability v-model:visible="showAddVuln" />
    <AddPlatformLoginInfo v-model:visible="showPlatformLogin"/>
  </div>
</template>

<script setup lang="ts">
// 引入组件
import { ref } from 'vue'
import Index from "./views/Index.vue";
import AboutDepartment from "./components/AboutDepartment.vue"
import AddVulnerability from "./components/AddVulnerability.vue";
import AddPlatformLoginInfo from "./components/AddPlatformLoginInfo.vue";
import { eventBus } from './eventBus'  // 你的 eventBus 路径

// 每个弹窗都要有自己的响应式开关！必须写这四行！
const showAbout = ref(false)
const showAddVuln = ref(false)
const showPlatformLogin = ref(false)


eventBus.on('show_about_department', () => { showAbout.value = true })
eventBus.on('show_add_vulnerability', () => { showAddVuln.value = true })
eventBus.on('show_add_platform_login_info', () => { showPlatformLogin.value = true })


</script>


<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 2vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: left;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
