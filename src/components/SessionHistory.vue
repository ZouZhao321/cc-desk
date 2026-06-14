<script setup lang="ts">
import { computed, ref } from 'vue'
import { NCard, NEmpty, NSpin, NButton } from 'naive-ui'
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
				<NButton size="small" @click="handleRefresh"> 刷新 </NButton>
			</div>
		</header>

		<!-- 列表视图 -->
		<template v-if="!selectedSessionId">
			<main class="flex-1 py-16px px-24px bg-[#F8F9FA] overflow-y-auto">
				<NSpin v-if="loading && sessions.length === 0" class="py-48px" />
				<div v-else-if="error" class="flex flex-col items-center gap-12px py-48px">
					<span class="text-red-500">{{ error }}</span>
					<NButton size="small" @click="handleRefresh">重试</NButton>
				</div>
				<NEmpty v-else-if="sessions.length === 0" description="暂无会话记录" class="py-48px" />
				<div v-else class="flex flex-col gap-16px">
					<NCard
						v-for="[projectPath, projectSessions] in groupedSessions"
						:key="projectPath"
						size="small"
						:segmented="{ content: true }"
					>
						<template #header>
							<div
								class="flex items-center gap-8px cursor-pointer select-none"
								@click="toggleGroup(projectPath)"
							>
								<span
									class="text-14px transition-transform"
									:class="{ 'rotate-90': !isCollapsed(projectPath) }"
								>
									▶
								</span>
								<span class="text-14px font-600">{{ projectPath }}</span>
								<span class="text-12px text-gray-400 ml-8px"
									>{{ projectSessions.length }} sessions</span
								>
							</div>
						</template>
						<div v-show="!isCollapsed(projectPath)" class="flex flex-col gap-8px">
							<SessionCard
								v-for="session in projectSessions"
								:key="session.session_id"
								:session="session"
								:note="getNote(session.session_id)"
								@click="handleCardClick"
							/>
						</div>
					</NCard>
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
