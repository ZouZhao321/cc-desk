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
</script>

<template>
	<div
		class="flex flex-col gap-8px p-12px_16px bg-white rounded-8px border border-[#E2E8F0] cursor-pointer hover:border-[#6366F1] transition-colors"
		style="box-shadow: 0 1px 3px rgba(0, 0, 0, 0.03)"
		@click="$emit('click', session.session_id)"
	>
		<div class="flex items-center justify-between">
			<div class="flex items-center gap-6px">
				<span class="text-13px font-500 text-[#1E293B]">{{ formatDate(session.started_at) }}</span>
				<span class="text-12px text-[#94A3B8]">·</span>
				<span class="text-12px text-[#64748B]">{{ session.message_count }} messages</span>
			</div>
		</div>
		<div v-if="note" class="flex items-center gap-6px px-10px py-6px rounded-6px bg-[#EEF2FF]">
			<svg
				class="w-12px h-12px text-[#6366F1] shrink-0"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
			</svg>
			<span class="text-12px text-[#6366F1] truncate">{{ note }}</span>
		</div>
		<div v-else class="flex items-center gap-6px px-10px py-6px rounded-6px bg-[#F3F4F6]">
			<svg
				class="w-12px h-12px text-[#94A3B8] shrink-0"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
			</svg>
			<span class="text-12px text-[#94A3B8]">未添加备注</span>
		</div>
	</div>
</template>
