<template>
  <div class="report-container">
    <a-page-header
      :ghost="false"
      title="漏洞预览"
      subtitle="部分已自动填充，补全剩余方可上传"
      @back="$emit('back')"
    >
      <template #extra>
        <div
          style="display: flex; flex-direction: column; align-items: flex-start"
        >
          <!-- 第一行：两个按钮水平并排 -->
          <div style="display: flex; gap: 12px; margin-bottom: 8px">
            <a-button
              type="primary"
              @click="getReportNumber"
              :loading="gettingNumber"
            >
              {{ gettingNumber ? "获取中..." : "获取报告编号" }}
            </a-button>
            <!-- :disabled="!form.fileIds || form.fileIds.length === 0" -->
            <a-button
              type="primary"
              danger
              @click="uploadReport"
              :loading="uploading"
            >
              {{ uploading ? "上传中..." : "上传报告" }}
            </a-button>
          </div>

          <!-- 第二行：只有获取到编号才显示，紧贴在“获取报告编号”按钮正下方，左对齐，灰色 -->
          <div
            v-if="form.fileIds && form.fileIds.length > 0"
            style="
              font-size: 12px;
              color: #999;
              margin-top: -4px;
              padding-left: 2px;
            "
          >
            报告编号：{{
              Array.isArray(form.fileIds)
                ? form.fileIds.join(", ")
                : form.fileIds
            }}
          </div>
        </div>
      </template>
    </a-page-header>

    <div class="form-section">
      <h3 style="margin-bottom: 16px">基础分类</h3>
      <table class="info-table">
        <tbody>
          <!-- 第一行：省 + 市 -->
          <tr>
            <td class="label">省</td>
            <td class="value">{{ provinceName }}</td>
            <td class="label">市</td>
            <td class="value">{{ cityName }}</td>
          </tr>
          <!-- 第二行：区县选择 -->
          <tr>
            <td class="label">区</td>
            <td class="value" colspan="3">
              <a-select
                v-model:value="form.district"
                allow-clear
                :options="districtOptions"
                show-search
                :filter-option="alarmSortFilterOption"
                placeholder="请选择区县（可搜索）"
                style="width: 100%"
              />
              <div style="margin-top: 6px; font-size: 12px; color: #999">
                指定区县需手动选择
              </div>
            </td>
          </tr>
          <!-- 第三行：情报类别 -->
          <tr>
            <td class="label">情报类别</td>
            <td class="value" colspan="3">
              <a-select
                v-model:value="form.alarmSort"
                :options="alarmSortOptions"
                allow-clear
                placeholder="请选择情报类别"
                @change="onAlarmSortChange"
                style="width: 100%"
              />
            </td>
          </tr>
          <!-- 第四行：情报类型 -->
          <tr>
            <td class="label">情报类型</td>
            <td class="value" colspan="3">
              <a-select
                v-model:value="form.riskType"
                :options="riskTypeOptions"
                placeholder="请选择情报类型"
                allow-clear
                :disabled="!form.alarmSort"
                style="width: 100%"
              />
            </td>
          </tr>
          <!-- 第五行：情报名称 -->
          <tr>
            <td class="label">情报名称</td>
            <td class="value" colspan="3">
              <a-select
                v-model:value="form.informationName"
                :options="informationNameOptions"
                allow-clear
                placeholder="请选择情报名称"
                style="width: 100%"
                :disabled="!form.riskType"
              />
              <div style="margin-top: 6px; font-size: 12px; color: #999">
                由于平台漏洞陈旧，无法与数据库对应时，请以报告原文为准，故当前下拉未展示平台漏洞名称
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    <div class="form-section">
      <h3 style="margin-bottom: 16px">漏洞信息</h3>
      <table class="info-table">
        <tbody>
          <tr>
            <td class="label">情报名称</td>
            <td class="value">{{ form.informationName || "-" }}</td>
            <td class="label">发现时间</td>
            <td class="value">{{ form.foundTime || "-" }}</td>
          </tr>
          <tr>
            <td class="label">情报描述</td>
            <td class="value" colspan="3">{{ form.alarmDesc || "-" }}</td>
          </tr>
          <tr>
            <td class="label">造成影响</td>
            <td class="value" colspan="3">{{ form.affectsType || "-" }}</td>
          </tr>
          <tr>
            <td class="label">解决方案</td>
            <td class="value" colspan="3">{{ form.solution || "-" }}</td>
          </tr>
        </tbody>
      </table>
    </div>
    <!-- 企业信息 -->
    <div class="form-section">
      <h3>企业信息</h3>
      <a-descriptions bordered :column="2">
        <a-descriptions-item label="企业名称">{{
          form.companyName
        }}</a-descriptions-item>
        <a-descriptions-item label="目标 URL">{{
          form.alarmTargetUrl
        }}</a-descriptions-item>
      </a-descriptions>
    </div>

    <!-- 地址信息 -->
    <div class="form-section">
      <h3>地址信息</h3>
      <a-descriptions bordered :column="2">
        <a-descriptions-item label="详细地址" :span="2">
          <a-input
            v-model:value="form.address"
            placeholder="请填写详细地址，可为空"
            allow-clear
          />
        </a-descriptions-item>
      </a-descriptions>
    </div>

    <div class="form-section">
      <h3 style="margin-bottom: 16px">报告信息</h3>
      <table class="info-table">
        <tbody>
          <tr>
            <td class="label">报告路径</td>
            <td class="value" style="word-break: break-all; line-height: 1.5">
              {{ reportFilePath || "暂无报告" }}
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { reactive, ref, computed, watch } from "vue";
// import {onMounted,nextTick} from "vue";
import { invoke } from "@tauri-apps/api/core";
import { readTextFile } from "@tauri-apps/plugin-fs";
import { message } from "ant-design-vue";
import type { SelectProps } from "ant-design-vue";

