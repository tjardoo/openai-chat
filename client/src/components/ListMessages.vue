<script setup lang="ts">
import { ref } from 'vue'
import hljs from 'highlight.js/lib/core'
import { decode } from 'he'
import { useChatStore } from '@/stores/ChatStore'
import { useMessagesStore } from '@/stores/MessagesStore'
import type { Message } from '@/Models.vue'

const chatStore = useChatStore()
const messagesStore = useMessagesStore()

const props = defineProps({
	isFetching: {
		type: Boolean,
		default: false,
		required: false
	},
	receivedChunks: {
		type: String,
		default: '',
		required: false
	}
})

const isLoading = ref<boolean>(false)

chatStore.$subscribe((mutation, state) => {
	isLoading.value = true

	if (chatStore.activeChat === null) {
		return
	}

	fetch(`http://localhost:3000/api/v1/chats/${chatStore.activeChat.id}/messages`, { method: 'GET', headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': 'http://localhost:3000' } })
		.then((response) => response.json())
		.then((messages: Array<Message>) => {
			messagesStore.setMessages(messages.reverse())
			isLoading.value = false
		})
		.catch((err) => console.log(err))
})

const hightlightCodeExamples = (message: string): string => {
	const text = message.replace(/```([\s\S]*?)```/g, '<pre><code>$1</code></pre>')

	const regex = /<pre><code(?: class="language-(\w+)")?>([\s\S]*?)<\/code><\/pre>/g

	return text.replace(regex, (match, lang, code) => {
		code = decode(code)

		if (lang && hljs.getLanguage(lang)) {
			return `<pre class="code-wrapper"><code class="language-${lang}">${hljs.highlight(code, { language: lang, ignoreIllegals: true }).value}</code></pre>`
		} else {
			const autoDetected = hljs.highlightAuto(code)

			autoDetected.value = autoDetected.value.replace(/^.*\n/, '')

			return `<pre class="code-wrapper"><code class="language-${autoDetected.language}">${autoDetected.value}</code></pre>`
		}
	})
}

const formatDateTime = (dateTime: string): string => {
	const date = new Date(dateTime)

	return `${date.toLocaleDateString()} ${date.toLocaleTimeString()}`
}
</script>

<template>
	<div
		class="px-2 overflow-y-auto min-h-[480px]"
		id="messages"
	>
		<div v-if="isLoading">Loading...</div>

		<div
			v-for="message in messagesStore.messages"
			:key="message.id"
			class="flex mt-1 mb-3"
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
					v-html="hightlightCodeExamples(message.content)"
				></div>
				<div class="text-right">
					<div class="text-[10px] text-gray-400 mt-1">
						<span>{{ formatDateTime(message.created_at) }}</span>
						&bull;
						<template v-if="message.temperature">
							&bull;
							<span>{{ message.temperature }}</span>
						</template>
					</div>
				</div>
			</div>
		</div>

		<div
			class="flex justify-start mt-1 mb-3"
			v-if="receivedChunks"
		>
			<div class="flex flex-col">
				<div
					class="flex-none px-3 py-1 font-light text-left text-gray-700 bg-gray-300 rounded-lg"
					v-html="hightlightCodeExamples(receivedChunks)"
				></div>
			</div>
		</div>
	</div>
</template>

<style scoped>
#messages {
	display: flex;
	flex-direction: column-reverse;
	height: 100%;
	overflow-y: scroll;
}
</style>
