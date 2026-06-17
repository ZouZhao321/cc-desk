<script setup lang="ts">
import ProviderCard from './ProviderCard.vue'
import AddProviderMenu from './AddProviderMenu.vue'
import type { Provider } from '../types'

defineProps<{
	providers: Provider[]
	loading: boolean
}>()

defineEmits<{
	add: []
	paste: []
	readCurrent: []
	sync: []
	edit: [provider: Provider]
	duplicate: [id: string]
	delete: [id: string]
	activate: [provider: Provider]
	session: []
}>()
</script>

<template>
	<div class="flex flex-col w-full h-full bg-white font-sans">
		<header class="flex items-center justify-between h-56px px-24px bg-white border-b border-gray-100 shrink-0">
			<div class="flex items-center gap-16px">
				<span class="text-20px font-bold text-[#FF6B35]">CC-Desk</span>
				<button
					class="flex items-center justify-center w-32px h-32px border-none bg-gray-100 rounded-6px cursor-pointer hover:bg-gray-200 transition-colors"
				>
					<svg
						width="18"
						height="18"
						viewBox="0 0 24 24"
						fill="none"
						stroke="#666666"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<path
							d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"
						/>
						<circle cx="12" cy="12" r="3" />
					</svg>
				</button>
				<button
					class="flex items-center justify-center w-32px h-32px border-none bg-gray-100 rounded-6px cursor-pointer hover:bg-gray-200 transition-colors"
					@click="$emit('sync')"
				>
					<svg
						width="18"
						height="18"
						viewBox="0 0 24 24"
						fill="none"
						stroke="#22C55E"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
						<path d="M3 3v5h5" />
						<path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16" />
						<path d="M16 16h5v5" />
					</svg>
				</button>
			</div>
			<div class="flex items-center gap-8px">
				<button
					class="flex items-center justify-center w-32px h-32px border-none bg-gray-100 rounded-6px cursor-pointer hover:bg-gray-200 transition-colors"
					@click="$emit('session')"
				>
					<svg
						width="18"
						height="18"
						viewBox="0 0 24 24"
						fill="none"
						stroke="#666666"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
					</svg>
				</button>
				<button
					class="flex items-center justify-center w-32px h-32px border-none bg-gray-100 rounded-6px cursor-pointer hover:bg-gray-200 transition-colors"
				>
					<svg
						width="18"
						height="18"
						viewBox="0 0 24 24"
						fill="none"
						stroke="#666666"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
						<line x1="3" y1="9" x2="21" y2="9" />
						<line x1="9" y1="21" x2="9" y2="9" />
					</svg>
				</button>
				<AddProviderMenu @add="$emit('add')" @paste="$emit('paste')" @read-current="$emit('readCurrent')" />
			</div>
		</header>

		<main class="flex-1 py-16px px-24px bg-[#F8F9FA] overflow-y-auto">
			<div v-if="loading" class="flex items-center justify-center h-200px text-gray-400">加载中...</div>
			<div
				v-else-if="providers.length === 0"
				class="flex flex-col items-center justify-center h-200px text-gray-400"
			>
				<p class="text-14px">暂无供应商配置</p>
				<p class="text-12px mt-8px">点击右上角 + 添加第一个供应商</p>
			</div>
			<div v-else class="flex flex-col gap-12px">
				<ProviderCard
					v-for="provider in providers"
					:key="provider.id"
					:provider="provider"
					@activate="$emit('activate', $event)"
					@edit="$emit('edit', $event)"
					@duplicate="$emit('duplicate', $event)"
					@delete="$emit('delete', $event)"
				/>
			</div>
		</main>
	</div>
</template>
