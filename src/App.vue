<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { NConfigProvider, NMessageProvider } from 'naive-ui'
import ConfigListMain from './components/ConfigListMain.vue'
import ConfigDetail from './components/ConfigDetail.vue'
import SessionHistory from './components/SessionHistory.vue'
import { useSettings } from './composables/useSettings'
import type { InheritedItem, ProjectOverride, ProjectListItem } from './types'

const { loadConfig } = useSettings()

const activePage = ref<'config' | 'sessions'>('config')
const currentView = ref<'list' | 'detail'>('list')
const sessionDetailView = ref(false)
const activeScope = ref('global')
const activeTab = ref<'project' | 'skills' | 'mcp' | 'plugins'>('skills')

const configName = computed(() => {
	const scope = scopes.value.find(s => s.id === activeScope.value)
	if (!scope) return 'Configuration'
	if (activeTab.value === 'project') return `${scope.name} Configuration`
	return `${scope.name} - ${activeTab.value.charAt(0).toUpperCase() + activeTab.value.slice(1)}`
})

const scopes = ref([
	{
		id: 'global',
		name: 'Global',
		icon: '🌐',
		iconColor: '#6366F1',
		iconBg: '#6366F115',
		description: 'Default settings for all projects',
		stats: [
			{ label: 'Skills', value: 6 },
			{ label: 'MCP', value: 5 },
			{ label: 'Plugins', value: 3 }
		],
		badge: 'Default',
		badgeColor: '#6366F1',
		isActive: true
	},
	{
		id: 'my-project',
		name: 'My Project',
		icon: '📁',
		iconColor: '#10B981',
		iconBg: '#10B98115',
		description: 'Project-specific overrides',
		stats: [
			{ label: 'Skills', value: 2 },
			{ label: 'MCP', value: 3 },
			{ label: 'Plugins', value: 2 },
			{ label: 'project', value: 1 }
		]
	},
	{
		id: 'team-shared',
		name: 'Team Shared',
		icon: '👥',
		iconColor: '#F59E0B',
		iconBg: '#F59E0B15',
		description: 'Shared team configuration',
		stats: [
			{ label: 'Skills', value: 4 },
			{ label: 'MCP', value: 4 },
			{ label: 'Plugins', value: 4 },
			{ label: 'projects', value: 2 }
		]
	}
])

const skillsItems = ref([
	{
		id: 'code-review',
		name: 'Code Review',
		description: 'Automated code analysis and suggestions',
		icon: '🔍',
		iconColor: '#3B82F6',
		iconBg: '#EFF6FF',
		active: true
	},
	{
		id: 'documentation',
		name: 'Documentation',
		description: 'Generate documentation from code comments',
		icon: '📄',
		iconColor: '#3B82F6',
		iconBg: '#EFF6FF',
		active: true
	},
	{
		id: 'testing',
		name: 'Testing',
		description: 'Write and run unit tests automatically',
		icon: '🧪',
		iconColor: '#3B82F6',
		iconBg: '#EFF6FF',
		active: true
	},
	{
		id: 'refactoring',
		name: 'Refactoring',
		description: 'Code improvement and optimization suggestions',
		icon: '🔄',
		iconColor: '#9CA3AF',
		iconBg: '#F3F4F6',
		active: false
	},
	{
		id: 'debugging',
		name: 'Debugging',
		description: 'Error diagnosis and fix recommendations',
		icon: '🐛',
		iconColor: '#9CA3AF',
		iconBg: '#F3F4F6',
		active: false
	},
	{
		id: 'deployment',
		name: 'Deployment',
		description: 'CI/CD pipeline integration and automation',
		icon: '🚀',
		iconColor: '#9CA3AF',
		iconBg: '#F3F4F6',
		active: false
	}
])

const mcpItems = ref([
	{
		id: 'github',
		name: 'GitHub Integration',
		description: 'Connect to GitHub repos, manage PRs',
		icon: '🔀',
		iconColor: '#3B82F6',
		iconBg: '#EFF6FF',
		active: true,
		version: 'v2.1.0'
	},
	{
		id: 'docker',
		name: 'Docker Manager',
		description: 'Container lifecycle management',
		icon: '📦',
		iconColor: '#3B82F6',
		iconBg: '#EFF6FF',
		active: true,
		version: 'v1.4.2'
	},
	{
		id: 'api-explorer',
		name: 'API Explorer',
		description: 'REST & GraphQL API testing',
		icon: '🌐',
		iconColor: '#9CA3AF',
		iconBg: '#F3F4F6',
		active: false,
		version: 'v1.8.0'
	}
])

