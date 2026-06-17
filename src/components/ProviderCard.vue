<script setup lang="ts">
import type { Provider } from '../types'

defineProps<{
	provider: Provider
}>()

defineEmits<{
	activate: [provider: Provider]
	edit: [provider: Provider]
	duplicate: [id: string]
	delete: [id: string]
}>()
</script>

<template>
	<div
		class="flex items-center gap-16px p-16px pr-20px bg-white rounded-12px border cursor-pointer transition-all shadow-[0_2px_8px_rgba(0,0,0,0.03)] hover:border-gray-300 hover:shadow-[0_4px_12px_rgba(0,0,0,0.06)]"
		:class="provider.is_active ? '!border-[#3B82F6] !border-l-[3px]' : 'border-gray-200'"
	>
		<!-- 拖拽手柄 -->
		<div class="shrink-0 cursor-grab">
			<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#D1D5DB" stroke-width="2">
				<circle cx="9" cy="6" r="1" />
				<circle cx="15" cy="6" r="1" />
				<circle cx="9" cy="12" r="1" />
				<circle cx="15" cy="12" r="1" />
				<circle cx="9" cy="18" r="1" />
				<circle cx="15" cy="18" r="1" />
			</svg>
		</div>

		<!-- Logo -->
		<div
			class="flex items-center justify-center w-40px h-40px rounded-full shrink-0"
			:class="provider.is_active ? 'bg-blue-50' : 'bg-gray-100'"
		>
			<span class="text-14px font-600" :class="provider.is_active ? 'text-blue-500' : 'text-gray-500'">
				{{ provider.name.charAt(0).toUpperCase() }}
			</span>
		</div>

		<!-- 信息 -->
		<div class="flex-1 flex flex-col gap-2px min-w-0">
			<span class="text-14px font-700 text-gray-900">{{ provider.name }}</span>
			<a
				v-if="provider.website"
				:href="provider.website"
				target="_blank"
				class="text-12px text-blue-500 no-underline truncate hover:underline"
				@click.stop
			>
				{{ provider.website }}
			</a>
		</div>

		<!-- 操作按钮 -->
		<div class="shrink-0 flex items-center gap-8px">
			<!-- 激活状态 -->
			<span
				v-if="provider.is_active"
				class="flex items-center gap-4px text-12px text-green-500 bg-green-50 px-8px py-4px rounded-4px"
			>
				<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<polyline points="20 6 9 17 4 12" />
				</svg>
				使用中
			</span>
			<!-- 激活按钮 -->
			<button
				v-else
				class="flex items-center gap-4px text-12px text-gray-600 bg-gray-100 px-8px py-4px rounded-4px border-none cursor-pointer hover:bg-gray-200 transition-colors"
				@click.stop="$emit('activate', provider)"
			>
				<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
					<polyline points="22 4 12 14.01 9 11.01" />
				</svg>
				激活
			</button>

			<!-- 复制 -->
			<button
				class="flex items-center justify-center w-28px h-28px rounded-6px border-none bg-transparent cursor-pointer hover:bg-gray-100 transition-colors"
				@click.stop="$emit('duplicate', provider.id)"
			>
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#6B7280" stroke-width="2">
					<rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
					<path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
				</svg>
			</button>

			<!-- 编辑 -->
			<button
				class="flex items-center justify-center w-28px h-28px rounded-6px border-none bg-transparent cursor-pointer hover:bg-gray-100 transition-colors"
				@click.stop="$emit('edit', provider)"
			>
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#6B7280" stroke-width="2">
					<path d="M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z" />
				</svg>
			</button>

			<!-- 删除 -->
			<button
				class="flex items-center justify-center w-28px h-28px rounded-6px border-none bg-transparent cursor-pointer hover:bg-red-50 transition-colors"
				@click.stop="$emit('delete', provider.id)"
			>
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#6B7280" stroke-width="2">
					<polyline points="3 6 5 6 21 6" />
					<path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
				</svg>
			</button>
		</div>
	</div>
</template>
