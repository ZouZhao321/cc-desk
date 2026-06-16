<script setup lang="ts">
import { Folder, ChevronRight, ChevronDown } from '@lucide/vue'
import type { SessionMeta } from '../types'
import SessionItem from './SessionItem.vue'

defineProps<{
	projectName: string
	projectPath: string
	sessionCount: number
	sessions: SessionMeta[]
	notes: Record<string, string>
	expanded: boolean
}>()

defineEmits<{
	toggle: []
	'session-click': [sessionId: string]
}>()

function extractProjectDir(fullPath: string): string {
	const parts = fullPath.replace(/\\/g, '/').split('/')
	return parts[parts.length - 1] || fullPath
}

function truncatePathMiddle(path: string, maxLen = 40): string {
	if (path.length <= maxLen) return path
	const start = path.slice(0, Math.ceil(maxLen / 2))
	const end = path.slice(-Math.floor(maxLen / 2))
	return `${start}...${end}`
}
</script>

<template>
	<div class="flex flex-col rounded-12px overflow-hidden bg-white" style="box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06)">
		<button
			class="flex items-center justify-between w-full px-20px py-14px bg-[#F1F5F9] cursor-pointer hover:bg-[#E2E8F0] transition-colors border-0 outline-none border-b-transparent rounded-t-12px"
			@click="$emit('toggle')"
		>
			<div class="flex items-center gap-10px min-w-0 flex-1">
				<Folder class="w-14px h-14px text-[#6366F1] shrink-0" :size="14" />
				<span class="text-14px font-600 text-[#1E293B] shrink-0">{{ extractProjectDir(projectPath) }}</span>
				<span class="text-12px text-[#64748B] shrink-0">{{ sessionCount }} 个会话</span>
				<span class="text-12px text-[#000000] truncate" :title="projectPath">{{
					truncatePathMiddle(projectPath)
				}}</span>
			</div>
			<ChevronDown v-if="expanded" class="w-14px h-14px text-[#64748B] shrink-0" :size="14" />
			<ChevronRight v-else class="w-14px h-14px text-[#64748B] shrink-0" :size="14" />
		</button>
		<div v-if="expanded" class="flex flex-col gap-12px p-12px">
			<SessionItem
				v-for="session in sessions"
				:key="session.session_id"
				:session="session"
				:note="notes[session.session_id] || ''"
				@click="$emit('session-click', $event)"
			/>
		</div>
	</div>
</template>
