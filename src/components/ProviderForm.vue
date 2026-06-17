<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { NInput, NSelect, NAlert } from 'naive-ui'
import { invoke } from '@tauri-apps/api/core'
import type { Provider } from '../types'
import type { ParsedConfig } from '../utils/parseConfig'

const props = defineProps<{
	provider?: Provider | null
	initialData?: ParsedConfig | null
}>()

const emit = defineEmits<{
	save: [provider: Omit<Provider, 'id' | 'is_active'>]
	cancel: []
}>()

const form = ref({
	name: '',
	notes: '',
	website: '',
	api_key: '',
	base_url: '',
	main_model: 'sonnet',
	opus_model: '',
	sonnet_model: '',
	haiku_model: '',
	sub_agent_model: 'haiku',
	reasoning_level: 'max'
})

const testing = ref(false)
const testResult = ref<{ success: boolean; message: string } | null>(null)
const saveAttempted = ref(false)

// ── 校验 ──

const urlError = computed(() => {
	const url = form.value.base_url.trim()
	if (!url) return saveAttempted.value ? '请输入请求地址' : ''
	try {
		const parsed = new URL(url)
		if (!['http:', 'https:'].includes(parsed.protocol)) return '仅支持 http/https 协议'
		return ''
	} catch {
		return 'URL 格式不合法'
	}
})

const nameError = computed(() => (saveAttempted.value && !form.value.name.trim() ? '请输入供应商名称' : ''))
const apiKeyError = computed(() => (saveAttempted.value && !form.value.api_key.trim() ? '请输入 API Key' : ''))
const opusModelError = computed(() => (saveAttempted.value && !form.value.opus_model.trim() ? '请输入模型 ID' : ''))
const sonnetModelError = computed(() => (saveAttempted.value && !form.value.sonnet_model.trim() ? '请输入模型 ID' : ''))
const haikuModelError = computed(() => (saveAttempted.value && !form.value.haiku_model.trim() ? '请输入模型 ID' : ''))

const validationErrors = computed(() => {
	if (!saveAttempted.value) return []
	const errors: string[] = []
	if (!form.value.name.trim()) errors.push('供应商名称')
	if (!form.value.api_key.trim()) errors.push('API Key')
	if (!form.value.base_url.trim() || urlError.value) errors.push('请求地址')
	if (!form.value.opus_model.trim()) errors.push('Opus 模型')
	if (!form.value.sonnet_model.trim()) errors.push('Sonnet 模型')
	if (!form.value.haiku_model.trim()) errors.push('Haiku 模型')
	return errors
})

// ── URL 自动解析 ──

function extractNameFromHost(host: string): string {
	// api.deepseek.com → DeepSeek, api.openai.com → Openai
	const parts = host.split('.')
	// 取倒数第二段作为名称（跳过 api/www 等前缀和 com/net 等后缀）
	const candidates = parts.filter(p => !['api', 'www', 'com', 'net', 'org', 'io', 'cn'].includes(p))
	const raw = candidates[0] || parts[0] || ''
	return raw.charAt(0).toUpperCase() + raw.slice(1)
}

let lastAutoName = ''
let lastAutoWebsite = ''

watch(
	() => form.value.base_url,
	url => {
		const trimmed = url.trim()
		if (!trimmed) return
		try {
			const parsed = new URL(trimmed)
			if (!['http:', 'https:'].includes(parsed.protocol)) return
			const website = `${parsed.protocol}//${parsed.hostname}`
			const name = extractNameFromHost(parsed.hostname)
			// 仅在 name/website 为空或之前是自动填充值时覆盖
			if (!form.value.name || form.value.name === lastAutoName) {
				form.value.name = name
				lastAutoName = name
			}
			if (!form.value.website || form.value.website === lastAutoWebsite) {
				form.value.website = website
				lastAutoWebsite = website
			}
		} catch {
			// URL 不合法时不处理
		}
	}
)

const modelOptions = [
	{ label: 'Haiku', value: 'haiku' },
	{ label: 'Sonnet', value: 'sonnet' },
	{ label: 'Opus', value: 'opus' }
]

