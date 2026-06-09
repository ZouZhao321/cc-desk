<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { NConfigProvider, NSpace, NButton, NH2, NSpin, NAlert, NMessageProvider } from 'naive-ui'
import ConfigCard from './components/ConfigCard.vue'
import PresetList from './components/PresetList.vue'
import PresetDialog from './components/PresetDialog.vue'
import { useSettings } from './composables/useSettings'
import { usePresets } from './composables/usePresets'
import type { ConfigField } from './types'

const { config, dirty, loading, error, loadConfig, saveConfig, updateField, applyConfig } = useSettings()
const { presets, activePresetId, addPreset, deletePreset, applyPreset } = usePresets()

const showDialog = ref(false)

const configFields: ConfigField[] = [
	{ key: 'auth_token', label: 'API Token', icon: '🔑', sensitive: true },
	{ key: 'base_url', label: 'Base URL', icon: '🌐', sensitive: false },
	{ key: 'model', label: 'Model', icon: '🤖', sensitive: false },
	{ key: 'reasoning_model', label: 'Reasoning Model', icon: '🧠', sensitive: false },
	{ key: 'haiku_id', label: 'Haiku ID', icon: '🟢', sensitive: false },
	{ key: 'haiku_name', label: 'Haiku Name', icon: '🟢', sensitive: false },
	{ key: 'sonnet_id', label: 'Sonnet ID', icon: '🔵', sensitive: false },
	{ key: 'sonnet_name', label: 'Sonnet Name', icon: '🔵', sensitive: false },
	{ key: 'opus_id', label: 'Opus ID', icon: '🟣', sensitive: false },
	{ key: 'opus_name', label: 'Opus Name', icon: '🟣', sensitive: false }
]

async function handleApplyPreset(id: string) {
	const presetConfig = await applyPreset(id)
	if (presetConfig) {
		applyConfig(presetConfig)
		await saveConfig()
	}
}

async function handleDeletePreset(id: string) {
	await deletePreset(id)
}

async function handleSavePreset(name: string) {
	await addPreset(name, config.value)
}

async function handleSave() {
	await saveConfig()
}

onMounted(loadConfig)
</script>

<template>
	<n-config-provider>
		<n-message-provider>
			<div class="p-6 max-w-3xl mx-auto">
				<n-space align="center" justify="space-between" class="mb-4">
					<n-h2 class="!mb-0">CC Model Switcher</n-h2>
					<n-button type="primary" :disabled="!dirty || loading" :loading="loading" @click="handleSave">
						保存
					</n-button>
				</n-space>

				<n-alert v-if="error" type="error" class="mb-4" closable>
					{{ error }}
				</n-alert>

				<n-space v-if="loading && !config.auth_token" justify="center" class="py-12">
					<n-spin size="large" />
				</n-space>

				<template v-else>
					<div class="mb-6">
						<div class="text-sm text-gray-500 mb-3">当前配置</div>
						<div class="flex flex-wrap gap-3">
							<config-card
								v-for="field in configFields"
								:key="field.key"
								:label="field.label"
								:icon="field.icon"
								:model-value="config[field.key]"
								:sensitive="field.sensitive"
								@update:model-value="updateField(field.key, $event)"
							/>
						</div>
					</div>

					<preset-list
						:presets="presets"
						:active-preset-id="activePresetId"
						@apply="handleApplyPreset"
						@delete="handleDeletePreset"
						@new-preset="showDialog = true"
					/>
				</template>

				<preset-dialog v-model:show="showDialog" @save="handleSavePreset" />
			</div>
		</n-message-provider>
	</n-config-provider>
</template>
