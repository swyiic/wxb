<template>
  <!-- <a-float-button
    tooltip="返回创建报告"
    :style="{ right: '24px', bottom: '80px' }"
    @click="back_create_report"
  >
    <template #icon>
      <LeftCircleFilled style="color: red" />
    </template>
  </a-float-button> -->

  <div class="pending-wrapper">
    <!-- 列表页 -->
    <div v-if="!currentFile">
      <h2 style="margin-bottom: 16px">待上传报告列表（共 {{ pendingReports.length }} 份）</h2>

      <a-list bordered :data-source="sortedPendingReports" v-if="pendingReports.length">
        <template #renderItem="{ item, index }">
          <a-list-item>
            <!-- 左侧序号 -->
            <div style="width: 50px; text-align: center; color: #999; font-weight: bold;">
              {{ index + 1 }}
            </div>

            <a-list-item-meta
              :title="item.companyName || '未知企业'"
              :description="`
                漏洞名称：${item.informationName || '未知'} 
                目标URL：${item.alarmTargetUrl || '-'}
                本地路径：${item.localReportPath || '-'}
              `"
            />

            <template #actions>
              <a-button type="link" size="small" @click="onSelectReport(item._pending_file)">
                查看详情并上传
              </a-button>
            </template>
          </a-list-item>
        </template>
      </a-list>

      <a-empty v-else description="暂无待上传报告" />
    </div>

    <!-- 详情页 -->
    <ShowReportsDetails
      v-else
      :file-path="currentFile!"
      :raw-data="rawData"
      @back="currentFile = null"
      @refresh="onReportSubmitted"
    />
  </div>
</template>

<script lang="ts" setup>
import { ref, shallowRef, onMounted, computed } from "vue";
// import { LeftCircleFilled } from "@ant-design/icons-vue";
import { invoke } from "@tauri-apps/api/core";
import { message } from "ant-design-vue";
import ShowReportsDetails from "./ShowReportsDetails.vue";
// import { eventBus } from "../eventBus";
import dayjs from "dayjs";

// 数据本体（shallowRef 防打包坑）
const rawData = shallowRef<{
  provinces: any[];
  areas: any[];
  event_types: any[];
  list: any[];
}>({
  provinces: [],
  areas: [],
  event_types: [],
  list: []
});

const currentFile = ref<string | null>(null);
const pendingReports = ref<any[]>([]);

// 计算属性：按创建时间倒序 + 格式化时间
const sortedPendingReports = computed(() => {
  return [...pendingReports.value]
    .sort((a, b) => {
      // 假设后端返回的字段叫 create_time（字符串或时间戳），没有就排到最后
      const ta = a.create_time || a.createTime || 0;
      const tb = b.create_time || b.createTime || 0;
      return new Date(tb).getTime() - new Date(ta).getTime(); // 降序
    })
    .map((item) => ({
      ...item,
      // 给每一项加上格式化后的时间，方便模板直接用
      createTime: formatTime(item.create_time || item.createTime),
    }));
});
// 时间格式化
function formatTime(time: any): string {
  if (!time) return "未知时间";
  const d = dayjs(time);
  if (!d.isValid()) return "无效时间";
  return d.format("YYYY-MM-DD HH:mm");
}


const loadAllData = async () => {
  try {
    const data: any = await invoke("load_all_data");
    
    const newData = {
      provinces: (data.provinces || []).map((p: any) => ({ ...p, id: String(p.id) })),
      areas: (data.areas || []).map((a: any) => ({
        ...a,
        id: String(a.id),
        pid: String(a.pid || "")
      })),
      event_types: (data.event_types || []).map((it: any) => ({
        ...it,
        id: String(it.id)
      })),
      list: data.list || []
    };

    rawData.value = newData;

    // 把原始 list 直接存进来，后面排序和格式化都在 computed 里处理
    pendingReports.value = newData.list.map((item: any) => ({
      ...item,
      _pending_file: item._pending_file,
      companyName: item.companyName,
      alarmTargetUrl: item.alarmTargetUrl,
      localReportPath: item.localReportPath,
      // 保留原始时间字段，后面排序要用
      create_time: item.create_time || item.createTime
    }));

    message.success(`数据加载成功 ${pendingReports.value.length} 份待上传报告 `);
  } catch (e: any) {
    console.error("[ERROR] load_all_data 失败:", e);
    message.error("加载基础数据失败：" + e);
  }
};

const onSelectReport = (filePath: string) => {
  currentFile.value = filePath;
};

const onReportSubmitted = () => {
  pendingReports.value = pendingReports.value.filter(
    item => item._pending_file !== currentFile.value
  )
  currentFile.value = null
  message.success("已更新待上传列表")
}

// const back_create_report = () => {
//   eventBus.emit("back-create-report");
// };

onMounted(() => {
  loadAllData();
});
</script>

<style scoped>
.pending-wrapper {
  min-height: 600px;
  background: #fafafa;
  border-radius: 8px;
  padding: 24px;
}

/* 让列表项内容左对齐，序号单独占一列 */
:deep(.ant-list-item) {
  display: flex;
  align-items: center;
}
:deep(.ant-list-item-meta-description) {
  white-space: pre-line;   /* 关键！自动识别 \n 换行 */
}
</style>