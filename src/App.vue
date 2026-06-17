<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { NConfigProvider, NMessageProvider, useMessage } from 'naive-ui'
import ConfigListMain from './components/ConfigListMain.vue'
import ProviderForm from './components/ProviderForm.vue'
import SessionHistory from './components/SessionHistory.vue'
import { useProviders } from './composables/useProviders'
import type { Provider } from './types'

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
const sessionDetailView = ref(false)
const sessionPageActive = computed(() => activePage.value === 'sessions')

function handleAdd() {
	editingProvider.value = null
	currentView.value = 'form'
}

function handleEdit(provider: Provider) {
	editingProvider.value = provider
	currentView.value = 'form'
}

function handleBack() {
	currentView.value = 'list'
	editingProvider.value = null
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

function handleSessionDetailChange(isDetail: boolean) {
	sessionDetailView.value = isDetail
}

onMounted(loadProviders)
</script>

<template>
	<n-config-provider>
		<n-message-provider>
			<div class="flex flex-col w-full h-full bg-white font-sans">
				<!-- 顶部 Tab 栏 -->
				<div
					v-if="!sessionPageActive"
					class="flex items-center h-40px px-24px bg-white border-b border-gray-100 shrink-0 gap-4px"
				>
					<button
						class="flex items-center gap-6px px-12px py-6px rounded-6px text-13px transition-colors"
						:class="
							activePage === 'config' ? 'bg-indigo-500 text-white' : 'text-gray-600 hover:bg-gray-100'
						"
						@click="activePage = 'config'"
					>
						Config
					</button>
					<button
						class="flex items-center gap-6px px-12px py-6px rounded-6px text-13px transition-colors"
						:class="
							activePage === 'sessions' ? 'bg-indigo-500 text-white' : 'text-gray-600 hover:bg-gray-100'
						"
						@click="activePage = 'sessions'"
					>
						Sessions
					</button>
				</div>

				<!-- 内容区 -->
				<div class="flex-1 overflow-hidden">
					<ConfigListMain
						v-if="activePage === 'config' && currentView === 'list'"
						:providers="providers"
						:loading="loading"
						@add="handleAdd"
						@edit="handleEdit"
						@duplicate="handleDuplicate"
						@delete="handleDelete"
						@activate="handleActivate"
					/>
					<ProviderForm
						v-else-if="activePage === 'config' && currentView === 'form'"
						:provider="editingProvider"
						@save="handleSave"
						@cancel="handleBack"
						@test="handleTest"
					/>
					<SessionHistory
						v-else-if="activePage === 'sessions'"
						@detail-change="handleSessionDetailChange"
						@back="activePage = 'config'"
					/>
				</div>
			</div>
		</n-message-provider>
	</n-config-provider>
</template>