const reasoningOptions = [
	{ label: 'Low', value: 'low' },
	{ label: 'Medium', value: 'medium' },
	{ label: 'High', value: 'high' },
	{ label: 'Max', value: 'max' },
	{ label: 'XHigh', value: 'xhigh' }
]

function populateForm(data: Omit<Provider, 'id' | 'is_active'>) {
	form.value = {
		name: data.name || '',
		notes: data.notes || '',
		website: data.website || '',
		api_key: data.api_key || '',
		base_url: data.base_url || '',
		main_model: data.main_model || 'sonnet',
		opus_model: data.opus_model || '',
		sonnet_model: data.sonnet_model || '',
		haiku_model: data.haiku_model || '',
		sub_agent_model: data.sub_agent_model || 'haiku',
		reasoning_level: data.reasoning_level || 'max'
	}
	saveAttempted.value = false
	lastAutoName = data.name || ''
	lastAutoWebsite = data.website || ''
}

watch(
	() => props.initialData,
	d => {
		if (d) populateForm(d)
	},
	{ immediate: true }
)

watch(
	() => props.provider,
	p => {
		if (p) populateForm(p)
	},
	{ immediate: true }
)

function handleSave() {
	saveAttempted.value = true
	if (
		nameError.value ||
		apiKeyError.value ||
		urlError.value ||
		opusModelError.value ||
		sonnetModelError.value ||
		haikuModelError.value
	)
		return
	emit('save', { ...form.value })
}

async function handleTest() {
	if (!form.value.base_url || urlError.value) return
	testing.value = true
	testResult.value = null
	try {
		const result = await invoke<string>('test_connection', {
			baseUrl: form.value.base_url
		})
		testResult.value = { success: true, message: result }
	} catch (e) {
		testResult.value = { success: false, message: String(e) }
	} finally {
		testing.value = false
	}
}
</script>

