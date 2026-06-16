<script setup lang="ts">
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
</script>

<template>
	<div class="flex flex-col rounded-12px overflow-hidden" style="box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06)">
		<button
			class="flex items-center justify-between w-full px-20px py-14px bg-[#F1F5F9] cursor-pointer hover:bg-[#E2E8F0] transition-colors"
			@click="$emit('toggle')"
		>
			<div class="flex items-center gap-10px">
				<svg
					class="w-14px h-14px text-[#6366F1] shrink-0"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<path
						d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"
					/>
				</svg>
				<span class="text-14px font-600 text-[#1E293B]">{{ extractProjectDir(projectPath) }}</span>
				<span class="text-12px text-[#64748B]">{{ sessionCount }} 个会话</span>
				<span class="text-12px text-[#000000]">{{ projectPath }}</span>
			</div>
			<svg
				class="w-14px h-14px text-[#64748B] transition-transform duration-200"
				:class="{ 'rotate-90': expanded }"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path d="m9 18 6-6-6-6" />
			</svg>
		</button>
		<div v-if="expanded" class="flex flex-col gap-8px p-12px_16px">
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
