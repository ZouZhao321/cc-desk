import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { SessionMeta, Message } from '../types'

export function useSessionHistory() {
	const sessions = ref<SessionMeta[]>([])
	const currentMessages = ref<Message[]>([])
	const notes = ref<Record<string, string>>({})
	const lastMessages = ref<Record<string, string>>({})
	const loading = ref(false)
	const error = ref<string | null>(null)
	const selectedSessionId = ref<string | null>(null)

	const selectedSession = computed(() => {
		if (!selectedSessionId.value) return null
		return sessions.value.find(s => s.session_id === selectedSessionId.value) ?? null
	})

	const currentNote = computed(() => {
		if (!selectedSessionId.value) return ''
		return notes.value[selectedSessionId.value] ?? ''
	})

	async function loadSessions() {
		loading.value = true
		error.value = null
		try {
			const [sessionList, noteData] = await Promise.all([
				invoke<SessionMeta[]>('list_sessions'),
				invoke<Record<string, string>>('load_annotations')
			])
			sessions.value = sessionList
			notes.value = noteData

			// 加载所有会话的最后消息
			const lastMessagePromises = sessionList.map(async session => {
				try {
					const lastMsg = await invoke<string | null>('get_session_last_message', {
						sessionId: session.session_id
					})
					if (lastMsg) {
						lastMessages.value[session.session_id] = lastMsg
					}
				} catch {
					// 忽略单个会话的加载错误
				}
			})
			await Promise.all(lastMessagePromises)
		} catch (e) {
			error.value = String(e)
		} finally {
			loading.value = false
		}
	}

	async function loadSession(sessionId: string) {
		loading.value = true
		error.value = null
		selectedSessionId.value = sessionId
		try {
			const messages = await invoke<Message[]>('read_session', { sessionId })
			currentMessages.value = messages
		} catch (e) {
			error.value = String(e)
		} finally {
			loading.value = false
		}
	}

	async function saveNote(sessionId: string, note: string) {
		notes.value[sessionId] = note
		try {
			await invoke('save_annotation', { sessionId, note })
		} catch (e) {
			error.value = String(e)
		}
	}

	function clearSession() {
		selectedSessionId.value = null
		currentMessages.value = []
	}

	function getNote(sessionId: string): string {
		return notes.value[sessionId] ?? ''
	}

	function getLastMessage(sessionId: string): string {
		return lastMessages.value[sessionId] ?? ''
	}

	return {
		sessions,
		currentMessages,
		notes,
		lastMessages,
		loading,
		error,
		selectedSessionId,
		selectedSession,
		currentNote,
		loadSessions,
		loadSession,
		saveNote,
		clearSession,
		getNote,
		getLastMessage
	}
}
