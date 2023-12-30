<script setup lang="ts">
import { ref } from 'vue'
import { useChatStore } from '@/stores/ChatStore'

const chatStore = useChatStore()

const chatName = ref<string | null | undefined>()

chatStore.$subscribe((mutation, state) => {
	chatName.value = state.activeChat?.title ?? 'Chat X'
})

defineEmits(['updateChatTitle'])
</script>

<template>
	<div class="flex space-x-4">
		<input
			type="text"
			v-model="chatName"
			placeholder="Chat name"
			class="w-full px-4 py-2 text-gray-700 border border-gray-300 rounded-lg"
			@change="$emit('updateChatTitle', chatName)"
		/>

		<button
			:disabled="chatName === ''"
			class="px-4 py-2 text-gray-700 bg-white border border-gray-300 rounded-lg disabled:cursor-not-allowed disabled:opacity-50"
		>
			Update
		</button>
	</div>
</template>
