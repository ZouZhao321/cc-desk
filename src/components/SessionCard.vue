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
		class="flex items-center gap-12px p-12px_18px bg-white rounded-10px border border-gray-200 cursor-pointer hover:border-blue-400 transition-colors"
		@click="$emit('click', session.session_id)"
	>
		<span class="text-13px font-500 text-gray-900 shrink-0">{{ formatDate(session.started_at) }}</span>
		<span v-if="note" class="flex-1 text-12px text-gray-500 truncate">{{ truncateNote(note) }}</span>
		<span v-else class="flex-1 text-12px text-gray-400">未添加备注</span>
		<span class="text-12px text-gray-400 shrink-0">{{ session.message_count }} messages</span>
	</div>
</template>
