import { listen } from '@tauri-apps/api/event';

// 定义处理器函数类型
type EventHandler = (payload: any) => void;

// 全局事件处理器对象
const eventHandler = {
  handlers: {} as Record<string, EventHandler[]>,

  // 注册事件和对应的处理函数
  register(eventType: string, handler: EventHandler) {
    if (!this.handlers[eventType]) {
      this.handlers[eventType] = [];
    }
    this.handlers[eventType].push(handler);
  },

  // 初始化监听所有事件
  init() {
    // 监听所有来自后端的事件
    listen<string>('text-pasted', (event) => this.dispatch('text-pasted', event.payload));
    listen<string>('image-pasted', (event) => this.dispatch('image-pasted', event.payload));
  },

  // 分发事件到相应的处理函数
  dispatch(eventType: string, payload: any) {
    if (this.handlers[eventType]) {
      this.handlers[eventType].forEach((handler) => handler(payload));
    }
  },
};

// 初始化事件处理器
eventHandler.init();

export default eventHandler;