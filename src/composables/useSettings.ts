import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { ModelConfig } from '../types'

const DEFAULT_CONFIG: ModelConfig = {
	auth_token: '',
	base_url: '',
	model: '',
	reasoning_model: '',
	haiku_id: '',
	haiku_name: '',
	sonnet_id: '',
	sonnet_name: '',
	opus_id: '',
	opus_name: ''
}

export function useSettings() {
	const config = ref<ModelConfig>({ ...DEFAULT_CONFIG })
	const dirty = ref(false)
	const loading = ref(false)
	const error = ref<string | null>(null)

	async function loadConfig() {
		loading.value = true
		error.value = null
		try {
			const result = await invoke<ModelConfig>('read_model_config')
			config.value = result
		} catch (e) {
			error.value = String(e)
		} finally {
			loading.value = false
		}
	}

	async function saveConfig() {
		loading.value = true
		error.value = null
		try {
			await invoke('write_model_config', { config: config.value })
			dirty.value = false
		} catch (e) {
			error.value = String(e)
		} finally {
			loading.value = false
		}
	}

	function updateField<K extends keyof ModelConfig>(key: K, value: ModelConfig[K]) {
		config.value[key] = value
		dirty.value = true
	}

	function applyConfig(newConfig: ModelConfig) {
		config.value = { ...newConfig }
		dirty.value = true
	}

	return { config, dirty, loading, error, loadConfig, saveConfig, updateField, applyConfig }
}
