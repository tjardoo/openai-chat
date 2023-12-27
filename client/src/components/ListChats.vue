<script setup lang="ts">
import type { Chat } from '@/Models.vue'
import { ref, watch } from 'vue'
import StoreChatButton from './StoreChatButton.vue'
import ChrevronLeftIcon from './Icons/ChevronLeftIcon.vue'

const props = defineProps({
	isFetching: {
		type: Boolean,
		default: false,
		required: false
	},
	selectedChat: {
		type: Object as () => Chat | undefined,
		required: true
	},
	isSidebarOpen: {
		type: Boolean,
		required: true
	}
})

let chats = ref<Array<Chat>>([])

watch(
	() => props.isFetching,
	(first, second) => {
		if (first !== second) {
			fetch(`http://localhost:3000/api/v1/chats`, { method: 'GET', headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': 'http://localhost:3000' } })
				.then((response) => response.json())
				.then((data: Array<Chat>) => (chats.value = data))
				.catch((err) => console.log(err))
		}
	},
	{ immediate: true }
)

defineEmits(['selectedChatChanged', 'createChat', 'toggleSidebar'])
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
				@click="$emit('selectedChatChanged', chat)"
				class="w-full px-8 py-2 text-left text-white hover:bg-gray-600"
				:class="{
					'bg-gray-600': selectedChat?.id === chat.id,
					'bg-gray-800': selectedChat?.id !== chat.id
				}"
			>
				{{ chat.title ?? 'Chat ' + chat.id }}
			</button>
		</div>
	</div>
</template>
