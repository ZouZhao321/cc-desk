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
