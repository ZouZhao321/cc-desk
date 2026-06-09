<script setup lang="ts">
import { ref } from 'vue'
import { NCard, NInput, NButton, NSpace } from 'naive-ui'
import { maskToken } from '../utils/mask'

const props = defineProps<{
	label: string
	icon: string
	modelValue: string
	sensitive: boolean
}>()

const emit = defineEmits<{
	'update:modelValue': [value: string]
}>()

const editing = ref(false)
const editValue = ref('')

function startEdit() {
	editValue.value = props.modelValue
	editing.value = true
}

function confirmEdit() {
	emit('update:modelValue', editValue.value)
	editing.value = false
}

function cancelEdit() {
	editing.value = false
}

function displayValue(): string {
	if (!props.modelValue) return '(未设置)'
	if (props.sensitive) return maskToken(props.modelValue)
	return props.modelValue
}
</script>

<template>
	<n-card size="small" hoverable class="w-72 cursor-pointer" @click="startEdit">
		<div class="flex items-center gap-2 mb-2">
			<span class="text-lg">{{ icon }}</span>
			<span class="font-bold text-sm">{{ label }}</span>
		</div>

		<div v-if="!editing" class="text-gray-600 dark:text-gray-300 text-sm truncate">
			{{ displayValue() }}
		</div>

		<div v-else @click.stop>
			<n-space vertical :size="8">
				<n-input
					v-model:value="editValue"
					size="small"
					:type="sensitive ? 'password' : 'text'"
					:placeholder="label"
					@keyup.enter="confirmEdit"
					@keyup.escape="cancelEdit"
				/>
				<n-space :size="4" justify="end">
					<n-button size="tiny" @click="cancelEdit">取消</n-button>
					<n-button size="tiny" type="primary" @click="confirmEdit">确认</n-button>
				</n-space>
			</n-space>
		</div>
	</n-card>
</template>