const props = defineProps<{
  filePath: string;
  rawData: any;
}>();

const emit = defineEmits<{
  back: [];
  close: [];
  refresh: [];
}>();

const form = reactive({
  companyName: "",
  companyWebName: "",
  alarmTargetUrl: "",
  province: "",
  city: "",
  district: "",
  address: "",
  alarmSort: undefined as undefined | 1 | 2 | 3 | 4,
  riskType: undefined as number | undefined,
  informationName: "",
  foundTime: "",
  alarmDesc: "",
  affectsType: "",
  solution: "",
  fileIds: [""] as string[],
  taskId: "",
  taskListId: "",
  alarmTargetIp: "",
  alarmPort: "",
});
const reportFilePath = ref<string>("");
const pendingFilePath = ref(props.filePath);

const gettingNumber = ref(false);
const uploading = ref(false);

// 一级选项情报类别
const alarmSortOptions = ref<SelectProps["options"]>([
  { label: "隐患", value: 1 },
  { label: "漏洞", value: 2 },
  { label: "事件", value: 3 },
  { label: "风险提示", value: 4 },
]);
const alarmSortFilterOption = (input: string, alarmSortOptions: any) => {
  return alarmSortOptions.value.toLowerCase().indexOf(input.toLowerCase()) >= 0;
};
const alarmSortToDbId = {
  1: "160",
  2: "161",
  3: "162",
  4: "393",
} as const;
// 二级下拉情报类类型
const riskTypeOptions = computed(() => {
  if (!form.alarmSort || !props.rawData.event_types?.length) return [];

  const parentDbId = alarmSortToDbId[form.alarmSort];
  if (!parentDbId) return [];

  return props.rawData.event_types
    .filter((item: any) => {
      return (
        String(item.parentId) === parentDbId && // 注意：字段是 parentId
        item.informationName?.trim() &&
        item.data_value != null
      ); // 关键！是 data_value，不是 dataValue
    })
    .map((item: any) => ({
      label: item.informationName.trim(),
      value: Number(item.data_value),
    }))
    .sort((a: any, b: any) => Number(a.value) - Number(b.value));
});

watch(
  () => form.riskType,
  (newValue) => {
    if (!newValue) {
      console.log("【情报类型】已清空");
      return;
    }

    // 找到当前选中的这条记录
    const item = props.rawData.event_types.find(
      (x: any) => Number(x.data_value) === newValue
    );

    if (item) {
      console.log(
        `【当前选择】label=${item.informationName}  value=${newValue}  父级id=${item.parentId}  父级值=${form.alarmSort}`
      );
    }
  },
  { immediate: false } // 改成 true 的话打开页面也会打一次
);

// 三级下拉情报名称
const informationNameOptions = computed(() => {
  if (!form.riskType || !props.rawData.event_types?.length) return [];

  // 注意：这里直接用 number 比较
  const current = props.rawData.event_types.find(
    (item: any) => Number(item.dataValue) === form.riskType // ← 改这里
  );

  if (!current) return [];

  const children = props.rawData.event_types.filter(
    (item: any) =>
      String(item.parentId) === String(current.id) && item.dataValue != null
  );

  if (children.length > 0) {
    return children.map((item: any) => ({
      label: item.informationName?.trim() || item.dataName || "未知",
      value: Number(item.dataValue),
    }));
  }

  return [
    {
      label: current.informationName?.trim() || current.dataName || "未知",
      value: Number(form.riskType),
    },
  ];
});

const provinceName = computed(() => {
  const p = props.rawData.provinces.find((x: any) => x.id === form.province);
  return p?.name || "未选择";
});

const cityName = computed(() => {
  if (form.province === "31" && form.city === "3101") return "上海市";
  const c = props.rawData.areas.find((x: any) => x.id === form.city);
  return c?.name || "未选择";
});

const districtOptions = computed(() => {
  return props.rawData.areas
    .filter((a: any) => a.pid === form.city)
    .map((a: any) => ({ label: a.name, value: String(a.id) }));
});

