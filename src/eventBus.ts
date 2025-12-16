// eventBus.ts
import mitt from "mitt";

// 定义事件类型
type Events = {
  "show-reports": void;
  "back-create-report": void;
  "data-update": void;
  "updateContent": void;
  "show_add_vulnerability": void;
  "show_about_department": void;
  "show_add_platform_login_info": void;
  "show_pendin_reports_list": void;
  "show_report_details": string;
}
// 创建和导出事件总线
export const eventBus = mitt<Events>();
