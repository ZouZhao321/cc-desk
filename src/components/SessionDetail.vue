<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import type { Message, ContentBlock } from '../types'

const props = defineProps<{
	messages: Message[]
	sessionInfo: {
		project_path: string
		started_at: string | null
		version: string | null
		message_count: number
	} | null
	note: string
	sessionId: string
}>()

const emit = defineEmits<{
	back: []
	'update:note': [value: string]
	'save-note': [sessionId: string, note: string]
}>()

const expandedThinking = ref<Set<string>>(new Set())

function toggleThinking(uuid: string) {
	if (expandedThinking.value.has(uuid)) {
		expandedThinking.value.delete(uuid)
	} else {
		expandedThinking.value.add(uuid)
	}
}

function formatTime(timestamp: string): string {
	const date = new Date(timestamp)
	const h = String(date.getHours()).padStart(2, '0')
	const m = String(date.getMinutes()).padStart(2, '0')
	const s = String(date.getSeconds()).padStart(2, '0')
	return `${h}:${m}:${s}`
}

function getRoleLabel(role: string): string {
	switch (role) {
		case 'user':
			return 'USER'
		case 'assistant':
			return 'ASSISTANT'
		case 'system':
			return 'SYSTEM'
		default:
			return role.toUpperCase()
	}
}

function getRoleColor(role: string): string {
	switch (role) {
		case 'user':
			return 'text-gray-500'
		case 'assistant':
			return 'text-indigo-600'
		case 'system':
			return 'text-gray-400'
		default:
			return 'text-gray-500'
	}
}

function getTextBlocks(content: ContentBlock[]): string {
	return content
		.filter((b): b is { type: 'text'; text: string } => b.type === 'text')
		.map(b => b.text)
		.join('\n')
}

function getToolUseBlocks(content: ContentBlock[]): { id: string; name: string; input: unknown }[] {
	return content.filter(
		(b): b is { type: 'tool_use'; id: string; name: string; input: unknown } => b.type === 'tool_use'
	)
}

function getToolResultBlocks(content: ContentBlock[]): { tool_use_id: string; content: string }[] {
	return content.filter(
		(b): b is { type: 'tool_result'; tool_use_id: string; content: string } => b.type === 'tool_result'
	)
}

function getThinkingBlocks(content: ContentBlock[]): string[] {
	return content
		.filter((b): b is { type: 'thinking'; thinking: string } => b.type === 'thinking')
		.map(b => b.thinking)
}

function getAttachmentBlocks(content: ContentBlock[]): { attachment_type: string; content: string }[] {
	return content.filter(
		(b): b is { type: 'attachment'; attachment_type: string; content: string } => b.type === 'attachment'
	)
}

const projectName = computed(() => {
	const p = props.sessionInfo?.project_path || ''
	const parts = p.replace(/\\/g, '/').split('/')
	return parts[parts.length - 1] || p
})

const metaInfo = computed(() => {
	if (!props.sessionInfo) return ''
	const parts: string[] = []
	if (props.sessionInfo.started_at) {
		const date = new Date(props.sessionInfo.started_at)
		parts.push(date.toLocaleString())
	}
	if (props.sessionInfo.version) parts.push(`v${props.sessionInfo.version}`)
	parts.push(`${props.sessionInfo.message_count} messages`)
	return parts.join(' · ')
})

const toolNameMap = new Map<string, string>()

function getToolNameFromResult(msg: Message): string {
	for (const block of msg.content) {
		if (block.type === 'tool_result') {
			const toolUseId = block.tool_use_id
			if (toolNameMap.has(toolUseId)) {
				return toolNameMap.get(toolUseId)!
			}
		}
	}
	return ''
}

// Build tool name map from all messages
for (const msg of props.messages) {
	for (const block of msg.content) {
		if (block.type === 'tool_use') {
			toolNameMap.set(block.id, block.name)
		}
	}
}

const noteValue = ref(props.note)
const saveTimer = ref<ReturnType<typeof setTimeout> | null>(null)

watch(
	() => props.note,
	newVal => {
		noteValue.value = newVal
	}
)

function handleNoteInput() {
	emit('update:note', noteValue.value)
	if (saveTimer.value) clearTimeout(saveTimer.value)
	saveTimer.value = setTimeout(() => {
		emit('save-note', props.sessionId, noteValue.value)
	}, 500)
}

function handleNoteBlur() {
	if (saveTimer.value) clearTimeout(saveTimer.value)
	emit('save-note', props.sessionId, noteValue.value)
}
</script>

