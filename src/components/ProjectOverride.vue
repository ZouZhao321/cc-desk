<script setup lang="ts">
import type { ProjectOverride as OverrideType } from '../types'

defineProps<{
	override: OverrideType
}>()

defineEmits<{
	update: [id: string, value: string]
	remove: [id: string]
}>()
</script>

<template>
	<div class="flex items-center gap-14px bg-white rounded-10px border border-blue-500 px-16px py-14px">
		<div class="flex-1 flex flex-col gap-3px min-w-0">
			<span class="text-13px font-500 text-gray-900">{{ override.label }}</span>
			<span class="text-11px text-gray-500"
				>Global: {{ override.globalValue }} → Project: {{ override.projectValue }}</span
			>
		</div>

		<div class="flex items-center bg-blue-50 border border-blue-500 rounded-6px h-32px px-10px w-80px">
			<input
				class="w-full bg-transparent border-none outline-none text-13px text-blue-500 font-mono text-center"
				:value="override.projectValue"
				@input="$emit('update', override.id, ($event.target as HTMLInputElement).value)"
			/>
		</div>

		<button
			class="flex items-center justify-center w-28px h-28px bg-red-50 border border-red-100 rounded-6px cursor-pointer hover:bg-red-100 transition-colors"
			@click="$emit('remove', override.id)"
		>
			<svg
				width="14"
				height="14"
				viewBox="0 0 24 24"
				fill="none"
				stroke="#EF4444"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path d="M18 6 6 18" />
				<path d="m6 6 12 12" />
			</svg>
		</button>
	</div>
</template>
