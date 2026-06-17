<script setup lang="ts">
import ProviderCard from './ProviderCard.vue'
import type { Provider } from '../types'

defineProps<{
	providers: Provider[]
	loading: boolean
}>()

defineEmits<{
	add: []
	edit: [provider: Provider]
	duplicate: [id: string]
	delete: [id: string]
	activate: [provider: Provider]
}>()
</script>

<template>
	<div class="flex flex-col w-full h-full bg-white font-sans">
		<header class="flex items-center justify-between h-56px px-24px bg-white border-b border-gray-100 shrink-0">
			<div class="flex items-center gap-16px">
				<span class="text-20px font-bold text-[#FF6B35]">CC-Desk</span>
			</div>
			<div class="flex items-center gap-8px">
				<button
					class="flex items-center justify-center w-36px h-36px border-none bg-[#FF6B35] rounded-full cursor-pointer hover:opacity-90 transition-opacity"
					@click="$emit('add')"
				>
					<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#FFFFFF" stroke-width="2">
						<path d="M5 12h14" />
						<path d="M12 5v14" />
					</svg>
				</button>
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
