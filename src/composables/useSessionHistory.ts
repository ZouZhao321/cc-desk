import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { load } from '@tauri-apps/plugin-store'
import type { SessionMeta, Message, NoteStore } from '../types'

const STORE_FILE = 'notes.json'

export function useSessionHistory() {
	const sessions = ref<SessionMeta[]>([])
	const currentMessages = ref<Message[]>([])
	const notes = ref<Record<string, string>>({})
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
			const [sessionList, noteStore] = await Promise.all([invoke<SessionMeta[]>('list_sessions'), loadNotes()])
			sessions.value = sessionList
			notes.value = noteStore
		} catch (e) {
			error.value = String(e)
		} finally {
			loading.value = false
		}
	}

	async function loadNotes(): Promise<Record<string, string>> {
		try {
			const store = await load(STORE_FILE, { defaults: {}, autoSave: false })
			const data = await store.get<NoteStore>('data')
			return data?.notes ?? {}
		} catch {
			return {}
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
			const store = await load(STORE_FILE, { defaults: {}, autoSave: false })
			const data: NoteStore = { notes: notes.value }
			await store.set('data', data)
			await store.save()
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

	return {
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
		clearSession,
		getNote
	}
}
