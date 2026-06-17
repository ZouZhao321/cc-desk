<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useMessage } from 'naive-ui'
import ConfigListMain from './ConfigListMain.vue'
import ProviderForm from './ProviderForm.vue'
import { useProviders } from '../composables/useProviders'
import type { Provider } from '../types'

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

const currentView = ref<'list' | 'form'>('list')
const editingProvider = ref<Provider | null>(null)

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

onMounted(loadProviders)
</script>

<template>
	<div class="flex flex-col w-full h-full bg-white font-sans">
		<ConfigListMain
			v-if="currentView === 'list'"
			:providers="providers"
			:loading="loading"
			@add="handleAdd"
			@edit="handleEdit"
			@duplicate="handleDuplicate"
			@delete="handleDelete"
			@activate="handleActivate"
		/>
		<ProviderForm v-else :provider="editingProvider" @save="handleSave" @cancel="handleBack" @test="handleTest" />
	</div>
</template>