// 加载单个报告文件内容
const loadReportFromFile = async () => {
  if (!props.filePath) return;
  console.log("[1] 开始加载报告文件:", props.filePath);
  try {
    const content = await readTextFile(props.filePath);
    console.log("[2] 文件读取成功，长度:", content.length);
    console.log("[3] 文件读取成功，长度:", content);
    const data = JSON.parse(content);
    const localReportPath = data.localReportPath || "";

    form.companyName = data.companyName ?? "";
    form.companyWebName = data.companyWebName ?? "";
    form.alarmTargetUrl = data.alarmTargetUrl ?? "";
    form.province = String(data.province || "31");
    form.city = String(data.city || "3101");
    form.district = data.district ? String(data.district) : "";
    form.address = data.address ?? "";
    form.informationName = data.informationName ?? "";
    form.foundTime = data.foundTime ?? "";
    form.alarmSort = data.alarmSort
      ? (Number(data.alarmSort) as 1 | 2 | 3 | 4)
      : undefined;
    form.riskType = data.riskType ? Number(data.riskType) : undefined;
    form.alarmDesc = data.alarmDesc ?? "";
    form.affectsType = data.affectsType ?? "";
    form.solution = data.solution ?? "";
    form.fileIds = data.fileIds ?? [];
    form.taskId = data.fields ?? "";
    form.taskListId = data.fields ?? "";
    form.alarmTargetIp = data.alarmTargetIp ?? "";
    form.alarmPort = data.alarmPort ?? "";
    // 本地展示路径
    reportFilePath.value = localReportPath;
    console.log("[5] form 对象本身:", form);
    console.log("[6] 直接访问 form.alarmDesc =", form.alarmDesc);
    console.log("[7] 直接访问 form.affectsType =", form.affectsType);
    console.log("[4] 表单填充完成:", form);
    message.success(`已加载：${data.companyName || "未知企业"}`);
  } catch (e: any) {
    console.error("[ERROR] 加载报告失败:", e);
    message.error("加载报告失败：" + e);
  }
};

watch(
  () => form.informationName,
  (name) => {
    if (!name) return;

    const item = props.rawData.event_types.find(
      (e: any) => e.informationName === name
    );

    if (item) {
      // 关键：只有当本地文件没有提供描述时，才用平台的（防止覆盖）
      if (!form.alarmDesc?.trim()) {
        form.alarmDesc = item.alarmDesc?.trim() || "";
      }
      if (!form.affectsType?.trim()) {
        form.affectsType = item.affectsType?.trim() || "";
      }
    }
  }
);

const onAlarmSortChange = () => {
  form.riskType = undefined;
  form.informationName = "";
  form.alarmDesc = "";
  form.affectsType = "";
};

const getReportNumber = async () => {
  if (!reportFilePath.value) {
    message.warning("未找到报告文件路径");
    return;
  }
  console.log("[INFO] 获取到的报告路径:", reportFilePath.value);
  try {
    const num: string = await invoke("get_report_number", {
      filePath: reportFilePath.value,
    });
    form.fileIds = [num];
    message.success("报告编号：" + form.fileIds);
  } catch (e: any) {
    message.error("获取失败：" + e);
  } finally {
    gettingNumber.value = false;
  }
};

const uploadReport = async () => {
  console.log("[INFO] 准备上传报告，表单数据:", form);
  if (!form.alarmTargetUrl?.trim()) {
    message.error("企业URL不能为空！");
    return;
  } else if (!form.riskType) {
    message.error("情报类型为必选字段！若无法自动获取请手动选择");
    return;
  } else if (!form.alarmSort) {
    message.error("选择情报类为必选字段！若无法自动获取请手动选择");
    return;
  } else if (!form.fileIds) {
    message.error("报告ID必须填写完整！请手动获取");
    return;
  }
  uploading.value = true;
  try {
    // 直接 await，不接收返回值（因为我们只关心是否成功）
    await invoke("process_submit_form", {
      formData: form,
      pendingFilePath: pendingFilePath.value,
    });
    message.success("情报提交成功，已删除本地记录");
    emit("close"); // 关闭弹窗
    emit("refresh"); // 触发列表刷新（推荐）
  } catch (e: any) {
    if (typeof e === "string" && (e.includes("Token") || e.includes("登录"))) {
      message.error("登录已失效，请重新登录");
    } else {
      message.error("提交失败：" + e);
    }
  } finally {
    uploading.value = false;
  }
};

watch(
  () => props.filePath,
  (newPath) => {
    if (newPath) {
      loadReportFromFile();
    }
  },
  { immediate: true }
);
</script>

<style scoped>
.report-container {
  padding: 24px;
}
.form-section {
  margin-top: 24px;
}

.info-table {
  width: 100%;
  border-collapse: collapse;
  background: #fff;
  font-size: 14px;
}
.info-table td {
  padding: 12px 16px;
  border: 1px solid #f0f0f0;
  vertical-align: top;
}
.info-table .label {
  background: #fafafa;
  width: 120px;
  font-weight: 600;
  color: #666;
}
.info-table .value {
  color: #333;
}
</style>
