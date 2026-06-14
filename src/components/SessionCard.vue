<script setup lang="ts">
import type { SessionMeta } from '../types'

defineProps<{
	session: SessionMeta
	note: string
}>()

defineEmits<{
	click: [sessionId: string]
}>()

function formatDate(dateStr: string | null): string {
	if (!dateStr) return '未知时间'
	const date = new Date(dateStr)
	const y = date.getFullYear()
	const m = String(date.getMonth() + 1).padStart(2, '0')
	const d = String(date.getDate()).padStart(2, '0')
	const h = String(date.getHours()).padStart(2, '0')
	const min = String(date.getMinutes()).padStart(2, '0')
	return `${y}-${m}-${d} ${h}:${min}`
}

function truncateNote(note: string, maxLen: number = 50): string {
	if (!note) return ''
	return note.length > maxLen ? note.slice(0, maxLen) + '...' : note
}
</script>

<template>
	<div
		class="flex flex-col gap-8px p-14px_18px bg-white rounded-10px border border-gray-200 cursor-pointer hover:border-blue-400 transition-colors"
		@click="$emit('click', session.session_id)"
	>
		<div class="flex items-center gap-8px text-13px">
			<span class="font-500 text-gray-900">{{ formatDate(session.started_at) }}</span>
			<span class="text-gray-400">·</span>
			<span class="font-mono text-11px" :class="session.project_path ? 'text-indigo-500' : 'text-gray-400'">
				{{ session.project_path || '未知路径' }}
			</span>
			<span class="text-gray-400">·</span>
			<span class="text-gray-500">{{ session.message_count }} messages</span>
		</div>
		<div v-if="note" class="flex items-center gap-6px px-10px py-6px bg-indigo-50 rounded-6px">
			<svg
				class="w-12px h-12px text-indigo-500"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
			</svg>
			<span class="text-12px text-indigo-600">{{ truncateNote(note) }}</span>
		</div>
		<div v-else class="flex items-center gap-6px px-10px py-6px bg-gray-100 rounded-6px">
			<svg
				class="w-12px h-12px text-gray-400"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
			</svg>
			<span class="text-12px text-gray-400">未添加备注</span>
		</div>
	</div>
</template>
