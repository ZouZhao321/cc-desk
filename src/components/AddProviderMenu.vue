<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

const emit = defineEmits<{
	add: []
	paste: []
	readCurrent: []
}>()

const open = ref(false)
const menuRef = ref<HTMLDivElement | null>(null)

function toggle() {
	open.value = !open.value
}

function handleAction(action: string) {
	open.value = false
	if (action === 'add') emit('add')
	else if (action === 'paste') emit('paste')
	else if (action === 'readCurrent') emit('readCurrent')
}

function handleClickOutside(e: globalThis.MouseEvent) {
	if (menuRef.value && !menuRef.value.contains(e.target as Node)) {
		open.value = false
	}
}

onMounted(() => document.addEventListener('click', handleClickOutside))
onUnmounted(() => document.removeEventListener('click', handleClickOutside))
</script>

<template>
	<div ref="menuRef" class="relative">
		<button
			class="flex items-center justify-center w-36px h-36px border-none bg-[#FF6B35] rounded-full cursor-pointer hover:opacity-90 transition-opacity"
			@click.stop="toggle"
		>
			<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#FFFFFF" stroke-width="2">
				<path d="M5 12h14" />
				<path d="M12 5v14" />
			</svg>
		</button>

		<div
			v-if="open"
			class="absolute right-0 top-full mt-8px w-160px bg-white rounded-8px shadow-lg border border-gray-100 py-4px z-50"
		>
			<button
				class="flex items-center gap-8px w-full px-12px py-8px border-none bg-transparent text-13px text-gray-700 cursor-pointer hover:bg-gray-50 transition-colors text-left"
				@click="handleAction('add')"
			>
				<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#6B7280" stroke-width="2">
					<path d="M12 5v14M5 12h14" />
				</svg>
				新建配置
			</button>
			<button
				class="flex items-center gap-8px w-full px-12px py-8px border-none bg-transparent text-13px text-gray-700 cursor-pointer hover:bg-gray-50 transition-colors text-left"
				@click="handleAction('paste')"
			>
				<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#6B7280" stroke-width="2">
					<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" />
					<rect x="8" y="2" width="8" height="4" rx="1" ry="1" />
				</svg>
				粘贴配置
			</button>
			<button
				class="flex items-center gap-8px w-full px-12px py-8px border-none bg-transparent text-13px text-gray-700 cursor-pointer hover:bg-gray-50 transition-colors text-left"
				@click="handleAction('readCurrent')"
			>
				<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#6B7280" stroke-width="2">
					<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
					<polyline points="7 10 12 15 17 10" />
					<line x1="12" y1="15" x2="12" y2="3" />
				</svg>
				读取配置
			</button>
		</div>
	</div>
</template>
