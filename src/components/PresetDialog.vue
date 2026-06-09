<script setup lang="ts">
import { ref, watch } from 'vue'
import { NModal, NInput, NSpace, NButton } from 'naive-ui'

const props = defineProps<{
	show: boolean
}>()

const emit = defineEmits<{
	'update:show': [value: boolean]
	save: [name: string]
}>()

const presetName = ref('')

watch(
	() => props.show,
	val => {
		if (val) presetName.value = ''
	}
)

function handleSave() {
	if (!presetName.value.trim()) return
	emit('save', presetName.value.trim())
	emit('update:show', false)
}
</script>

<template>
	<n-modal :show="show" preset="dialog" title="新建预设" @update:show="emit('update:show', $event)">
		<n-space vertical :size="12">
			<n-input v-model:value="presetName" placeholder="输入预设名称" @keyup.enter="handleSave" />
			<n-space justify="end">
				<n-button @click="emit('update:show', false)">取消</n-button>
				<n-button type="primary" :disabled="!presetName.trim()" @click="handleSave">保存</n-button>
			</n-space>
		</n-space>
	</n-modal>
</template>
