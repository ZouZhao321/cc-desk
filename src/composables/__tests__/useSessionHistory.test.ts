import { describe, it, expect, vi, beforeEach } from 'vitest'
import { useSessionHistory } from '../useSessionHistory'

vi.mock('@tauri-apps/api/core', () => ({
	invoke: vi.fn()
}))

import { invoke } from '@tauri-apps/api/core'
const mockInvoke = vi.mocked(invoke)

beforeEach(() => {
	vi.clearAllMocks()
})

describe('useSessionHistory', () => {
	describe('loadSessions', () => {
		it('loads sessions and notes', async () => {
			const sessions = [
				{
					session_id: 's1',
					project_path: 'proj-a',
					started_at: '2026-06-14T10:00:00Z',
					message_count: 5,
					version: '1.0'
				},
				{
					session_id: 's2',
					project_path: 'proj-b',
					started_at: '2026-06-14T09:00:00Z',
					message_count: 3,
					version: null
				}
			]
			const notes = { s1: 'my note' }

			mockInvoke.mockImplementation(async (cmd: string) => {
				if (cmd === 'list_sessions') return sessions
				if (cmd === 'load_annotations') return notes
				return null
			})

			const { loadSessions, sessions: state, error } = useSessionHistory()
			await loadSessions()

			expect(error.value).toBeNull()
			expect(state.value).toHaveLength(2)
			expect(state.value[0].session_id).toBe('s1')
		})

		it('sets error on failure', async () => {
			mockInvoke.mockRejectedValue(new Error('scan failed'))

			const { loadSessions, sessions, error } = useSessionHistory()
			await loadSessions()

			expect(error.value).toBe('Error: scan failed')
			expect(sessions.value).toHaveLength(0)
		})
	})

	describe('loadSession', () => {
		it('loads messages for a session', async () => {
			const messages = [
				{
					uuid: 'u1',
					parent_uuid: null,
					timestamp: '2026-06-14T10:00:00Z',
					role: 'user',
					content: [{ type: 'text', text: 'hello' }],
					model: null,
					is_sidechain: false
				}
			]

			mockInvoke.mockImplementation(async (cmd: string) => {
				if (cmd === 'list_sessions') return []
				if (cmd === 'load_annotations') return {}
				if (cmd === 'read_session') return messages
				return null
			})

			const { loadSessions, loadSession, currentMessages, selectedSessionId } = useSessionHistory()
			await loadSessions()
			await loadSession('s1')

			expect(selectedSessionId.value).toBe('s1')
			expect(currentMessages.value).toHaveLength(1)
			expect(currentMessages.value[0].uuid).toBe('u1')
		})

		it('sets error when session not found', async () => {
			mockInvoke.mockImplementation(async (cmd: string) => {
				if (cmd === 'list_sessions') return []
				if (cmd === 'load_annotations') return {}
				if (cmd === 'read_session') throw new Error('未找到会话: bad-id')
				return null
			})

			const { loadSessions, loadSession, error } = useSessionHistory()
			await loadSessions()
			await loadSession('bad-id')

			expect(error.value).toContain('未找到会话')
		})
	})

	describe('saveNote', () => {
		it('saves note via Rust command', async () => {
			mockInvoke.mockImplementation(async (cmd: string) => {
				if (cmd === 'list_sessions') return []
				if (cmd === 'load_annotations') return {}
				if (cmd === 'save_annotation') return undefined
				return null
			})

			const { loadSessions, saveNote, notes } = useSessionHistory()
			await loadSessions()
			await saveNote('s1', 'test note')

			expect(notes.value['s1']).toBe('test note')
			expect(mockInvoke).toHaveBeenCalledWith('save_annotation', { sessionId: 's1', note: 'test note' })
		})

		it('sets error when save fails', async () => {
			mockInvoke.mockImplementation(async (cmd: string) => {
				if (cmd === 'list_sessions') return []
				if (cmd === 'load_annotations') return {}
				if (cmd === 'save_annotation') throw new Error('write failed')
				return null
			})

			const { loadSessions, saveNote, error } = useSessionHistory()
			await loadSessions()
			await saveNote('s1', 'note')

			expect(error.value).toBe('Error: write failed')
		})
	})

	describe('getNote', () => {
		it('returns note for existing session', async () => {
			mockInvoke.mockImplementation(async (cmd: string) => {
				if (cmd === 'list_sessions') return []
				if (cmd === 'load_annotations') return { s1: 'hello' }
				return null
			})

			const { loadSessions, getNote } = useSessionHistory()
			await loadSessions()

			expect(getNote('s1')).toBe('hello')
			expect(getNote('nonexistent')).toBe('')
		})
	})

	describe('clearSession', () => {
		it('resets selectedSessionId and currentMessages', async () => {
			const messages = [
				{
					uuid: 'u1',
					parent_uuid: null,
					timestamp: '2026-06-14T10:00:00Z',
					role: 'user',
					content: [{ type: 'text', text: 'hi' }],
					model: null,
					is_sidechain: false
				}
			]

			mockInvoke.mockImplementation(async (cmd: string) => {
				if (cmd === 'list_sessions') return []
				if (cmd === 'load_annotations') return {}
				if (cmd === 'read_session') return messages
				return null
			})

			const { loadSessions, loadSession, clearSession, selectedSessionId, currentMessages } = useSessionHistory()
			await loadSessions()
			await loadSession('s1')

			expect(selectedSessionId.value).toBe('s1')
			expect(currentMessages.value).toHaveLength(1)

			clearSession()

			expect(selectedSessionId.value).toBeNull()
			expect(currentMessages.value).toHaveLength(0)
		})
	})
})
