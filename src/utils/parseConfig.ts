export interface ParsedConfig {
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
}

const PROVIDER_MAP: Record<string, { name: string; website: string }> = {
	'deepseek.com': { name: 'DeepSeek', website: 'https://platform.deepseek.com' },
	'xiaomimimo.com': { name: 'Xiaomi MiMo', website: 'https://platform.xiaomimimo.com' },
	mimo: { name: 'Xiaomi MiMo', website: 'https://platform.xiaomimimo.com' },
	'openai.com': { name: 'OpenAI', website: 'https://platform.openai.com' },
	'anthropic.com': { name: 'Anthropic', website: 'https://console.anthropic.com' },
	'googleapis.com': { name: 'Google Gemini', website: 'https://ai.google.dev' },
	gemini: { name: 'Google Gemini', website: 'https://ai.google.dev' },
	'groq.com': { name: 'Groq', website: 'https://console.groq.com' },
	'mistral.ai': { name: 'Mistral', website: 'https://console.mistral.ai' },
	'cohere.com': { name: 'Cohere', website: 'https://dashboard.cohere.com' }
}

export function inferProviderInfo(baseUrl: string): { name: string; website?: string } {
	try {
		const url = new URL(baseUrl)
		const hostname = url.hostname.toLowerCase()

		for (const [keyword, info] of Object.entries(PROVIDER_MAP)) {
			if (hostname.includes(keyword)) {
				return info
			}
		}

		const domain = hostname.replace(/^www\./, '').split('.')[0]
		return { name: domain.charAt(0).toUpperCase() + domain.slice(1) }
	} catch {
		return { name: 'Unknown Provider' }
	}
}

function deriveMainModel(anthropicModel: string, opus: string, sonnet: string, haiku: string): string {
	if (anthropicModel && anthropicModel === opus) return 'opus'
	if (anthropicModel && anthropicModel === sonnet) return 'sonnet'
	if (anthropicModel && anthropicModel === haiku) return 'haiku'
	return 'sonnet'
}

export function parseConfigText(text: string): ParsedConfig {
	const env: Record<string, string> = {}
	const lines = text.split('\n')

	for (const line of lines) {
		const trimmed = line.trim()
		if (!trimmed || trimmed.startsWith('#')) continue

		// Linux/Mac: export KEY=VALUE
		const exportMatch = trimmed.match(/^export\s+(\w+)=(.+)$/)
		if (exportMatch) {
			env[exportMatch[1]] = exportMatch[2].replace(/^["']|["']$/g, '')
			continue
		}

		// Windows PowerShell: $env:KEY="VALUE"
		const psMatch = trimmed.match(/^\$env:(\w+)=(.+)$/)
		if (psMatch) {
			env[psMatch[1]] = psMatch[2].replace(/^["']|["']$/g, '')
			continue
		}
	}

	return parseSettingsEnv(env)
}

export function parseSettingsEnv(env: Record<string, string>): ParsedConfig {
	const baseUrl = env.ANTHROPIC_BASE_URL || ''
	const apiKey = env.ANTHROPIC_AUTH_TOKEN || ''
	const anthropicModel = env.ANTHROPIC_MODEL || ''
	const opusModel = env.ANTHROPIC_DEFAULT_OPUS_MODEL || ''
	const sonnetModel = env.ANTHROPIC_DEFAULT_SONNET_MODEL || ''
	const haikuModel = env.ANTHROPIC_DEFAULT_HAIKU_MODEL || ''
	const subAgentModel = env.CLAUDE_CODE_SUBAGENT_MODEL || env.ANTHROPIC_REASONING_MODEL || ''
	const reasoningLevel = env.CLAUDE_CODE_EFFORT_LEVEL || 'max'

	const providerInfo = inferProviderInfo(baseUrl)
	const mainModel = deriveMainModel(anthropicModel, opusModel, sonnetModel, haikuModel)

	return {
		name: providerInfo.name,
		website: providerInfo.website,
		api_key: apiKey,
		base_url: baseUrl,
		main_model: mainModel,
		opus_model: opusModel,
		sonnet_model: sonnetModel,
		haiku_model: haikuModel,
		sub_agent_model: subAgentModel,
		reasoning_level: reasoningLevel
	}
}
