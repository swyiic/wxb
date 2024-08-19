<template>
    <a-form :model="formState" ref="formRef" :validateTrigger="['change', 'blur']" autocomplete="off" @finish="onFinish"
        @finishFailed="onFinishFailed">
        <a-row :gutter="24" justify="center">
            <a-col :span="1"></a-col>
            <a-col :span="11" flex="1">
                <a-form-item name="company_name" v-bind="validateInfos.company_name">
                    <label>单位名称</label>
                    <a-input v-model:value="formState.company_name" placeholder="单位名称" allow-clear />
                </a-form-item>
            </a-col>
            <a-col :span="11" flex="1">
                <a-form-item name="web_url" v-bind="validateInfos.web_url">
                    <label>网站地址</label>
                    <a-input v-model:value="formState.web_url" placeholder="网站URL" allow-clear />
                </a-form-item>
            </a-col>
            <a-col :span="1"></a-col>
        </a-row>

        <a-row :gutter="24" justify="center">
            <a-col :span="1"></a-col>
            <a-col :span="11" flex="1">
                <a-form-item :rules="[{ required: true, message: '必填项！' }]">
                    <label>发现时间</label>
                    <a-date-picker v-model:value="dateValue" format="YYYY-MM-DD" />
                </a-form-item>
            </a-col>
            <a-col :span="11" flex="1">
                <a-form-item name="system_name" v-bind="validateInfos.system_name">
                    <label>系统名称</label>
                    <a-input v-model:value="formState.system_name" placeholder="系统名称" allow-clear />
                </a-form-item>
            </a-col>
            <a-col :span="1"></a-col>
        </a-row>

        <a-row :gutter="24" justify="center">
            <a-col :span="1"></a-col>
            <a-col :span="11" flex="1">
                <a-form-item name="vuln_name" v-bind="validateInfos.vuln_name">
                    <label>漏洞名称</label>
                    <a-select v-model:value="formState.vuln_name" :options="vulnOptions"
                        @dropdownVisibleChange="handleDropdownVisibleChange" @change="handleVulnChange" show-search
                        allow-clear></a-select>
                </a-form-item>
            </a-col>
            <a-col :span="11" flex="1">
                <a-form-item name="vuln_url" v-bind="validateInfos.vuln_url">
                    <label>漏洞URL</label>
                    <a-input v-model:value="formState.vuln_url" placeholder="如果过长导致单元格样式损害则写在附录中" allow-clear />
                </a-form-item>
            </a-col>
            <a-col :span="1"></a-col>
        </a-row>

        <a-row :gutter="24" justify="center">
            <a-col :span="1"></a-col>
            <a-col :span="22" flex="1">
                <a-form-item name="assist_image" :rules="[{ required: true, message: '必填项！' }]">
                    <label>资产截图</label>
                    <AssistImageUploader ref="assistUploader" :content="formState.assist_image"
                        @updateAssistContent="updateAssistImage" />
                </a-form-item>
            </a-col>
            <a-col :span="1"></a-col>
        </a-row>
        <a-row :gutter="24" justify="center">
            <a-col :span="1"></a-col>
            <a-col :span="22" flex="1">
                <a-form-item name="test_process">
                    <label>测试过程</label>
                    <TestProcessImageUploader ref="testProcessUploader" :content="formState.test_process"
                        @updateTestProcessContent="updateTestProcess" />
                </a-form-item>
            </a-col>
            <a-col :span="1"></a-col>
        </a-row>

        <a-row :gutter="24" justify="center">
            <a-col :span="1"></a-col>
            <a-col :span="22" flex="1">
                <a-form-item name="vuln_description" v-bind="validateInfos.vuln_description">
                    <label>漏洞描述</label>
                    <a-textarea v-model:value="formState.vuln_description" placeholder="根据下拉自动获取" allow-clear />
                </a-form-item>
            </a-col>
            <a-col :span="1"></a-col>
        </a-row>

        <a-row :gutter="24" justify="center">
            <a-col :span="1"></a-col>
            <a-col :span="22" flex="1">
                <a-form-item name="fix_suggestion" v-bind="validateInfos.fix_suggestion">
                    <label>修复建议</label>
                    <a-textarea v-model:value="formState.fix_suggestion" placeholder="根据下拉自动获取" allow-clear />
                </a-form-item>
            </a-col>
            <a-col :span="1"></a-col>
        </a-row>
        <a-row :gutter="24" justify="center">
            <a-col :span="1"></a-col>
            <a-col :span="12" flex="1">
                <a-form-item>
                    <a-button type="dashed" @click="showDrawer">添加数据</a-button>
                    <a-drawer :width="500" title="添加详细数据" :placement="placement" :open="open" @close="onClose">
                        <textarea v-model="formState.appendix" :rows="10" :cols="50"
                            placeholder="BurpSuite数据包粘贴在这里"></textarea>
                        <template #extra>
                            <a-button style="margin-right: 8px" @click="onClose">取消</a-button>
                            <a-button type="primary" @click="onDateSave">保存</a-button>
                        </template>
                    </a-drawer>
                </a-form-item>
            </a-col>
            <a-col :span="5" flex="1">
                <a-form-item>
                    <a-button type="primary" @click="generateReport">生成报告</a-button>
                </a-form-item>
            </a-col>
            <a-col :span="5" flex="1">
                <a-form-item>
                    <a-button danger @click="open_platform">打开平台</a-button>
                </a-form-item>
            </a-col>
            <a-col :span="1"></a-col>
        </a-row>
    </a-form>