const pluginsItems = ref([
	{
		id: 'gh-plugin',
		name: 'GitHub Integration',
		description: 'Connect to GitHub repos, manage PRs',
		icon: '🔀',
		iconColor: '#3B82F6',
		iconBg: '#EFF6FF',
		active: true,
		version: 'v2.1.0'
	},
	{
		id: 'docker-plugin',
		name: 'Docker Manager',
		description: 'Container lifecycle management',
		icon: '📦',
		iconColor: '#3B82F6',
		iconBg: '#EFF6FF',
		active: true,
		version: 'v1.4.2'
	},
	{
		id: 'api-plugin',
		name: 'API Explorer',
		description: 'REST & GraphQL API testing',
		icon: '🌐',
		iconColor: '#9CA3AF',
		iconBg: '#F3F4F6',
		active: false,
		version: 'v1.8.0'
	}
])

const inheritedSkills = ref<InheritedItem[]>([
	{
		id: 'i-code-review',
		name: 'Code Review',
		description: 'Automated code analysis',
		icon: '🔍',
		iconColor: '#3B82F6',
		iconBg: '#EFF6FF',
		active: true
	},
	{
		id: 'i-documentation',
		name: 'Documentation',
		description: 'Generate docs from code',
		icon: '📄',
		iconColor: '#3B82F6',
		iconBg: '#EFF6FF',
		active: true
	},
	{
		id: 'i-testing',
		name: 'Testing',
		description: 'Write and run unit tests',
		icon: '🧪',
		iconColor: '#3B82F6',
		iconBg: '#EFF6FF',
		active: true
	}
])

const inheritedMcp = ref<InheritedItem[]>([
	{
		id: 'i-github',
		name: 'GitHub Integration',
		description: 'Connect to GitHub repos',
		icon: '🔀',
		iconColor: '#3B82F6',
		iconBg: '#EFF6FF',
		active: true,
		version: 'v2.1.0'
	},
	{
		id: 'i-docker',
		name: 'Docker Manager',
		description: 'Container lifecycle',
		icon: '📦',
		iconColor: '#3B82F6',
		iconBg: '#EFF6FF',
		active: true,
		version: 'v1.4.2'
	}
])

const inheritedPlugins = ref<InheritedItem[]>([
	{
		id: 'i-gh-plugin',
		name: 'GitHub Integration',
		description: 'Connect to GitHub repos',
		icon: '🔀',
		iconColor: '#3B82F6',
		iconBg: '#EFF6FF',
		active: true,
		version: 'v2.1.0'
	},
	{
		id: 'i-docker-plugin',
		name: 'Docker Manager',
		description: 'Container lifecycle',
		icon: '📦',
		iconColor: '#3B82F6',
		iconBg: '#EFF6FF',
		active: true,
		version: 'v1.4.2'
	}
])

const projectOverrides = ref<ProjectOverride[]>([
	{
		id: 'temp-override',
		label: 'Temperature',
		description: 'Global: 0.7 → Project: 0.5',
		globalValue: '0.7',
		projectValue: '0.5'
	}
])

const projectSkills = ref([
	{
		id: 'p-debugging',
		name: 'Debugging',
		description: 'Error diagnosis and fix',
		icon: '🐛',
		iconColor: '#3B82F6',
		iconBg: '#EFF6FF',
		active: true
	},
	{
		id: 'p-refactoring',
		name: 'Refactoring',
		description: 'Code improvement suggestions',
		icon: '🔄',
		iconColor: '#9CA3AF',
		iconBg: '#F3F4F6',
		active: false
	}
])

const projectPlugins = ref([
	{
		id: 'p-api-plugin',
		name: 'API Explorer',
		description: 'REST & GraphQL API testing',
		icon: '🌐',
		iconColor: '#9CA3AF',
		iconBg: '#F3F4F6',
		active: false
	}
])

const projects = ref<ProjectListItem[]>([
	{ id: 'proj-1', name: 'my-app', path: '~/projects/my-app', language: 'TypeScript', syncTime: '2 min ago' },
	{ id: 'proj-2', name: 'api-server', path: '~/projects/api-server', language: 'Python', syncTime: '1 hour ago' },
	{ id: 'proj-3', name: 'mobile-app', path: '~/projects/mobile-app', language: 'Swift', syncTime: '3 hours ago' }
])

const currentItems = computed(() => {
	if (activeScope.value === 'global') {
		switch (activeTab.value) {
			case 'skills':
				return skillsItems.value
			case 'mcp':
				return mcpItems.value
			case 'plugins':
				return pluginsItems.value
			default:
				return []
		}
	}
	switch (activeTab.value) {
		case 'skills':
			return projectSkills.value
		case 'mcp':
			return []
		case 'plugins':
			return projectPlugins.value
		default:
			return []
	}
})

