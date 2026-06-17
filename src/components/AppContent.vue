<script setup lang="ts">
import { ref, onMounted, nextTick } from 'vue'
import { useMessage } from 'naive-ui'
import ConfigListMain from './ConfigListMain.vue'
import ProviderForm from './ProviderForm.vue'
import PasteConfigDialog from './PasteConfigDialog.vue'
import SessionHistory from './SessionHistory.vue'
import { useProviders } from '../composables/useProviders'
import { parseSettingsEnv } from '../utils/parseConfig'
import type { Provider } from '../types'
import type { ParsedConfig } from '../utils/parseConfig'
import { invoke } from '@tauri-apps/api/core'

const message = useMessage()
const {
	providers,
	loading,
	loadProviders,
	addProvider,
	updateProvider,
	deleteProvider,
	duplicateProvider,
	activateProvider,
	testConnection
} = useProviders()

const activePage = ref<'config' | 'sessions'>('config')
const currentView = ref<'list' | 'form'>('list')
const editingProvider = ref<Provider | null>(null)
const initialData = ref<ParsedConfig | null>(null)
const showPasteDialog = ref(false)

function handleAdd() {
	editingProvider.value = null
	initialData.value = null
	currentView.value = 'form'
}

function handleEdit(provider: Provider) {
	editingProvider.value = provider
	initialData.value = null
	currentView.value = 'form'
}

function handlePaste() {
	showPasteDialog.value = true
}

async function handlePasteConfirm(config: ParsedConfig) {
	showPasteDialog.value = false
	editingProvider.value = null
	currentView.value = 'form'
	await nextTick()
	initialData.value = config
}

async function handleReadCurrent() {
	try {
		const config = await invoke<{
			auth_token: string
			base_url: string
			model: string
			reasoning_model: string
			haiku_id: string
			sonnet_id: string
			opus_id: string
		}>('read_model_config')

		const env: Record<string, string> = {
			ANTHROPIC_AUTH_TOKEN: config.auth_token,
			ANTHROPIC_BASE_URL: config.base_url,
			ANTHROPIC_MODEL: config.model,
			ANTHROPIC_DEFAULT_HAIKU_MODEL: config.haiku_id,
			ANTHROPIC_DEFAULT_SONNET_MODEL: config.sonnet_id,
			ANTHROPIC_DEFAULT_OPUS_MODEL: config.opus_id,
			ANTHROPIC_REASONING_MODEL: config.reasoning_model
		}

		const parsed = parseSettingsEnv(env)
		editingProvider.value = null
		currentView.value = 'form'
		await nextTick()
		initialData.value = parsed
	} catch (e) {
		message.error(`读取配置失败: ${String(e)}`)
	}
}

function handleBack() {
	currentView.value = 'list'
	editingProvider.value = null
	initialData.value = null
}

async function handleSave(formData: Omit<Provider, 'id' | 'is_active'>) {
	try {
		if (editingProvider.value) {
			await updateProvider({ ...editingProvider.value, ...formData })
			message.success('供应商已更新')
		} else {
			await addProvider(formData)
			message.success('供应商已添加')
		}
		currentView.value = 'list'
		editingProvider.value = null
		initialData.value = null
	} catch (e) {
		message.error(String(e))
	}
}

async function handleDelete(id: string) {
	const provider = providers.value.find(p => p.id === id)
	if (provider?.is_active) {
		message.warning('不能删除当前激活的供应商')
		return
	}
	try {
		await deleteProvider(id)
		message.success('供应商已删除')
	} catch (e) {
		message.error(String(e))
	}
}

async function handleDuplicate(id: string) {
	try {
		const duplicate = await duplicateProvider(id)
		if (duplicate) {
			editingProvider.value = duplicate
			initialData.value = null
			currentView.value = 'form'
			message.success('供应商已复制')
		}
	} catch (e) {
		message.error(String(e))
	}
}

async function handleActivate(provider: Provider) {
	try {
		await activateProvider(provider)
		message.success(`${provider.name} 已激活`)
	} catch (e) {
		message.error(String(e))
	}
}

async function handleTest(api_key: string, base_url: string) {
	return await testConnection(api_key, base_url)
}

function handleSession() {
	activePage.value = 'sessions'
}

onMounted(loadProviders)
</script>

<template>
	<div class="flex flex-col w-full h-full bg-white font-sans">
		<ConfigListMain
			v-if="activePage === 'config' && currentView === 'list'"
			:providers="providers"
			:loading="loading"
			@add="handleAdd"
			@paste="handlePaste"
			@read-current="handleReadCurrent"
			@edit="handleEdit"
			@duplicate="handleDuplicate"
			@delete="handleDelete"
			@activate="handleActivate"
			@session="handleSession"
		/>
		<ProviderForm
			v-else-if="activePage === 'config' && currentView === 'form'"
			:provider="editingProvider"
			:initial-data="initialData"
			@save="handleSave"
			@cancel="handleBack"
			@test="handleTest"
		/>
		<SessionHistory v-else-if="activePage === 'sessions'" @back="activePage = 'config'" />
		<PasteConfigDialog v-if="showPasteDialog" @confirm="handlePasteConfirm" @cancel="showPasteDialog = false" />
	</div>
</template>
