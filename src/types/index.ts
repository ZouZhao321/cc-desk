/** ~/.claude/settings.json 中的模型相关字段 */
export interface ModelConfig {
	auth_token: string
	base_url: string
	model: string
	reasoning_model: string
	haiku_id: string
	haiku_name: string
	sonnet_id: string
	sonnet_name: string
	opus_id: string
	opus_name: string
}

/** 已保存的预设 */
export interface Preset {
	id: string
	name: string
	config: ModelConfig
}

/** 预设存储数据（通过 tauri-plugin-store 持久化） */
export interface PresetStore {
	presets: Preset[]
	active_preset_id: string | null
}

/** 单个配置卡片的元数据 */
export interface ConfigField {
	key: keyof ModelConfig
	label: string
	icon: string
	sensitive: boolean
}

/** 继承的配置项（只读） */
export interface InheritedItem {
	id: string
	name: string
	description: string
	icon: string
	iconColor: string
	iconBg: string
	active: boolean
	version?: string
}

/** 项目 Override（MCP 专用） */
export interface ProjectOverride {
	id: string
	label: string
	description: string
	globalValue: string
	projectValue: string
}

/** 项目列表项 */
export interface ProjectListItem {
	id: string
	name: string
	path: string
	language: string
	syncTime: string
}

/** 会话元数据（列表页使用） */
export interface SessionMeta {
	session_id: string
	project_path: string
	started_at: string | null
	message_count: number
	version: string | null
	last_message?: string
}

/** 消息角色 */
export type MessageRole = 'user' | 'assistant' | 'system'

/** 消息内容块 */
export type ContentBlock =
	| { type: 'text'; text: string }
	| { type: 'thinking'; thinking: string }
	| { type: 'tool_use'; id: string; name: string; input: unknown }
	| { type: 'tool_result'; tool_use_id: string; content: string }
	| { type: 'attachment'; attachment_type: string; content: string }

/** 完整消息（详情页使用） */
export interface Message {
	uuid: string
	parent_uuid: string | null
	timestamp: string
	role: MessageRole
	content: ContentBlock[]
	model: string | null
	is_sidechain: boolean
}

/** 供应商配置 */
export interface Provider {
	id: string
	name: string
	notes?: string
	website?: string
	api_key: string
	base_url: string
	main_model: string
	opus_model: string
	sonnet_model: string
	haiku_model: string
	sub_agent_model: string
	reasoning_level: string
	is_active: boolean
}

/** 供应商存储数据 */
export interface ProviderStore {
	providers: Provider[]
}
