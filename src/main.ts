import { createApp } from "vue";
import App from "./App.vue";
import Antd from 'ant-design-vue'
import 'ant-design-vue/dist/reset.css'

// PrimeVue 4.x 新主题系统（必须用这个！）
import PrimeVue from 'primevue/config'
import Aura from '@primevue/themes/aura'  // 现代主题（可换成 Lara 等）
import 'primeicons/primeicons.css'        // 图标（你已经装了 7.0.0）

import Menubar from 'primevue/menubar'   // Menubar 导入不变！
import './eventHandler'

const app = createApp(App)

// 关键：4.x 主题配置（这样就不会 404 了）
app.use(PrimeVue, {
  theme: {
    preset: Aura,  // Aura 是默认现代主题，美观大方
    options: {
      prefix: 'p',  // 组件类名前缀（默认 p-）
      darkModeSelector: '.dark',  // 可选：支持暗黑模式切换
    }
  }
})

app.use(Antd)
app.component('Menubar', Menubar)  // 全局注册（不变）
app.mount('#app')