<template>
	<div class="flex flex-col w-full h-full bg-white font-sans">
		<!-- 头部 -->
		<header class="flex items-center justify-between h-56px px-24px bg-white border-b border-gray-100 shrink-0">
			<div class="flex items-center gap-12px">
				<button
					class="flex items-center justify-center w-32px h-32px border-none bg-gray-100 rounded-6px cursor-pointer hover:bg-gray-200 transition-colors"
					@click="emit('cancel')"
				>
					<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#666666" stroke-width="2">
						<path d="M19 12H5M12 19l-7-7 7-7" />
					</svg>
				</button>
				<span class="text-20px font-700 text-gray-900">
					{{ provider ? '编辑供应商' : '添加供应商' }}
				</span>
			</div>
			<!-- <button
				class="text-14px text-gray-500 bg-transparent border-none cursor-pointer hover:text-gray-700"
				@click="emit('cancel')"
			>
				取消
			</button> -->
		</header>

		<!-- 表单内容 -->
		<div class="flex-1 overflow-y-auto px-32px py-24px">
			<div class="max-w-640px mx-auto flex flex-col gap-24px">
				<!-- 校验汇总 -->
				<n-alert v-if="validationErrors.length" type="error">
					请填写以下必填项：{{ validationErrors.join('、') }}
				</n-alert>
				<!-- 基本信息 -->
				<section class="flex flex-col gap-16px">
					<h3 class="text-16px font-600 text-gray-900 m-0">基本信息</h3>

					<div class="flex flex-col gap-6px">
						<label class="text-13px text-gray-600 font-500">供应商名称</label>
						<n-input
							v-model:value="form.name"
							placeholder="例如：DeepSeek"
							:validation-status="nameError ? 'error' : undefined"
							:feedback="nameError || undefined"
						/>
					</div>

					<div class="flex flex-col gap-6px">
						<label class="text-13px text-gray-600 font-500">备注</label>
						<n-input v-model:value="form.notes" placeholder="可选备注信息" />
					</div>

					<div class="flex flex-col gap-6px">
						<label class="text-13px text-gray-600 font-500">官网链接</label>
						<n-input v-model:value="form.website" placeholder="https://platform.example.com" />
					</div>
				</section>

				<div class="h-1px bg-gray-200" />

				<!-- API 配置 -->
				<section class="flex flex-col gap-16px">
					<h3 class="text-16px font-600 text-gray-900 m-0">API 配置</h3>

					<div class="flex flex-col gap-6px">
						<label class="text-13px text-gray-600 font-500">API Key</label>
						<n-input
							v-model:value="form.api_key"
							type="password"
							show-password-on="click"
							placeholder="sk-xxxxxxxxxxxxxxxx"
							:validation-status="apiKeyError ? 'error' : undefined"
							:feedback="apiKeyError || undefined"
						/>
					</div>

					<div class="flex flex-col gap-6px">
						<label class="text-13px text-gray-600 font-500">请求地址</label>
						<n-input
							v-model:value="form.base_url"
							placeholder="https://api.example.com/anthropic"
							:validation-status="urlError ? 'error' : undefined"
							:feedback="urlError || undefined"
						/>
					</div>
				</section>

				<div class="h-1px bg-gray-200" />

				<!-- 模型配置 -->
				<section class="flex flex-col gap-16px">
					<div>
						<h3 class="text-16px font-600 text-gray-900 m-0">模型配置</h3>
						<p class="text-13px text-gray-500 m-0 mt-4px">配置不同场景使用的模型</p>
					</div>

					<div class="grid grid-cols-2 gap-16px">
						<div class="flex flex-col gap-6px">
							<label class="text-13px text-gray-600 font-500">主模型</label>
							<n-select v-model:value="form.main_model" :options="modelOptions" />
						</div>
						<div class="flex flex-col gap-6px">
							<label class="text-13px text-gray-600 font-500">Opus 模型</label>
							<n-input
								v-model:value="form.opus_model"
								placeholder="模型 ID"
								:validation-status="opusModelError ? 'error' : undefined"
								:feedback="opusModelError || undefined"
							/>
						</div>
						<div class="flex flex-col gap-6px">
							<label class="text-13px text-gray-600 font-500">Sonnet 模型</label>
							<n-input
								v-model:value="form.sonnet_model"
								placeholder="模型 ID"
								:validation-status="sonnetModelError ? 'error' : undefined"
								:feedback="sonnetModelError || undefined"
							/>
						</div>
						<div class="flex flex-col gap-6px">
							<label class="text-13px text-gray-600 font-500">Haiku 模型</label>
							<n-input
								v-model:value="form.haiku_model"
								placeholder="模型 ID"
								:validation-status="haikuModelError ? 'error' : undefined"
								:feedback="haikuModelError || undefined"
							/>
						</div>
						<div class="flex flex-col gap-6px">
							<label class="text-13px text-gray-600 font-500">子代理模型</label>
							<n-select v-model:value="form.sub_agent_model" :options="modelOptions" />
						</div>
						<div class="flex flex-col gap-6px">
							<label class="text-13px text-gray-600 font-500">推理强度</label>
							<n-select v-model:value="form.reasoning_level" :options="reasoningOptions" />
						</div>
					</div>
				</section>

				<div class="h-1px bg-gray-200" />

				<!-- 测试结果提示 -->
				<n-alert v-if="testResult" :type="testResult.success ? 'success' : 'error'" class="mt-8px">
					{{ testResult.message }}
				</n-alert>

				<!-- 操作栏 -->
				<div class="flex items-center justify-end gap-12px py-16px border-t border-gray-200">
					<button
						class="flex items-center gap-8px px-20px py-10px rounded-8px border border-gray-200 bg-white text-14px font-500 text-gray-700 cursor-pointer hover:bg-gray-50 transition-colors"
						:disabled="testing"
						@click="handleTest"
					>
						<svg
							width="16"
							height="16"
							viewBox="0 0 24 24"
							fill="none"
							stroke="#6B7280"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
						</svg>
						测试连接
					</button>
					<button
						class="flex items-center gap-8px px-24px py-10px rounded-8px border-none bg-[#FF6B35] text-14px font-600 text-white cursor-pointer hover:opacity-90 transition-opacity"
						@click="handleSave"
					>
						<svg
							width="16"
							height="16"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<polyline points="20 6 9 17 4 12" />
						</svg>
						保存配置
					</button>
				</div>
			</div>
		</div>
	</div>
</template>
