// eventBus.ts
import mitt from "mitt";

// 定义事件类型
type Events = {
  "open-platform": void;
  "back-create-report": void;
  "data-update": void;
  updateContent: void;
};

// 创建和导出事件总线
export const eventBus = mitt<Events>();
