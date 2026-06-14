<script setup lang="ts">
import { computed } from 'vue'
import InheritedSection from './InheritedSection.vue'
import ProjectOverride from './ProjectOverride.vue'
import ProjectList from './ProjectList.vue'
import type { InheritedItem, ProjectOverride as OverrideType, ProjectListItem } from '../types'

interface DetailItem {
	id: string
	name: string
	description: string
	icon: string
	iconColor: string
	iconBg: string
	active: boolean
	version?: string
}

const props = defineProps<{
	configName: string
	activeTab: 'project' | 'skills' | 'mcp' | 'plugins'
	items: DetailItem[]
	mode?: 'global' | 'project'
	inheritedItems?: InheritedItem[]
	overrides?: OverrideType[]
	projects?: ProjectListItem[]
	inheritedCount?: string
}>()

const emit = defineEmits<{
	back: []
	'tab-change': [tab: 'project' | 'skills' | 'mcp' | 'plugins']
	toggle: [id: string]
	save: []
	reset: []
	'override-update': [id: string, value: string]
	'override-remove': [id: string]
}>()

const isProject = computed(() => props.mode === 'project')

const tabs = computed(() => {
	const base: { key: 'project' | 'skills' | 'mcp' | 'plugins'; label: string; icon: string }[] = [
		{ key: 'skills', label: 'Skills', icon: '🧩' },
		{ key: 'mcp', label: 'MCP', icon: '⚙️' },
		{ key: 'plugins', label: 'Plugins', icon: '🧱' }
	]
	if (isProject.value) {
		base.unshift({ key: 'project', label: 'Project', icon: '📁' })
	}
	return base
})

const headerInfo = computed(() => {
	switch (props.activeTab) {
		case 'project':
			return { title: 'Projects Using This Configuration', count: `${props.projects?.length || 0} projects` }
		case 'skills':
			return { title: 'Skills Configuration', count: `${props.items.filter(i => i.active).length} active` }
		case 'mcp':
			return { title: 'MCP Configuration', count: 'Model control parameters' }
		case 'plugins':
			return { title: 'Plugins Configuration', count: `${props.items.filter(i => i.active).length} installed` }
		default:
			return { title: 'Configuration', count: '' }
	}
})

const isLargeItem = computed(() => props.activeTab !== 'skills')
</script>

