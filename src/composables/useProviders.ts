import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Provider } from '../types'

export function useProviders() {
	const providers = ref<Provider[]>([])
	const loading = ref(false)
	const error = ref<string | null>(null)

	async function loadProviders() {
		loading.value = true
		error.value = null
		try {
			const result = await invoke<Provider[]>('list_providers')
			providers.value = result
		} catch (e) {
			error.value = String(e)
		} finally {
			loading.value = false
		}
	}

	async function saveProviders() {
		loading.value = true
		error.value = null
		try {
			await invoke('save_providers', { providers: providers.value })
		} catch (e) {
			error.value = String(e)
		} finally {
			loading.value = false
		}
	}

	async function addProvider(provider: Omit<Provider, 'id' | 'is_active'>) {
		const newProvider: Provider = {
			...provider,
			id: crypto.randomUUID(),
			is_active: false
		}
		providers.value.push(newProvider)
		await saveProviders()
		return newProvider
	}

	async function updateProvider(updated: Provider) {
		const index = providers.value.findIndex(p => p.id === updated.id)
		if (index !== -1) {
			providers.value[index] = updated
			await saveProviders()
		}
	}

	async function deleteProvider(id: string) {
		providers.value = providers.value.filter(p => p.id !== id)
		await saveProviders()
	}

	async function duplicateProvider(id: string) {
		const source = providers.value.find(p => p.id === id)
		if (!source) return null
		const duplicate: Provider = {
			...source,
			id: crypto.randomUUID(),
			name: `${source.name}(副本)`,
			is_active: false
		}
		providers.value.push(duplicate)
		await saveProviders()
		return duplicate
	}

	async function activateProvider(provider: Provider) {
		providers.value.forEach(p => {
			p.is_active = p.id === provider.id
		})
		await invoke('activate_provider', { provider })
		await saveProviders()
	}

	async function testConnection(api_key: string, base_url: string) {
		try {
			const result = await invoke<string>('test_connection', { apiKey: api_key, baseUrl: base_url })
			return { success: true, message: result }
		} catch (e) {
			return { success: false, message: String(e) }
		}
	}

	return {
		providers,
		loading,
		error,
		loadProviders,
		saveProviders,
		addProvider,
		updateProvider,
		deleteProvider,
		duplicateProvider,
		activateProvider,
		testConnection
	}
}
