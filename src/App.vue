<script setup lang="ts">
import { NConfigProvider, NMessageProvider } from 'naive-ui'
import AppContent from './components/AppContent.vue'
</script>

<template>
	<n-config-provider>
		<n-message-provider>
			<AppContent />
		</n-message-provider>
	</n-config-provider>
</template>
