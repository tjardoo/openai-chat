<script setup lang="ts">
import { ref, watch } from 'vue'
import hljs from 'highlight.js/lib/core'
import { decode } from 'he'
import { useChatStore } from '@/stores/ChatStore'
import { useMessagesStore } from '@/stores/MessagesStore'
import type { Message } from '@/Models.vue'
import { storeToRefs } from 'pinia'

const chatStore = useChatStore()
const chatId = storeToRefs(chatStore).id

const messagesStore = useMessagesStore()

const isLoading = ref<boolean>(false)

watch(
	chatId,
	() => {
		if (chatId.value === null) {
			return
		}

		fetch(`http://localhost:3000/api/v1/chats/${chatId.value}/messages`, { method: 'GET', headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': 'http://localhost:3000' } })
			.then((response) => response.json())
			.then((messages: Array<Message>) => {
				const reversedMessages = messages.reverse()

				messagesStore.setMessages(reversedMessages)
				isLoading.value = false
			})
			.catch((err) => console.log(err))
	},
	{ immediate: true }
)

const hightlightCodeExamples = (message: string): string => {
	const text = message.replace(/```([\s\S]*?)```/g, '<pre><code>$1</code></pre>')

	const regex = /(<pre><code(?: class="language-(\w+)")?>([\s\S]*?)<\/code><\/pre>)|(`(.*?)`)|(<bold>(.*?)<\/bold>)/g

	return text.replace(regex, (match, codeBlock, lang, code, backticks, content) => {
		if (codeBlock) {
			code = decode(code)

			if (lang && hljs.getLanguage(lang)) {
				return `<pre class="code-wrapper"><code class="language-${lang}">${hljs.highlight(code, { language: lang, ignoreIllegals: true }).value}</code></pre>`
			} else {
				const autoDetected = hljs.highlightAuto(code)
				autoDetected.value = autoDetected.value.replace(/^.*\n/, '')
				return `<pre class="code-wrapper"><code class="language-${autoDetected.language}">${autoDetected.value}</code></pre>`
			}
		} else if (backticks) {
			return `<span class="font-medium">\`${content}\`</span>`
		} else if (content) {
			return `<span class="font-medium">${content}</span>`
		}

		return match
	})
}

const formatDateTime = (dateTime: string): string => {
	const date = new Date(dateTime)

	return `${date.toLocaleDateString()} ${date.toLocaleTimeString()}`
}
</script>

<template>
	<div
		class="px-2 mt-4 overflow-y-auto min-h-[480px]"
		id="messages"
	>
		<div
			class="flex justify-start mt-1 mb-3"
			v-if="messagesStore.streamingMessage"
		>
			<div class="flex flex-col">
				<div
					class="flex-none px-3 py-1 font-light text-left text-gray-700 whitespace-pre-wrap bg-gray-300 rounded-lg"
					v-html="hightlightCodeExamples(messagesStore.streamingMessage)"
				></div>
			</div>
		</div>

		<div
			v-for="message in messagesStore.messages"
			:key="message.id"
			class="flex mt-1 mb-3"
			:class="{
				'justify-start': message.role === 'assistant',
				'justify-end': message.role === 'user'
			}"
		>
			<div class="flex flex-col overflow-x-hidden">
				<div
					class="flex-none w-full px-3 py-1 font-light break-words whitespace-pre-wrap rounded-lg"
					:class="{
						'text-left bg-gray-300 text-gray-700': message.role === 'assistant',
						'bg-blue-600 text-white': message.role === 'user'
					}"
					v-html="hightlightCodeExamples(message.content)"
				></div>
				<div class="text-right">
					<div class="text-[10px] text-gray-400 mt-1">
						<span>{{ formatDateTime(message.created_at) }}</span>
					</div>
				</div>
			</div>
		</div>

		<div v-if="isLoading">Loading...</div>
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
