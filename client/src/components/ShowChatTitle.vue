<script setup lang="ts">
import { useChatStore } from '@/stores/ChatStore'
import { storeToRefs } from 'pinia'

const chatStore = useChatStore()

const chatTitle = storeToRefs(chatStore).title

defineEmits(['updateChatTitle'])
</script>

<template>
	<div class="flex space-x-4">
		<input
			type="text"
			v-model="chatTitle"
			placeholder="Chat name"
			class="w-full px-4 py-2 text-gray-700 border border-gray-300 rounded-lg"
			@change="$emit('updateChatTitle', chatTitle)"
		/>

		<button
			:disabled="chatTitle !== null"
			class="px-4 py-2 text-gray-700 bg-white border border-gray-300 rounded-lg disabled:cursor-not-allowed disabled:opacity-50"
		>
			Update
		</button>
	</div>
</template>
