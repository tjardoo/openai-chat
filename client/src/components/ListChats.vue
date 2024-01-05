<script setup lang="ts">
import { ref } from 'vue'
import { useChatStore } from '@/stores/ChatStore'
import StoreChatButton from '@/components/StoreChatButton.vue'
import ChrevronLeftIcon from '@/components/Icons/ChevronLeftIcon.vue'
import type { Chat } from '@/Models.vue'
import DotsIcon from './Icons/DotsIcon.vue'

const chatStore = useChatStore()

let chats = ref<Array<Chat>>([])
let dropdownMenuId = ref<number | null>(null)

const deleteChat = (chatId: number) => {
	if (chatStore.activeChat?.id === chatId) {
		chatStore.activeChat = null
	}

	chats.value = chats.value.filter((chat) => chat.id !== chatId)

	fetch(`http://localhost:3000/api/v1/chats/${chatId}`, { method: 'DELETE', headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': 'http://localhost:3000' } })
		.then((response) => response)
		.catch((err) => console.log(err))
}

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
			class="relative flex justify-between group"
		>
			<button
				@click="chatStore.setActiveChat(chat)"
				class="w-full px-8 py-2 text-left text-white group-hover:bg-gray-600"
				:class="{
					'bg-gray-600': chatStore.activeChat?.id === chat.id,
					'bg-gray-800': chatStore.activeChat?.id !== chat.id
				}"
			>
				{{ chat.title ?? 'Chat ' + chat.id }}
			</button>

			<button
				@click="dropdownMenuId = chat.id"
				class="relative px-2 group-hover:bg-gray-600"
				:class="{
					'bg-gray-600': chatStore.activeChat?.id === chat.id,
					'bg-gray-800': chatStore.activeChat?.id !== chat.id
				}"
			>
				<DotsIcon class="text-gray-400" />
			</button>

			<div
				class="absolute top-0 right-0 z-50 px-2 py-2 bg-gray-50"
				v-if="dropdownMenuId == chat.id"
			>
				<button
					@click="deleteChat(chat.id)"
					class="w-full px-2 py-1 text-left"
				>
					Delete
				</button>
			</div>
		</div>
	</div>
</template>