<template>
	<div class="flex flex-col w-full h-full bg-white">
		<header class="flex items-center h-56px px-24px bg-white border-b border-gray-100 shrink-0 gap-16px">
			<div
				class="flex items-center gap-8px text-[#666666] cursor-pointer hover:text-gray-700"
				@click="emit('back')"
			>
				<svg class="w-18px h-18px" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M19 12H5M12 19l-7-7 7-7" />
				</svg>
				<span class="text-14px">Back</span>
			</div>
			<div class="flex-1"></div>
			<div v-if="sessionInfo" class="flex flex-col items-end gap-2px">
				<span class="text-15px font-600 text-gray-900">{{ projectName }}</span>
				<span class="text-11px font-mono text-gray-400">{{ metaInfo }}</span>
			</div>
		</header>

		<div class="h-1px bg-gray-200 shrink-0"></div>

		<main class="flex-1 overflow-y-auto bg-[#F8F9FA]">
			<div class="flex flex-col gap-20px py-20px px-48px">
				<div v-for="msg in messages" :key="msg.uuid" class="flex flex-col gap-6px">
					<div class="flex items-center gap-8px text-11px font-mono">
						<span :class="getRoleColor(msg.role)">{{ getRoleLabel(msg.role) }}</span>
						<span class="text-gray-400">·</span>
						<span class="text-gray-400">{{ formatTime(msg.timestamp) }}</span>
						<span v-if="msg.model" class="text-gray-400">· {{ msg.model }}</span>
					</div>

					<div
						v-if="getTextBlocks(msg.content)"
						class="text-14px text-gray-900 leading-relaxed whitespace-pre-wrap"
					>
						{{ getTextBlocks(msg.content) }}
					</div>

					<div v-for="tool in getToolUseBlocks(msg.content)" :key="tool.id" class="flex flex-col gap-6px">
						<div class="text-11px font-mono text-indigo-600">TOOL USE · {{ tool.name }}</div>
						<div
							class="p-12px bg-white border border-gray-200 rounded-6px font-mono text-12px text-gray-700 overflow-hidden break-all max-h-200px overflow-y-auto whitespace-pre-wrap"
						>
							{{ JSON.stringify(tool.input, null, 2) }}
						</div>
					</div>

					<div
						v-for="result in getToolResultBlocks(msg.content)"
						:key="result.tool_use_id"
						class="flex flex-col gap-6px"
					>
						<div class="text-11px font-mono text-gray-400">
							TOOL RESULT · {{ getToolNameFromResult(msg) || 'unknown' }}
						</div>
						<div
							class="p-12px bg-white border border-gray-200 rounded-6px font-mono text-12px text-gray-700 overflow-hidden break-all max-h-200px overflow-y-auto whitespace-pre-wrap"
						>
							{{ result.content }}
						</div>
					</div>

					<div v-for="(thinking, idx) in getThinkingBlocks(msg.content)" :key="idx">
						<div
							class="flex items-center gap-8px px-12px py-8px bg-gray-100 rounded-6px cursor-pointer w-fit"
							@click="toggleThinking(msg.uuid)"
						>
							<svg
								class="w-14px h-14px text-gray-400"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
							>
								<path
									d="M12 2a7 7 0 0 1 7 7c0 2.5-1.5 4.5-3 6-.5.5-1 1.5-1 2.5V19a1 1 0 0 1-1 1h-4a1 1 0 0 1-1-1v-1.5c0-1-.5-2-1-2.5-1.5-1.5-3-3.5-3-6a7 7 0 0 1 7-7z"
								/>
								<line x1="9" y1="21" x2="15" y2="21" />
							</svg>
							<span class="text-12px text-gray-400">
								{{ expandedThinking.has(msg.uuid) ? 'Collapse' : 'Thinking...' }}
							</span>
						</div>
						<div
							v-if="expandedThinking.has(msg.uuid)"
							class="mt-8px p-12px bg-white border border-gray-200 rounded-6px text-13px text-gray-600 leading-relaxed whitespace-pre-wrap overflow-hidden break-all"
						>
							{{ thinking }}
						</div>
					</div>

					<div
						v-for="(att, idx) in getAttachmentBlocks(msg.content)"
						:key="idx"
						class="text-12px text-gray-400"
					>
						[{{ att.attachment_type }}]
					</div>
				</div>
			</div>
		</main>

		<div class="flex flex-col gap-8px py-12px px-48px bg-white border-t border-gray-100 shrink-0">
			<span class="text-12px font-500 text-gray-500">Note</span>
			<div class="flex items-center px-12px py-10px bg-[#F9FAFB] border border-gray-200 rounded-6px">
				<input
					v-model="noteValue"
					type="text"
					class="flex-1 bg-transparent text-13px text-gray-900 outline-none placeholder:text-gray-400"
					placeholder="添加备注..."
					@input="handleNoteInput"
					@blur="handleNoteBlur"
				/>
			</div>
		</div>
	</div>
</template>
