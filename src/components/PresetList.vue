<script setup lang="ts">
import { NCard, NButton, NSpace, NList, NListItem, NTag, NEmpty } from 'naive-ui'
import type { Preset } from '../types'

defineProps<{
	presets: Preset[]
	activePresetId: string | null
}>()

const emit = defineEmits<{
	apply: [id: string]
	delete: [id: string]
	newPreset: []
}>()
</script>

<template>
	<n-card title="预设管理" size="small">
		<template #header-extra>
			<n-button size="small" type="primary" @click="emit('newPreset')">+ 新建预设</n-button>
		</template>

		<n-empty v-if="presets.length === 0" description="暂无预设" class="py-4" />

		<n-list v-else bordered :show-divider="false">
			<n-list-item v-for="preset in presets" :key="preset.id">
				<n-space align="center" justify="space-between" class="w-full">
					<n-space align="center" :size="8">
						<span>{{ preset.name }}</span>
						<n-tag v-if="preset.id === activePresetId" type="success" size="small">当前</n-tag>
					</n-space>
					<n-space :size="4">
						<n-button size="tiny" @click="emit('apply', preset.id)">应用</n-button>
						<n-button size="tiny" type="error" @click="emit('delete', preset.id)">删除</n-button>
					</n-space>
				</n-space>
			</n-list-item>
		</n-list>
	</n-card>
</template>