<template>
	<div class="flex flex-col w-full h-full bg-white font-sans">
		<header class="flex items-center h-56px px-24px bg-white border-b border-gray-200 shrink-0 gap-16px">
			<button
				class="flex items-center gap-8px text-gray-500 hover:text-gray-700 transition-colors bg-transparent border-none cursor-pointer text-14px"
				@click="emit('back')"
			>
				<svg
					width="18"
					height="18"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<path d="m12 19-7-7 7-7" />
					<path d="M19 12H5" />
				</svg>
				Back
			</button>
			<span class="text-16px font-600 text-gray-900">{{ configName }}</span>
			<span v-if="isProject" class="text-11px font-500 text-amber-600 bg-amber-50 px-10px py-4px rounded-12px">
				Inherits from Global
			</span>
			<div class="flex-1 flex items-center justify-end gap-12px">
				<button
					class="flex items-center gap-6px px-16px py-8px bg-blue-500 text-white rounded-6px border-none cursor-pointer text-13px font-500 hover:bg-blue-600 transition-colors"
					@click="emit('save')"
				>
					<svg
						width="14"
						height="14"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<polyline points="20 6 9 17 4 12" />
					</svg>
					Save
				</button>
				<button
					class="flex items-center gap-6px px-16px py-8px bg-gray-100 text-gray-500 rounded-6px border-none cursor-pointer text-13px hover:bg-gray-200 transition-colors"
					@click="emit('reset')"
				>
					<svg
						width="14"
						height="14"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
						<path d="M3 3v5h5" />
					</svg>
					Reset
				</button>
			</div>
		</header>

		<div class="flex border-b border-gray-200 bg-white shrink-0">
			<button
				v-for="tab in tabs"
				:key="tab.key"
				class="flex items-center gap-8px px-20px py-12px border-none cursor-pointer text-14px transition-colors"
				:class="
					activeTab === tab.key
						? 'bg-white text-blue-500 font-500'
						: 'bg-transparent text-gray-500 hover:text-gray-700'
				"
				@click="emit('tab-change', tab.key)"
			>
				<span>{{ tab.icon }}</span>
				{{ tab.label }}
			</button>
		</div>

		<main class="flex-1 bg-[#F9FAFB] overflow-y-auto">
			<div v-if="activeTab === 'project'" class="flex flex-col gap-20px p-24px">
				<div class="flex items-center justify-between">
					<span class="text-18px font-600 text-gray-900">{{ headerInfo.title }}</span>
					<span class="text-13px text-gray-500">{{ headerInfo.count }}</span>
				</div>
				<ProjectList v-if="projects" :projects="projects" />
			</div>

			<div v-else class="flex flex-col gap-16px p-20px px-24px">
				<InheritedSection
					v-if="isProject && inheritedItems && inheritedItems.length > 0"
					title="Inherited from Global"
					:count="inheritedCount || ''"
					:items="inheritedItems"
					:show-lock="true"
				/>

				<div v-if="isProject && inheritedItems && inheritedItems.length > 0" class="h-1px bg-gray-200" />

				<div
					v-if="isProject && activeTab === 'mcp' && overrides && overrides.length > 0"
					class="flex flex-col gap-12px"
				>
					<div class="flex items-center justify-between">
						<span class="text-15px font-600 text-gray-900">Project Overrides</span>
						<span class="text-12px text-gray-500">{{ overrides.length }} override</span>
					</div>
					<ProjectOverride
						v-for="override in overrides"
						:key="override.id"
						:override="override"
						@update="(id, val) => emit('override-update', id, val)"
						@remove="id => emit('override-remove', id)"
					/>
				</div>

				<div class="flex flex-col gap-12px">
					<div v-if="isProject" class="flex items-center justify-between">
						<span class="text-15px font-600 text-gray-900">
							{{
								activeTab === 'skills'
									? 'Project Skills'
									: activeTab === 'mcp'
										? 'Project MCP'
										: 'Project Plugins'
							}}
						</span>
						<span class="text-12px text-gray-500">
							{{ items.filter(i => i.active).length }}
							{{ activeTab === 'plugins' ? 'installed' : 'custom' }}
						</span>
					</div>
					<div v-if="!isProject" class="flex items-center justify-between">
						<span class="text-18px font-600 text-gray-900">{{ headerInfo.title }}</span>
						<span class="text-13px text-gray-500">{{ headerInfo.count }}</span>
					</div>

					<div class="flex flex-col" :class="isLargeItem ? 'gap-12px' : 'gap-10px'">
						<div
							v-for="item in items"
							:key="item.id"
							class="flex items-center bg-white rounded-10px border border-gray-200 cursor-pointer transition-all hover:border-gray-300"
							:class="isLargeItem ? 'gap-16px p-16px pr-20px' : 'gap-14px p-14px pr-16px'"
						>
							<div
								class="flex items-center justify-center rounded-lg shrink-0"
								:class="
									isLargeItem ? 'w-44px h-44px rounded-10px text-22px' : 'w-40px h-40px text-20px'
								"
								:style="{ backgroundColor: item.iconBg }"
							>
								<span :style="{ color: item.iconColor }">{{ item.icon }}</span>
							</div>

							<div class="flex-1 flex flex-col min-w-0" :class="isLargeItem ? 'gap-4px' : 'gap-3px'">
								<span class="text-14px font-500 text-gray-900">{{ item.name }}</span>
								<span class="text-12px text-gray-500">{{ item.description }}</span>
								<span v-if="item.version" class="text-11px text-gray-400 font-mono">{{
									item.version
								}}</span>
							</div>

							<span
								class="text-11px font-500 px-10px py-4px rounded-12px shrink-0"
								:class="item.active ? 'text-emerald-600 bg-emerald-50' : 'text-gray-500 bg-gray-100'"
							>
								{{ item.active ? 'Active' : 'Inactive' }}
							</span>

							<button
								class="flex items-center rounded-11px border-none cursor-pointer p-2px shrink-0 transition-colors w-40px h-22px"
								:class="item.active ? 'bg-blue-500 justify-end' : 'bg-gray-300 justify-start'"
								@click.stop="emit('toggle', item.id)"
							>
								<span class="w-18px h-18px bg-white rounded-full shadow-sm" />
							</button>
						</div>
					</div>
				</div>
			</div>
		</main>
	</div>
</template>
