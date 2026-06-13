<script setup lang="ts">
import type { InheritedItem } from '../types'

defineProps<{
	title: string
	count: string
	items: InheritedItem[]
	showLock?: boolean
}>()
</script>

<template>
	<div class="flex flex-col bg-white rounded-10px border border-gray-200">
		<div class="flex items-center gap-12px px-16px py-14px">
			<svg
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="#6B7280"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path d="m6 9 6 6 6-6" />
			</svg>
			<svg
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="#3B82F6"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<circle cx="12" cy="12" r="10" />
				<path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20" />
				<path d="M2 12h20" />
			</svg>
			<span class="text-13px font-500 text-gray-700">{{ title }}</span>
			<span class="text-11px text-blue-500 bg-blue-50 px-8px py-2px rounded-10px">{{ count }}</span>
			<svg
				v-if="showLock"
				width="14"
				height="14"
				viewBox="0 0 24 24"
				fill="none"
				stroke="#9CA3AF"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<rect width="18" height="11" x="3" y="11" rx="2" ry="2" />
				<path d="M7 11V7a5 5 0 0 1 10 0v4" />
			</svg>
		</div>

		<div class="flex flex-col gap-8px px-16px pb-12px">
			<div
				v-for="item in items"
				:key="item.id"
				class="flex items-center gap-12px bg-gray-50 rounded-8px px-12px py-10px"
			>
				<div class="flex items-center justify-center w-16px h-16px" :style="{ color: item.iconColor }">
					<span class="text-16px">{{ item.icon }}</span>
				</div>

				<div class="flex-1 flex flex-col gap-2px min-w-0">
					<span class="text-13px font-500 text-gray-900">{{ item.name }}</span>
					<span class="text-11px text-gray-500">{{ item.description }}</span>
				</div>

				<span
					class="text-10px font-500 px-8px py-3px rounded-10px"
					:class="item.active ? 'text-emerald-600 bg-emerald-50' : 'text-gray-500 bg-gray-100'"
				>
					{{ item.active ? 'Active' : 'Inactive' }}
				</span>

				<svg
					width="12"
					height="12"
					viewBox="0 0 24 24"
					fill="none"
					stroke="#D1D5DB"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<rect width="18" height="11" x="3" y="11" rx="2" ry="2" />
					<path d="M7 11V7a5 5 0 0 1 10 0v4" />
				</svg>
			</div>
		</div>
	</div>
</template>
