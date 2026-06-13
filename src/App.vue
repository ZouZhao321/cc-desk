<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { NConfigProvider, NMessageProvider } from 'naive-ui'
import ConfigListMain from './components/ConfigListMain.vue'
import { useSettings } from './composables/useSettings'

const { loadConfig } = useSettings()

const activeScope = ref('global')

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

function handleSelectScope(id: string) {
	activeScope.value = id
	scopes.value.forEach(s => (s.isActive = s.id === id))
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

onMounted(loadConfig)
</script>

<template>
	<n-config-provider>
		<n-message-provider>
			<ConfigListMain
				:scopes="scopes"
				@select="handleSelectScope"
				@add="handleAdd"
				@settings="handleSettings"
				@sync="handleSync"
			/>
		</n-message-provider>
	</n-config-provider>
</template>