</template>

<script setup lang="ts">
import type { DrawerProps } from 'ant-design-vue'
import { FormInstance, Form, notification } from 'ant-design-vue';
import { ref, reactive } from 'vue'
import dayjs, { Dayjs } from 'dayjs'
import AssistImageUploader from '../components/AssistImageUploader.vue';
import TestProcessImageUploader from '../components/TestProcessImageUploader.vue';
import { eventBus } from '../eventBus';
import { invoke } from '@tauri-apps/api/tauri';

// 获取当前日期
const dateFormat = 'YYYY-MM-DD'
const getCurrentDate = (): string => {
    const now = new Date()
    const year = now.getFullYear()
    // 手动格式化月份和日期
    const month =
        (now.getMonth() + 1).toString().length === 1
            ? `0${now.getMonth() + 1}`
            : (now.getMonth() + 1).toString()

    const day = now.getDate().toString().length === 1 ? `0${now.getDate()}` : now.getDate().toString()

    return `${year}-${month}-${day}`
}
const dateValue = ref<Dayjs>(dayjs(getCurrentDate(), dateFormat));
// 定义结构
interface FormItem {
    company_name: string;
    web_url: string;
    find_date: string;
    system_name: string;
    vuln_name: string;
    vuln_url: string;
    vuln_description: string;
    fix_suggestion: string;
    assist_image: string;
    test_process: string;
    appendix: string;
}
// 获取结构的值状态
const formState = reactive<FormItem>({
    company_name: '',
    web_url: '',
    find_date: '',
    system_name: '',
    vuln_name: '',
    vuln_url: '',
    vuln_description: '',
    fix_suggestion: '',
    assist_image: '',
    test_process: '',
    appendix: '',
});
// 全局监听事件
const formRef = ref<FormInstance>();
const onFinish = () => {
    formRef.value?.validate().then(() => {
        console.log('Success:', formState);
    }).catch((errorInfo) => {
        console.log('Failed:', errorInfo);
        alert('请填写所有必填项！');
    });
};
const onFinishFailed = (errorInfo: any) => {
    console.log('保存失败，存在必填项为空:', errorInfo);
};

// 漏洞相关
interface VulnerabilityDetails {
    id: number;
    vuln_name: string;
    vuln_description: string;
    fix_suggestion: string;
}

const vulnOptions = ref<{ label: string; value: string }[]>([]);
const handleDropdownVisibleChange = async (open: boolean) => {
    if (open) {
        console.log("检测到下拉动作, 发送指令...");
        try {
            const vulnerabilities = (await invoke('query_all_vulnerability')) as VulnerabilityDetails[];
            console.log('Data received:', vulnerabilities);
            if (vulnerabilities && vulnerabilities.length > 0) {
                vulnOptions.value = vulnerabilities.map((vuln: VulnerabilityDetails) => ({
                    label: vuln.vuln_name,
                    value: vuln.vuln_name,
                }));
                console.log('下拉值为', vulnOptions);
            } else {
                console.warn('未查询到该漏洞');
            }
        } catch (error) {
            console.error('查询漏洞异常:', error);
        }
    }
};
const handleVulnChange = async (selectedName: string) => {
    try {
        // 根据选择的漏洞名称从后端获取详细信息
        const detailsArray = (await invoke('search_vulnerability_by_name', { query: selectedName })) as VulnerabilityDetails[];
        if (detailsArray.length > 0) {
            const details = detailsArray[0];  // 提取数组中的第一个元素
            console.log('选中项全部数据为：', details)
            formState.vuln_name = details.vuln_name;
            formState.vuln_description = details.vuln_description;
            formState.fix_suggestion = details.fix_suggestion;
        } else {
            console.warn('No vulnerabilities found');
        }
    } catch (error) {
        console.error('Error fetching vulnerability details:', error);
    }
};

