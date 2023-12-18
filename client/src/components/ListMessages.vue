<script setup lang="ts">
import type { Message, Chat } from '@/Models.vue'
import { ref, watch } from 'vue'
import hljs from 'highlight.js/lib/core'
import { decode } from 'he'
import InfoIcon from './Icons/InfoIcon.vue'

const props = defineProps({
	selectedChat: {
		type: Object as () => Chat,
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

	fetch(`http://localhost:3000/api/v1/chats/${props.selectedChat.id}/messages`, { method: 'GET', headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': 'http://localhost:3000' } })
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
		if (first == undefined) {
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
</script>

<template>
	<div class="overflow-y-auto [overflow-anchor:none]" id="scroller">
		<div v-if="isLoading">Loading...</div>

		<div
			v-for="message in messages"
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
						{{ message.created_at }}
                        <span>
                            ({{ message.prompt_tokens || message.completion_tokens || 0 }})
                        </span>
					</div>
				</div>
			</div>
		</div>

		<div id="anchor" class="[overflow-anchor: auto] h-0.5"></div>
	</div>
</template>
