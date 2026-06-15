<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { NEmpty, NSkeleton, NButton, NCollapse, NCollapseItem, NTag } from 'naive-ui'
import { useSessionHistory } from '../composables/useSessionHistory'
import SessionCard from './SessionCard.vue'
import SessionDetail from './SessionDetail.vue'

const emit = defineEmits<{
	'detail-change': [isDetail: boolean]
}>()

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

const collapseValue = ref<string[]>([])

watch(
	groupedSessions,
	newGroups => {
		if (collapseValue.value.length === 0 && newGroups.length > 0) {
			collapseValue.value = newGroups.map(([path]) => path)
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

// 初始加载
loadSessions()
</script>

<template>
	<div class="flex flex-col w-full h-full bg-white font-sans">
		<header
			v-if="!selectedSessionId"
			class="flex items-center justify-between h-56px px-24px bg-white border-b border-gray-100 shrink-0"
		>
			<div class="flex items-center gap-16px">
				<span class="text-20px font-bold text-[#FF6B35]">CC-Desk</span>
				<div class="flex-1"></div>
				<NButton size="small" @click="handleRefresh">刷新</NButton>
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
					<NButton size="small" @click="handleRefresh">重试</NButton>
				</div>
				<NEmpty v-else-if="sessions.length === 0" description="暂无会话记录" class="py-48px" />
				<NCollapse v-else v-model:value="collapseValue">
					<NCollapseItem
						v-for="[projectPath, projectSessions] in groupedSessions"
						:key="projectPath"
						:name="projectPath"
					>
						<template #header>
							<div class="flex items-center gap-8px">
								<span class="text-14px font-600">{{ projectPath }}</span>
								<NTag size="small" :bordered="false" type="info">
									{{ projectSessions.length }}
								</NTag>
							</div>
						</template>
						<div class="flex flex-col gap-8px">
							<SessionCard
								v-for="session in projectSessions"
								:key="session.session_id"
								:session="session"
								:note="getNote(session.session_id)"
								@click="handleCardClick"
							/>
						</div>
					</NCollapseItem>
				</NCollapse>
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