//截图文字相关
const updateAssistImage = (content: string) => {
    formState.assist_image = content;
};
const updateTestProcess = (content: string) => {
    formState.test_process = content;
};

// 添加数据抽屉
const placement = ref<DrawerProps['placement']>('right')
const open = ref<boolean>(false)
const showDrawer = () => {
    open.value = true
}
const onClose = () => {
    open.value = false
}
const onDateSave = () => {
    console.log(formState);
    console.log('Saved content:', formState.appendix);
    open.value = false
}

//打开平台tab
const open_platform = () => {
    eventBus.emit('open-platform');
};

const useForm = Form.useForm;
const { validate, validateInfos, resetFields } = useForm(
    formState,
    reactive({
        company_name: [
            {
                required: true,
                message: '需要完善数据！',
            },
        ],
        web_url: [
            {
                required: true,
                message: '需要完善数据！',
            },
        ],
        system_name: [
            {
                required: true,
                message: '需要完善数据！',
            },
        ],
        vuln_name: [
            {
                required: true,
                message: '需要完善数据！',
            },
        ],
        vuln_url: [
            {
                required: true,
                message: '需要完善数据！',
            },
        ],
        vuln_description: [
            {
                required: true,
                message: '需要完善数据！',
            },
        ],
        fix_suggestion: [
            {
                required: true,
                message: '需要完善数据！',
            },
        ],
        assist_image: [
            {
                required: true,
                message: '需要完善数据！',
            },
        ],
        test_process: [
            {
                required: true,
                message: '需要完善数据！',
            },
        ]
    }),
);
const testProcessUploader = ref<{ resetContent: () => void } | null>(null);
const assistUploader = ref<{ resetContent: () => void } | null>(null);
const resetForm = () => {
    resetFields(); // 重置表单数据和验证状态
    formState.company_name = '';
    formState.web_url = '';
    dateValue.value = dayjs(getCurrentDate(), dateFormat); // 重置日期
    formState.system_name = '';
    formState.vuln_name = '';
    formState.vuln_url = '';
    formState.vuln_description = '';
    formState.fix_suggestion = '';
    formState.appendix = '';
    vulnOptions.value = []; // 重置漏洞选项

    formState.assist_image = '';
    formState.test_process = '';

    // 调用子组件的 `resetContent` 方法清空内容
    if (testProcessUploader.value) {
        testProcessUploader.value.resetContent();
    }
    // 调用子组件的 `resetContent` 方法清空内容
    if (assistUploader.value) {
        assistUploader.value.resetContent();
    }

};
const openNotificationWithIconSuccess = (type: 'success' | 'info' | 'warning' | 'error') => {
    notification[type]({
        message: '创建报告成功，请在程序同目录查看'
    });
};
const openNotificationWithIconFail = (type: 'success' | 'info' | 'warning' | 'error') => {
    notification[type]({
        message: '报告生成异常，请核查数据是否完整'
    });
};
const selectDate = dateValue.value.format('YYYY-MM-DD');
// 调用后端的 generate_report 命令
const generateReport = async () => {
    console.log('全部数据：', formState);
    try {
        // 使用 await 直接等待 validate 的执行结果
        await validate();

        // 如果 validate 成功，继续执行报告生成逻辑
        await invoke('generate_report', {
            companyName: formState.company_name,
            webUrl: formState.web_url,
            findDate: selectDate,
            systemName: formState.system_name,
            vulnName: formState.vuln_name,
            vulnUrl: formState.vuln_url,
            vulnDescription: formState.vuln_description,
            fixSuggestion: formState.fix_suggestion,
            assistImage: formState.assist_image,
            testProcess: formState.test_process,
            appendix: formState.appendix,
        });

        // 成功后显示通知并重置表单
        openNotificationWithIconSuccess('success');
        resetForm();

    } catch (error) {
        // 如果验证失败或报告生成失败，处理异常
        console.log('验证失败或报告生成失败:', error);
        openNotificationWithIconFail('error');
    }
};



</script>

<style scoped>
.input-field {
    width: 100%;
    /* 使宽度自适应父容器 */
}

.column {
    margin: 0 10px;
    /* 可根据需要调整列间距 */
    flex: 1;
    /* 使列自适应宽度 */
}

.row {
    display: flex;
    margin-bottom: 10px;
}

label {
    display: block;
    margin-bottom: 5px;
}

input[type='text'],
select,
textarea {
    width: 100%;
    padding: 8px;
    box-sizing: border-box;
    border: 1px solid #ccc;
    border-radius: 4px;
}

textarea {
    resize: vertical;
    /* 允许垂直调整大小 */
}

@media (max-width: 768px) {
    .row {
        flex-direction: column;
    }

    .column {
        margin-bottom: 10px;
    }
}
</style>