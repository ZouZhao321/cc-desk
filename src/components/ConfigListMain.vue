<script setup lang="ts">
interface ConfigScope {
	id: string
	name: string
	icon: string
	iconColor: string
	iconBg: string
	description: string
	stats: { label: string; value: number }[]
	badge?: string
	badgeColor?: string
	isActive?: boolean
}

defineProps<{
	scopes: ConfigScope[]
}>()

defineEmits<{
	select: [id: string]
	add: []
	settings: []
	sync: []
}>()
</script>

<template>
	<div class="flex flex-col w-full h-full bg-white font-sans">
		<header class="flex items-center justify-between h-56px px-24px bg-white border-b border-gray-100 shrink-0">
			<div class="flex items-center gap-16px">
				<span class="text-20px font-bold text-[#FF6B35]">CC-Desk</span>
				<button
					class="flex items-center justify-center w-32px h-32px border-none bg-gray-100 rounded-6px cursor-pointer hover:bg-gray-200 transition-colors"
					@click="$emit('settings')"
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
						stroke="#10B981"
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
					class="flex items-center justify-center w-36px h-36px border-none bg-[#FF6B35] rounded-full cursor-pointer hover:opacity-90 transition-opacity"
					@click="$emit('add')"
				>
					<svg
						width="18"
						height="18"
						viewBox="0 0 24 24"
						fill="none"
						stroke="#FFFFFF"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<path d="M5 12h14" />
						<path d="M12 5v14" />
					</svg>
				</button>
			</div>
		</header>

		<main class="flex-1 py-16px px-24px bg-[#F8F9FA] overflow-y-auto">
			<div class="flex flex-col gap-12px">
				<div
					v-for="scope in scopes"
					:key="scope.id"
					class="flex items-center gap-16px p-16px pr-20px bg-white rounded-12px border border-gray-200 cursor-pointer transition-all shadow-[0_2px_8px_rgba(0,0,0,0.03)] hover:border-gray-300 hover:shadow-[0_4px_12px_rgba(0,0,0,0.06)]"
					:class="{ '!border-[#6366F1] !border-2': scope.isActive }"
					@click="$emit('select', scope.id)"
				>
					<div
						class="flex items-center justify-center w-44px h-44px rounded-10px text-22px shrink-0"
						:style="{ backgroundColor: scope.iconBg }"
					>
						<span :style="{ color: scope.iconColor }">{{ scope.icon }}</span>
					</div>
					<div class="flex-1 flex flex-col gap-4px min-w-0">
						<div class="flex items-center gap-8px">
							<span class="text-15px font-600 text-gray-900">{{ scope.name }}</span>
							<span
								v-if="scope.badge"
								class="text-10px font-500 text-white px-8px py-2px rounded-4px"
								:style="{ backgroundColor: scope.badgeColor || '#6366F1' }"
							>
								{{ scope.badge }}
							</span>
						</div>
						<p class="text-12px text-gray-500 m-0">{{ scope.description }}</p>
						<div class="flex items-center gap-8px flex-wrap">
							<span
								v-for="stat in scope.stats"
								:key="stat.label"
								class="text-10px text-gray-500 bg-gray-100 px-6px py-2px rounded-4px"
								:class="{
									'!text-blue-500 !bg-blue-50': stat.label === 'project' || stat.label === 'projects'
								}"
							>
								{{ stat.value }} {{ stat.label }}
							</span>
						</div>
					</div>
					<div class="shrink-0 flex items-center">
						<svg
							width="16"
							height="16"
							viewBox="0 0 24 24"
							fill="none"
							stroke="#D1D5DB"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<path d="m9 18 6-6-6-6" />
						</svg>
					</div>
				</div>
			</div>
		</main>
	</div>
</template>
