<script setup lang="ts">
import { ref, watch } from 'vue'

const props = defineProps<{
	modelValue: string
	sessionId: string
}>()

const emit = defineEmits<{
	'update:modelValue': [value: string]
	save: [sessionId: string, note: string]
}>()

const localValue = ref(props.modelValue)
const saveTimer = ref<ReturnType<typeof setTimeout> | null>(null)

watch(
	() => props.modelValue,
	newVal => {
		localValue.value = newVal
	}
)

function handleInput() {
	emit('update:modelValue', localValue.value)

	if (saveTimer.value) {
		clearTimeout(saveTimer.value)
	}

	saveTimer.value = setTimeout(() => {
		emit('save', props.sessionId, localValue.value)
	}, 500)
}

function handleBlur() {
	if (saveTimer.value) {
		clearTimeout(saveTimer.value)
	}
	emit('save', props.sessionId, localValue.value)
}
</script>

<template>
	<div class="flex flex-col gap-8px py-12px px-48px bg-white">
		<span class="text-12px font-500 text-gray-500">Note</span>
		<div class="flex items-center px-12px py-10px bg-gray-50 border border-gray-200 rounded-6px">
			<input
				v-model="localValue"
				type="text"
				class="flex-1 bg-transparent text-13px text-gray-900 outline-none placeholder:text-gray-400"
				placeholder="添加备注..."
				@input="handleInput"
				@blur="handleBlur"
			/>
		</div>
	</div>
</template>
