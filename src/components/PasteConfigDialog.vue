<script setup lang="ts">
import { ref } from 'vue'
import { NInput, NAlert } from 'naive-ui'
import { parseConfigText } from '../utils/parseConfig'
import type { ParsedConfig } from '../utils/parseConfig'

const emit = defineEmits<{
	confirm: [config: ParsedConfig]
	cancel: []
}>()

const text = ref('')
const error = ref('')

function handleParse() {
	error.value = ''
	if (!text.value.trim()) {
		error.value = '请粘贴配置文本'
		return
	}

	try {
		const config = parseConfigText(text.value)
		if (!config.api_key) {
			error.value = '未找到 API Key（ANTHROPIC_AUTH_TOKEN）'
			return
		}
		if (!config.base_url) {
			error.value = '未找到请求地址（ANTHROPIC_BASE_URL）'
			return
		}
		emit('confirm', config)
	} catch (e) {
		error.value = `解析失败: ${String(e)}`
	}
}
</script>

<template>
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50" @click.self="emit('cancel')">
		<div class="w-560px bg-white rounded-12px shadow-lg overflow-hidden" @click.stop>
			<!-- 头部 -->
			<div class="flex items-center justify-between px-24px py-16px border-b border-gray-100">
				<span class="text-16px font-600 text-gray-900">粘贴配置</span>
				<button
					class="flex items-center justify-center w-28px h-28px border-none bg-transparent rounded-6px cursor-pointer hover:bg-gray-100 transition-colors"
					@click="emit('cancel')"
				>
					<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#6B7280" stroke-width="2">
						<path d="M18 6L6 18M6 6l12 12" />
					</svg>
				</button>
			</div>

			<!-- 内容 -->
			<div class="px-24px py-16px">
				<p class="text-13px text-gray-500 m-0 mb-12px">
					粘贴 Linux/Mac 的 export 命令或 Windows PowerShell 的 $env: 命令：
				</p>
				<n-input
					v-model:value="text"
					type="textarea"
					:rows="10"
					placeholder="export ANTHROPIC_BASE_URL=https://api.example.com/anthropic&#10;export ANTHROPIC_AUTH_TOKEN=sk-xxx&#10;export ANTHROPIC_MODEL=model-id&#10;..."
					class="font-mono text-12px"
				/>
				<n-alert v-if="error" type="error" class="mt-12px">
					{{ error }}
				</n-alert>
			</div>

			<!-- 底部 -->
			<div class="flex items-center justify-end gap-12px px-24px py-16px border-t border-gray-100">
				<button
					class="px-16px py-8px rounded-8px border border-gray-200 bg-white text-14px text-gray-700 cursor-pointer hover:bg-gray-50 transition-colors"
					@click="emit('cancel')"
				>
					取消
				</button>
				<button
					class="px-20px py-8px rounded-8px border-none bg-[#FF6B35] text-14px font-600 text-white cursor-pointer hover:opacity-90 transition-opacity"
					@click="handleParse"
				>
					解析并填充
				</button>
			</div>
		</div>
	</div>
</template>
