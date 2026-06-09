import { ref, onMounted } from 'vue'
import { load } from '@tauri-apps/plugin-store'
import type { ModelConfig, Preset, PresetStore } from '../types'

const STORE_FILE = 'presets.json'

export function usePresets() {
	const presets = ref<Preset[]>([])
	const activePresetId = ref<string | null>(null)
	const loading = ref(false)

	async function loadPresets() {
		loading.value = true
		try {
			const store = await load(STORE_FILE, { defaults: {}, autoSave: false })
			const data = await store.get<PresetStore>('data')
			if (data) {
				presets.value = data.presets
				activePresetId.value = data.active_preset_id
			}
		} catch {
			presets.value = []
			activePresetId.value = null
		} finally {
			loading.value = false
		}
	}

	async function persistStore() {
		const store = await load(STORE_FILE, { defaults: {}, autoSave: false })
		const data: PresetStore = {
			presets: presets.value,
			active_preset_id: activePresetId.value
		}
		await store.set('data', data)
		await store.save()
	}

	async function addPreset(name: string, config: ModelConfig) {
		const id = crypto.randomUUID()
		presets.value.push({ id, name, config })
		activePresetId.value = id
		await persistStore()
	}

	async function deletePreset(id: string) {
		presets.value = presets.value.filter(p => p.id !== id)
		if (activePresetId.value === id) {
			activePresetId.value = null
		}
		await persistStore()
	}

	async function applyPreset(id: string): Promise<ModelConfig | null> {
		const preset = presets.value.find(p => p.id === id)
		if (!preset) return null
		activePresetId.value = id
		await persistStore()
		return preset.config
	}

	onMounted(loadPresets)

	return { presets, activePresetId, loading, addPreset, deletePreset, applyPreset }
}
