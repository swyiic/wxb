<!-- ReportForm.vue -->
<template>
  <a-form autocomplete="off" @submit.prevent="generateReport">
    <!-- 第一行：单位名称 + 网站地址 -->
    <a-row :gutter="24" justify="center">
      <a-col :span="1"></a-col>
      <a-col :span="11">
        <div class="form-item">
          <label class="ant-form-item-required">单位名称</label>
          <a-input v-model:value="companyName" placeholder="单位名称" allow-clear />
        </div>
      </a-col>
      <a-col :span="11">
        <div class="form-item">
          <label class="ant-form-item-required">网站地址</label>
          <a-input v-model:value="webUrl" placeholder="网站URL" allow-clear />
        </div>
      </a-col>
      <a-col :span="1"></a-col>
    </a-row>

    <!-- 第二行：发现时间 + 系统名称 -->
    <a-row :gutter="24" justify="center">
      <a-col :span="1"></a-col>
      <a-col :span="11">
        <div class="form-item">
          <label class="ant-form-item-required">发现时间</label>
          <a-date-picker
            v-model:value="findDate"
            format="YYYY-MM-DD"
            style="width: 100%"
            :allow-clear="false"
          />
        </div>
      </a-col>
      <a-col :span="11">
        <div class="form-item">
          <label class="ant-form-item-required">系统名称</label>
          <a-input v-model:value="systemName" placeholder="系统名称" allow-clear />
        </div>
      </a-col>
      <a-col :span="1"></a-col>
    </a-row>

    <!-- 第三行：漏洞名称 + 漏洞URL -->
    <a-row :gutter="24" justify="center">
      <a-col :span="1"></a-col>
      <a-col :span="11">
        <div class="form-item">
          <label class="ant-form-item-required">漏洞名称</label>
          <a-select
            v-model:value="vulnName"
            :options="vulnOptions"
            placeholder="下拉选择漏洞，无结果请手动添加"
            show-search
            allow-clear
            @dropdownVisibleChange="handleDropdownVisibleChange"
            @change="handleVulnChange"
          />
        </div>
      </a-col>
      <a-col :span="11">
        <div class="form-item">
          <label class="ant-form-item-required">漏洞URL</label>
          <a-input
            v-model:value="vulnUrl"
            placeholder="如果过长导致单元格样式损害则写在附录中"
            allow-clear
          />
        </div>
      </a-col>
      <a-col :span="1"></a-col>
    </a-row>

    <!-- 资产截图 -->
    <a-row :gutter="24" justify="center">
      <a-col :span="1"></a-col>
      <a-col :span="22">
        <div class="form-item">
          <label class="ant-form-item-required">资产截图</label>
          <AssistImageUploader
            ref="assistUploader"
            :content="assistImage"
            @updateAssistContent="assistImage = $event"
          />
        </div>
      </a-col>
      <a-col :span="1"></a-col>
    </a-row>

    <!-- 测试过程 -->
    <a-row :gutter="24" justify="center">
      <a-col :span="1"></a-col>
      <a-col :span="22">
        <div class="ant-form-item-required">
          <label>测试过程</label>
          <TestProcessImageUploader
            ref="testProcessUploader"
            :content="testProcess"
            @updateTestProcessContent="testProcess = $event"
          />
        </div>
      </a-col>
      <a-col :span="1"></a-col>
    </a-row>

    <!-- 漏洞描述 -->
    <a-row :gutter="24" justify="center">
      <a-col :span="1"></a-col>
      <a-col :span="22">
        <div class="form-item">
          <label class="ant-form-item-required">漏洞描述</label>
          <a-textarea
            v-model:value="vulnDescription"
            placeholder="根据下拉自动获取"
            :rows="4"
            allow-clear
          />
        </div>
      </a-col>
      <a-col :span="1"></a-col>
    </a-row>

    <!-- 修复建议 -->
    <a-row :gutter="24" justify="center">
      <a-col :span="1"></a-col>
      <a-col :span="22">
        <div class="form-item">
          <label class="ant-form-item-required">修复建议</label>
          <a-textarea
            v-model:value="fixSuggestion"
            placeholder="根据下拉自动获取"
            :rows="4"
            allow-clear
          />
        </div>
      </a-col>
      <a-col :span="1"></a-col>
    </a-row>

    <!-- 底部按钮 -->
    <a-row :gutter="24" justify="center">
      <a-col :span="1"></a-col>
      <a-col :span="12">
        <a-button type="dashed" block @click="showDrawer">
          添加数据（Burp 包等附录）
        </a-button>
      </a-col>
      <a-col :span="5">
        <a-button type="primary" block html-type="submit" :loading="generating">
          生成报告
        </a-button>
      </a-col>
      <a-col :span="1"></a-col>
    </a-row>
  </a-form>

  <!-- 附录抽屉 -->
  <a-drawer
    title="添加详细数据"
    placement="right"
    width="520"
    :open="drawerOpen"
    @close="drawerOpen = false"
  >
    <a-textarea
      v-model:value="appendix"
      placeholder="BurpSuite 数据包、长 URL、POC 等都可粘贴在这里"
      :rows="15"
    />
    <template #footer>
      <a-button @click="drawerOpen = false">关闭</a-button>
    </template>
  </a-drawer>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { notification } from 'ant-design-vue'
