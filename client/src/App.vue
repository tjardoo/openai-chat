<script setup lang="ts">
import { ref } from 'vue'
import { useChatStore } from '@/stores/ChatStore'
import ListChats from '@/components/ListChats.vue'
import ListMessages from '@/components/ListMessages.vue'
import StoreMessage from '@/components/StoreMessage.vue'
import ShowChatTitle from '@/components/ShowChatTitle.vue'
import ChevronRightIcon from '@/components/Icons/ChevronRightIcon.vue'
import type { Chat } from '@/Models.vue'

const chatStore = useChatStore()

const models = ref<Array<String> | undefined>(undefined)
const isFetchChats = ref<boolean>(false)
const isSidebarOpen = ref<boolean>(true)
const receivedChunks = ref<string>('')

fetch(`http://localhost:3000/api/v1/models`, {
	method: 'GET',
	headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': 'http://localhost:3000' }
})
	.then((response) => response.json())
	.then((data: Array<String>) => {
		models.value = data
	})
	.catch((err) => console.log(err))

const createChat = () => {
	fetch(`http://localhost:3000/api/v1/chats`, { method: 'POST', headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': 'http://localhost:3000' } })
		.then((response) => response.json())
		.then((chat: Chat) => {
			chatStore.setActiveChat(chat)

			fetchChats()
		})
		.catch((err) => console.log(err))
}

const fetchChats = () => {
	isFetchChats.value = true

	setTimeout(() => {
		isFetchChats.value = false
	}, 1000)
}

const updateReceivedChunks = (value: string) => {
	receivedChunks.value = value
}

const updateChatTitle = (title: string) => {
	if (chatStore.activeChat === null) {
		return
	}

	fetch(`http://localhost:3000/api/v1/chats/${chatStore.activeChat.id}`, {
		method: 'PATCH',
		headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': 'http://localhost:3000' },
		body: JSON.stringify({ title: title })
	})
		.then((response) => response.json())
		.then((chat: Chat) => {
			chatStore.activeChat = chat

			fetchChats()
		})
		.catch((err) => console.log(err))
}

const toggleSidebar = (shouldOpen: boolean) => {
	isSidebarOpen.value = shouldOpen
}
</script>

<template>
	<div class="flex bg-gray-50">
		<div
			class="h-screen bg-gray-800"
			:class="{
				'w-16': !isSidebarOpen,
				'w-96': isSidebarOpen
			}"
		>
			<ListChats
				:is-fetching="isFetchChats"
				:is-sidebar-open="isSidebarOpen"
				@create-chat="createChat"
				@toggle-sidebar="toggleSidebar"
				v-if="isSidebarOpen"
			/>
			<div
				class="relative h-screen py-12 overflow-y-auto"
				v-if="isSidebarOpen === false"
			>
				<button
					@click="toggleSidebar(true)"
					class="absolute top-0 right-0 px-4 py-4"
				>
					<ChevronRightIcon class="text-white" />
				</button>
			</div>
		</div>
		<main class="w-full max-w-4xl px-4 mx-auto xl:px-0">
			<div
				v-if="chatStore.activeChat !== null"
				class="flex flex-col h-screen"
			>
				<div class="my-6">
					<ShowChatTitle @update-chat-title="updateChatTitle" />
				</div>

				<div class="h-full overflow-y-auto">
					<ListMessages :receivedChunks="receivedChunks" />
				</div>

				<div class="h-auto">
					<StoreMessage
						:models="models"
						@update-received-chunks="updateReceivedChunks"
					/>
				</div>
			</div>
		</main>
	</div>
</template>
