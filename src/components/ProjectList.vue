<script setup lang="ts">
import type { ProjectListItem } from '../types'

defineProps<{
	projects: ProjectListItem[]
}>()
</script>

<template>
	<div class="flex flex-col gap-12px">
		<div
			v-for="project in projects"
			:key="project.id"
			class="flex items-center gap-16px bg-white rounded-10px border border-gray-200 px-20px py-16px"
		>
			<div class="flex items-center justify-center w-44px h-44px bg-blue-50 rounded-10px shrink-0">
				<svg
					width="22"
					height="22"
					viewBox="0 0 24 24"
					fill="none"
					stroke="#3B82F6"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<path
						d="m6 14 1.5-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.54 6a2 2 0 0 1-1.95 1.5H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H18a2 2 0 0 1 2 2v2"
					/>
				</svg>
			</div>

			<div class="flex-1 flex flex-col gap-4px min-w-0">
				<span class="text-14px font-500 text-gray-900">{{ project.name }}</span>
				<span class="text-12px text-gray-500 font-mono">{{ project.path }}</span>
				<span class="text-10px text-gray-500 bg-gray-100 px-8px py-2px rounded-4px w-fit">{{
					project.language
				}}</span>
			</div>

			<span class="text-12px text-gray-400 shrink-0">{{ project.syncTime }}</span>

			<button
				class="flex items-center justify-center w-32px h-32px bg-gray-100 rounded-6px border-none cursor-pointer hover:bg-gray-200 transition-colors shrink-0"
			>
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
					<circle cx="12" cy="12" r="1" />
					<circle cx="19" cy="12" r="1" />
					<circle cx="5" cy="12" r="1" />
				</svg>
			</button>
		</div>
	</div>
</template>