import { invoke } from '@tauri-apps/api/core'
import dayjs from 'dayjs'
import AssistImageUploader from "../components/AssistImageUploader.vue";
import TestProcessImageUploader from "../components/TestProcessImageUploader.vue";

// 所有字段用 ref 管理，永不丢失
const companyName = ref('')
const webUrl = ref('')
const findDate = ref(dayjs()) // 默认今天
const systemName = ref('')
const vulnName = ref<string | undefined>(undefined)
const vulnUrl = ref('')
const vulnDescription = ref('')
const fixSuggestion = ref('')
const affectsType = ref('') // 关键！再也不会丢
const assistImage = ref('')
const testProcess = ref('')
const appendix = ref('')

const vulnOptions = ref<{ label: string; value: string }[]>([])
const generating = ref(false)
const drawerOpen = ref(false)

//  false)

const showDrawer = () => (drawerOpen.value = true)

// 漏洞下拉加载
const handleDropdownVisibleChange = async (open: boolean) => {
  if (!open) return
  try {
    const list = await invoke<any[]>('query_all_vulnerability')
    vulnOptions.value = list.map(v => ({
      label: v.vuln_name,
      value: v.vuln_name
    }))
  } catch (e) {
    notification.error({ message: '加载漏洞库失败' })
  }
}

// 选择漏洞后自动填充
const handleVulnChange = async (name: string) => {
  try {
    const res = await invoke<any[]>('search_vulnerability_by_name', { query: name })
    if (res.length === 0) return
    const v = res[0]
    vulnDescription.value = v.vuln_description ?? ''
    fixSuggestion.value = v.fix_suggestion ?? ''
    affectsType.value = v.affects_type ?? ''
    console.log('下拉获取漏洞全部信息', v)
  } catch (e) {
    notification.error({ message: '获取漏洞详情失败' })
  }
}

// 手动校验
const validateForm = () => {
  const fields = [
    { value: companyName.value, name: '单位名称' },
    { value: webUrl.value, name: '网站地址' },
    { value: systemName.value, name: '系统名称' },
    { value: vulnName.value, name: '漏洞名称' },
    { value: vulnUrl.value, name: '漏洞URL' },
    { value: vulnDescription.value.trim(), name: '漏洞描述' },
    { value: fixSuggestion.value.trim(), name: '修复建议' },
    { value: assistImage.value, name: '资产截图' },
    // testProcess 可选，你可以加回去
  ]

  for (const f of fields) {
    if (!f.value) {
      throw new Error(`${f.name} 不能为空`)
    }
  }
}

// 生成报告（唯一会弹窗的地方）
const generateReport = async () => {
  try {
    validateForm()
    generating.value = true
    console.log('正在生成报告，字段值：', {
      companyName: companyName.value,
      webUrl: webUrl.value,
      findDate: findDate.value.format('YYYY-MM-DD'),
      systemName: systemName.value,
      vulnName: vulnName.value,
      vulnUrl: vulnUrl.value,
      vulnDescription: vulnDescription.value,
      fixSuggestion: fixSuggestion.value,
      affectsType: affectsType.value,
      assistImage: assistImage.value,
      testProcess: testProcess.value,
      appendix: appendix.value,
    })
    const reportPath: string = await invoke('generate_report', {
      companyName: companyName.value.trim(),
      webUrl: webUrl.value.trim(),
      findDate: findDate.value.format('YYYY-MM-DD'),
      systemName: systemName.value.trim(),
      vulnName: vulnName.value,
      vulnUrl: vulnUrl.value.trim(),
      vulnDescription: vulnDescription.value,
      fixSuggestion: fixSuggestion.value,
      affectsType: affectsType.value, // 100% 拿得到
      assistImage: assistImage.value,
      testProcess: testProcess.value,
      appendix: appendix.value,
    })

    notification.success({
      message: '报告生成成功！',
      description: reportPath,
      duration: 10,
    })

    // 重置所有字段
    companyName.value = ''
    webUrl.value = ''
    findDate.value = dayjs()
    systemName.value = ''
    vulnName.value = undefined
    vulnUrl.value = ''
    vulnDescription.value = ''
    fixSuggestion.value = ''
    affectsType.value = ''
    assistImage.value = ''
    testProcess.value = ''
    appendix.value = ''
    ;(assistUploader.value as any)?.resetContent?.()
    ;(testProcessUploader.value as any)?.resetContent?.()

  } catch (err: any) {
    notification.error({
      message: '生成报告失败',
      description: err.message || '请检查必填项',
      duration: 8,
    })
  } finally {
    generating.value = false
  }
}

// 子组件 ref（用于重置）
const assistUploader = ref<any>(null)
const testProcessUploader = ref<any>(null)
</script>

<style scoped>
.form-item {
  margin-bottom: 24px;
}
label {
  display: block;
  margin-bottom: 8px;
  font-weight: 500;
}
.ant-form-item-required::before {
  display: inline-block;
  margin-right: 4px;
  color: #ff4d4f;
  font-size: 14px;
  font-family: SimSun, sans-serif;
  line-height: 1;
  content: "*";
}
</style>