const currentInherited = computed(() => {
	if (activeScope.value === 'global') return []
	switch (activeTab.value) {
		case 'skills':
			return inheritedSkills.value
		case 'mcp':
			return inheritedMcp.value
		case 'plugins':
			return inheritedPlugins.value
		default:
			return []
	}
})

const currentInheritedCount = computed(() => {
	switch (activeTab.value) {
		case 'skills':
			return `${inheritedSkills.value.filter(i => i.active).length} active`
		case 'mcp':
			return `${inheritedMcp.value.length} params`
		case 'plugins':
			return `${inheritedPlugins.value.filter(i => i.active).length} active`
		default:
			return ''
	}
})

function handleSelectScope(id: string) {
	activeScope.value = id
	scopes.value.forEach(s => (s.isActive = s.id === id))
	if (id === 'global') {
		activeTab.value = 'skills'
	} else {
		activeTab.value = 'project'
	}
	currentView.value = 'detail'
}

function handleBack() {
	currentView.value = 'list'
}

function handleTabChange(tab: 'project' | 'skills' | 'mcp' | 'plugins') {
	activeTab.value = tab
}

function handleToggle(id: string) {
	const items = activeTab.value === 'skills' ? skillsItems : activeTab.value === 'mcp' ? mcpItems : pluginsItems
	const item = items.value.find(i => i.id === id)
	if (item) item.active = !item.active
}

function handleSave() {
	// TODO: 保存配置
}

function handleReset() {
	// TODO: 重置配置
}

function handleOverrideUpdate(id: string, value: string) {
	const override = projectOverrides.value.find(o => o.id === id)
	if (override) override.projectValue = value
}

function handleOverrideRemove(id: string) {
	projectOverrides.value = projectOverrides.value.filter(o => o.id !== id)
}

function handleAdd() {
	// TODO: 打开新建 scope 弹窗
}

function handleSettings() {
	// TODO: 打开设置页面
}

function handleSync() {
	// TODO: 同步配置
}

function handleSessionDetailChange(isDetail: boolean) {
	sessionDetailView.value = isDetail
}

onMounted(loadConfig)
</script>

<template>
	<n-config-provider>
		<n-message-provider>
			<div class="flex flex-col w-full h-full bg-white font-sans">
				<!-- 顶部 Tab 栏 -->
				<div
					v-if="!(activePage === 'sessions' && sessionDetailView)"
					class="flex items-center h-40px px-24px bg-white border-b border-gray-100 shrink-0 gap-4px"
				>
					<button
						class="flex items-center gap-6px px-12px py-6px rounded-6px text-13px transition-colors"
						:class="
							activePage === 'config' ? 'bg-indigo-500 text-white' : 'text-gray-600 hover:bg-gray-100'
						"
						@click="activePage = 'config'"
					>
						<svg
							class="w-14px h-14px"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<circle cx="12" cy="12" r="3" />
							<path
								d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"
							/>
						</svg>
						Config
					</button>
					<button
						class="flex items-center gap-6px px-12px py-6px rounded-6px text-13px transition-colors"
						:class="
							activePage === 'sessions' ? 'bg-indigo-500 text-white' : 'text-gray-600 hover:bg-gray-100'
						"
						@click="activePage = 'sessions'"
					>
						<svg
							class="w-14px h-14px"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
						</svg>
						Sessions
					</button>
				</div>

				<!-- 内容区 -->
				<div class="flex-1 overflow-hidden">
					<ConfigListMain
						v-if="activePage === 'config' && currentView === 'list'"
						:scopes="scopes"
						@select="handleSelectScope"
						@add="handleAdd"
						@settings="handleSettings"
						@sync="handleSync"
					/>
					<ConfigDetail
						v-else-if="activePage === 'config' && currentView === 'detail'"
						:config-name="configName"
						:active-tab="activeTab"
						:items="currentItems"
						:mode="activeScope === 'global' ? 'global' : 'project'"
						:inherited-items="currentInherited"
						:overrides="activeTab === 'mcp' ? projectOverrides : undefined"
						:projects="activeTab === 'project' ? projects : undefined"
						:inherited-count="currentInheritedCount"
						@back="handleBack"
						@tab-change="handleTabChange"
						@toggle="handleToggle"
						@save="handleSave"
						@reset="handleReset"
						@override-update="handleOverrideUpdate"
						@override-remove="handleOverrideRemove"
					/>
					<SessionHistory v-else-if="activePage === 'sessions'" @detail-change="handleSessionDetailChange" />
				</div>
			</div>
		</n-message-provider>
	</n-config-provider>
</template>
