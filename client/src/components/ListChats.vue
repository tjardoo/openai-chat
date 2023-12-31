<script setup lang="ts">
import { ref } from 'vue'
import { useChatStore } from '@/stores/ChatStore'
import StoreChatButton from '@/components/StoreChatButton.vue'
import ChrevronLeftIcon from '@/components/Icons/ChevronLeftIcon.vue'
import type { Chat } from '@/Models.vue'

const chatStore = useChatStore()

let chats = ref<Array<Chat>>([])

chatStore.$subscribe((mutation, state) => {
    fetch(`http://localhost:3000/api/v1/chats`, { method: 'GET', headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': 'http://localhost:3000' } })
        .then((response) => response.json())
        .then((data: Array<Chat>) => (chats.value = data))
        .catch((err) => console.log(err))
})

// upon first load, fetch chats
fetch(`http://localhost:3000/api/v1/chats`, { method: 'GET', headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': 'http://localhost:3000' } })
    .then((response) => response.json())
    .then((data: Array<Chat>) => (chats.value = data))
    .catch((err) => console.log(err))

defineEmits(['createChat', 'toggleSidebar'])
</script>

<template>
	<div class="relative h-screen py-12 overflow-y-auto">
		<button
			@click="$emit('toggleSidebar', false)"
			class="absolute top-0 right-0 px-4 py-4"
		>
			<ChrevronLeftIcon class="text-white" />
		</button>

		<StoreChatButton @create-chat="$emit('createChat')" />

		<div
			v-for="chat in chats"
			:key="chat.id"
		>
			<button
				@click="chatStore.setActiveChat(chat)"
				class="w-full px-8 py-2 text-left text-white hover:bg-gray-600"
				:class="{
					'bg-gray-600': chatStore.activeChat?.id === chat.id,
					'bg-gray-800': chatStore.activeChat?.id !== chat.id
				}"
			>
				{{ chat.title ?? 'Chat ' + chat.id }}
			</button>
		</div>
	</div>
</template>
