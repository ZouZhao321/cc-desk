<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { NEmpty, NSkeleton } from 'naive-ui'
import { useSessionHistory } from '../composables/useSessionHistory'
import ProjectCard from './ProjectCard.vue'
import SessionDetail from './SessionDetail.vue'

const emit = defineEmits<{
	'detail-change': [isDetail: boolean]
	back: []
}>()

const {
	sessions,
	currentMessages,
	notes,
	loading,
	error,
	selectedSessionId,
	selectedSession,
	currentNote,
	loadSessions,
	loadSession,
	saveNote,
	clearSession
} = useSessionHistory()

watch(selectedSessionId, newVal => {
	emit('detail-change', newVal !== null)
})

const groupedSessions = computed(() => {
	const groups: Record<string, typeof sessions.value> = {}
	for (const session of sessions.value) {
		const path = session.project_path || '未知项目'
		if (!groups[path]) groups[path] = []
		groups[path].push(session)
	}
	return Object.entries(groups)
})

const expandedProjects = ref<string[]>([])

watch(
	groupedSessions,
	newGroups => {
		if (expandedProjects.value.length === 0 && newGroups.length > 0) {
			expandedProjects.value = newGroups.map(([path]) => path)
		}
	},
	{ immediate: true }
)

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

function toggleProject(projectPath: string) {
	const idx = expandedProjects.value.indexOf(projectPath)
	if (idx >= 0) {
		expandedProjects.value.splice(idx, 1)
	} else {
		expandedProjects.value.push(projectPath)
	}
}

// 初始加载
loadSessions()
</script>

<template>
	<div class="flex flex-col w-full h-full bg-white font-sans">
		<header v-if="!selectedSessionId" class="flex items-center h-56px px-24px bg-white shrink-0">
			<div class="flex items-center gap-20px">
				<button
					class="flex items-center justify-center w-24px h-24px text-[#666666] hover:text-gray-800 transition-colors"
					@click="emit('back')"
				>
					<svg
						class="w-24px h-24px"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<path d="m15 18-6-6 6-6" />
					</svg>
				</button>
				<span class="text-20px font-bold text-[#111827]">会话管理</span>
				<div class="flex-1"></div>
				<button
					class="flex items-center justify-center w-24px h-24px text-[#000000] hover:text-gray-600 transition-colors"
					@click="handleRefresh"
				>
					<svg
						class="w-24px h-24px"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
						<path d="M3 3v5h5" />
						<path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16" />
						<path d="M16 16h5v5" />
					</svg>
				</button>
			</div>
		</header>

		<!-- 列表视图 -->
		<template v-if="!selectedSessionId">
			<main class="flex-1 py-16px px-24px bg-[#F8F9FA] overflow-y-auto">
				<div v-if="loading && sessions.length === 0" class="flex flex-col gap-16px py-16px">
					<div v-for="g in 3" :key="g" class="flex flex-col gap-8px">
						<NSkeleton :width="200 + g * 40" :height="20" :border-radius="6" />
						<div
							v-for="c in 2 + g"
							:key="c"
							class="flex items-center gap-12px p-12px_18px bg-white rounded-10px border border-gray-200"
						>
							<NSkeleton :width="140" :height="14" :border-radius="4" />
							<NSkeleton class="flex-1" :height="14" :border-radius="4" />
							<NSkeleton :width="80" :height="14" :border-radius="4" />
						</div>
					</div>
				</div>
				<div v-else-if="error" class="flex flex-col items-center gap-12px py-48px">
					<span class="text-red-500">{{ error }}</span>
					<button
						class="px-12px py-6px text-13px text-white bg-indigo-500 rounded-6px hover:bg-indigo-600 transition-colors"
						@click="handleRefresh"
					>
						重试
					</button>
				</div>
				<NEmpty v-else-if="sessions.length === 0" description="暂无会话记录" class="py-48px" />
				<div v-else class="flex flex-col gap-12px">
					<ProjectCard
						v-for="[projectPath, projectSessions] in groupedSessions"
						:key="projectPath"
						:project-name="projectPath"
						:project-path="projectPath"
						:session-count="projectSessions.length"
						:sessions="projectSessions"
						:notes="notes"
						:expanded="expandedProjects.includes(projectPath)"
						@toggle="toggleProject(projectPath)"
						@session-click="handleCardClick"
					/>
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
				:note="noteInputValue"
				:session-id="selectedSessionId || ''"
				@back="handleBack"
				@update:note="noteInputValue = $event"
				@save-note="handleNoteSave"
			/>
		</template>
	</div>
</template>
