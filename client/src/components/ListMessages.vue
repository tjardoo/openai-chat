<script setup lang="ts">
import type { Message } from '@/Models.vue'
import { ref, watch } from 'vue'

const props = defineProps({
	selectedChat: {
		type: Number,
		default: null,
		required: true
	},
	isFetching: {
		type: Boolean,
		default: false,
		required: false
	}
})

const isLoading = ref<boolean>(false)
const messages = ref<Array<Message>>([])

const updateMessages = () => {
	isLoading.value = true

	fetch(`http://localhost:3000/api/v1/chats/${props.selectedChat}/messages`, { method: 'GET', headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': 'http://localhost:3000' } })
		.then((response) => response.json())
		.then((data: Array<Message>) => {
			messages.value = data
			isLoading.value = false
		})
		.catch((err) => console.log(err))
}

watch(
	() => props.isFetching,
	(first, second) => {
		if (props.selectedChat == undefined) {
			return
		}

		if (first !== second && first === true) {
			updateMessages()
		}
	},
	{ immediate: false }
)

watch(
	() => props.selectedChat,
	(first, second) => {
		if (first === undefined) {
			return
		}

		if (first !== second) {
			updateMessages()
		}
	},
	{ immediate: true }
)
</script>

<template>
	<div class="overflow-y-auto [overflow-anchor:none]" id="scroller">
		<div v-if="isLoading">Loading...</div>

		<div
			v-for="message in messages"
			:key="message.created_at"
			class="flex my-1"
			:class="{
				'justify-start': message.role === 'assistant',
				'justify-end': message.role === 'user'
			}"
		>
			<div class="flex flex-col">
				<div
					class="flex-none px-3 py-1 font-light rounded-lg"
					:class="{
						'text-left bg-gray-300 text-gray-700': message.role === 'assistant',
						'text-right bg-blue-600 text-white': message.role === 'user'
					}"
				>
					{{ message.content }}
				</div>
				<div class="text-right">
					<span class="text-[10px] text-gray-400">{{ message.created_at }}</span>
				</div>
			</div>
		</div>

		<div id="anchor" class="[overflow-anchor: auto] h-0.5"></div>
	</div>
</template>
