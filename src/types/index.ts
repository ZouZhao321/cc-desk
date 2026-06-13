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
