<script setup lang="ts">
import { ref } from 'vue'
import ListChats from './components/ListChats.vue'
import ListMessages from './components/ListMessages.vue'
import StoreMessage from './components/StoreMessage.vue'
import ShowChatTitle from './components/ShowChatTitle.vue'
import type { Chat } from './Models.vue'

let selectedChat = ref<Chat | undefined>(undefined)
let models = ref<Array<String> | undefined>(undefined)
let isFetchChats = ref<boolean>(false)
let isFetchMessages = ref<boolean>(false)

const setSelectedChat = (chat: Chat) => {
	selectedChat.value = chat
}

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
		.then((data: Chat) => {
			selectedChat.value = data

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

const fetchMessages = () => {
	isFetchMessages.value = true

	setTimeout(() => {
		isFetchMessages.value = false
	}, 1000)
}

const updateChatTitle = (title: string) => {
	if (selectedChat.value === undefined) {
		return
	}

	fetch(`http://localhost:3000/api/v1/chats/${selectedChat.value.id}`, {
		method: 'PATCH',
		headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': 'http://localhost:3000' },
		body: JSON.stringify({ title: title })
	})
		.then((response) => response.json())
		.then((data: Chat) => {
			selectedChat.value = data

			fetchChats()
		})
		.catch((err) => console.log(err))
}
</script>

<template>
	<div class="flex bg-gray-50">
		<div class="h-screen bg-gray-800 w-96">
			<ListChats :is-fetching="isFetchChats" @selected-chat-changed="setSelectedChat" @create-chat="createChat" />
		</div>
		<main class="w-full max-w-4xl mx-auto">
			<div v-if="selectedChat !== undefined" class="flex flex-col h-screen">
				<div class="my-6">
					<ShowChatTitle :selected-chat="selectedChat" @update-chat-title="updateChatTitle" />
				</div>

				<div class="h-full overflow-y-auto">
					<ListMessages :selected-chat="selectedChat" :is-fetching="isFetchMessages" />
				</div>

				<div class="h-auto">
					<StoreMessage :selected-chat="selectedChat" :models="models" @message-sent="fetchMessages" />
				</div>
			</div>
		</main>
	</div>
</template>
