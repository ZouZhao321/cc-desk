<script setup lang="ts">
import { computed, ref } from 'vue'
import { useSessionHistory } from '../composables/useSessionHistory'
import SessionCard from './SessionCard.vue'
import SessionDetail from './SessionDetail.vue'
import SessionNoteInput from './SessionNoteInput.vue'

const {
	sessions,
	currentMessages,
	loading,
	error,
	selectedSessionId,
	selectedSession,
	currentNote,
	loadSessions,
	loadSession,
	saveNote,
	clearSession,
	getNote
} = useSessionHistory()

const groupedSessions = computed(() => {
	const groups: Record<string, typeof sessions.value> = {}
	for (const session of sessions.value) {
		const path = session.project_path || '未知项目'
		if (!groups[path]) groups[path] = []
		groups[path].push(session)
	}
	return Object.entries(groups)
})

const noteInputValue = computed({
	get: () => currentNote.value,
	set: (val: string) => {
		if (selectedSessionId.value) {
			saveNote(selectedSessionId.value, val)
		}
	}
})

function handleCardClick(sessionId: string) {
	loadSession(sessionId)
}

function handleBack() {
	clearSession()
}

function handleRefresh() {
	loadSessions()
}

function handleNoteSave(sessionId: string, note: string) {
	saveNote(sessionId, note)
}

// 初始加载
loadSessions()

const collapsedGroups = ref(new Set<string>())

function toggleGroup(path: string) {
	const newSet = new Set(collapsedGroups.value)
	if (newSet.has(path)) {
		newSet.delete(path)
	} else {
		newSet.add(path)
	}
	collapsedGroups.value = newSet
}

function isCollapsed(path: string): boolean {
	return collapsedGroups.value.has(path)
}
</script>

<template>
	<div class="flex flex-col w-full h-full bg-white font-sans">
		<header class="flex items-center justify-between h-56px px-24px bg-white border-b border-gray-100 shrink-0">
			<div class="flex items-center gap-16px">
				<span class="text-20px font-bold text-[#FF6B35]">CC-Desk</span>
				<div class="flex-1"></div>
				<button
					class="flex items-center gap-6px px-12px py-6px bg-gray-100 rounded-6px text-12px text-gray-600 hover:bg-gray-200 transition-colors"
					@click="handleRefresh"
				>
					<svg class="w-14px h-14px" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M23 4v6h-6M1 20v-6h6" />
						<path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15" />
					</svg>
					Refresh
				</button>
			</div>
		</header>

		<!-- 列表视图 -->
		<template v-if="!selectedSessionId">
			<main class="flex-1 py-16px px-24px bg-[#F8F9FA] overflow-y-auto">
				<div
					v-if="loading && sessions.length === 0"
					class="flex items-center justify-center py-48px text-gray-400"
				>
					加载中...
				</div>
				<div v-else-if="error" class="flex flex-col items-center gap-12px py-48px">
					<span class="text-red-500">{{ error }}</span>
					<button class="text-13px text-indigo-600 hover:underline" @click="handleRefresh">重试</button>
				</div>
				<div v-else-if="sessions.length === 0" class="flex items-center justify-center py-48px text-gray-400">
					暂无会话记录
				</div>
				<div v-else class="flex flex-col gap-24px">
					<div
						v-for="[projectPath, projectSessions] in groupedSessions"
						:key="projectPath"
						class="flex flex-col gap-12px"
					>
						<div
							class="flex items-center gap-8px px-0 py-8px cursor-pointer select-none"
							@click="toggleGroup(projectPath)"
						>
							<svg
								class="w-12px h-12px text-gray-400 transition-transform"
								:class="{ 'rotate-90': !isCollapsed(projectPath) }"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
							>
								<path d="M9 18l6-6-6-6" />
							</svg>
							<svg
								class="w-14px h-14px text-indigo-500"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
							>
								<path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" />
							</svg>
							<span class="text-13px font-600 text-gray-700">{{ projectPath }}</span>
							<span class="text-12px text-gray-400">{{ projectSessions.length }} sessions</span>
						</div>
						<div v-show="!isCollapsed(projectPath)" class="flex flex-col gap-12px">
							<SessionCard
								v-for="session in projectSessions"
								:key="session.session_id"
								:session="session"
								:note="getNote(session.session_id)"
								@click="handleCardClick"
							/>
						</div>
					</div>
				</div>
			</main>
		</template>

		<!-- 详情视图 -->
		<template v-else>
			<SessionDetail
				:messages="currentMessages"
				:session-info="
					selectedSession
						? {
								project_path: selectedSession.project_path,
								started_at: selectedSession.started_at,
								version: selectedSession.version,
								message_count: selectedSession.message_count
							}
						: null
				"
				@back="handleBack"
			/>
			<SessionNoteInput
				v-if="selectedSessionId"
				:model-value="noteInputValue"
				:session-id="selectedSessionId"
				@update:model-value="noteInputValue = $event"
				@save="handleNoteSave"
			/>
		</template>
	</div>
</template>
