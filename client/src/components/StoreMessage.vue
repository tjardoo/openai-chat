<script setup lang="ts">
import { computed, ref } from 'vue'
import PaperAirplaneIcon from '@/components/Icons/PaperAirplaneIcon.vue'
import ErrorMessage from '@/components/Forms/ErrorMessage.vue'
import { useChatStore } from '@/stores/ChatStore'
import { useMessagesStore } from '@/stores/MessagesStore'
import type { Chat, FieldValidatorError, Message } from '@/Models.vue'
import type { TextareaHTMLAttributes } from 'vue'

const chatStore = useChatStore()
const messagesStore = useMessagesStore()

const props = defineProps({
	models: {
		type: Array as () => Array<String>,
		default: [],
		required: true
	}
})

const isLoading = ref<boolean>(false)
const isError = ref<boolean>(false)
const validationErrors = ref<Array<FieldValidatorError> | null>(null)
const content = ref<TextareaHTMLAttributes['value']>('')
const model = ref<string>('gpt-4-1106-preview')

const textDecoder = new TextDecoder('utf-8')

const clearErrors = (): void => {
	isError.value = false
	isLoading.value = false
	validationErrors.value = null
}

const streamCompleted = (): void => {
	isLoading.value = false

	if (chatStore.activeChat === null) {
		return
	}

	if (messagesStore.messages.length === 1) {
		fetch(`http://localhost:3000/api/v1/chats/${chatStore.activeChat.id}`, { method: 'GET', headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': 'http://localhost:3000' } })
			.then((response) => response.json())
			.then((chat: Chat) => chatStore.updateActiveChatTitle(chat.title ?? 'Untitled'))
			.catch((err) => console.log(err))
	}

	fetch(`http://localhost:3000/api/v1/chats/${chatStore.activeChat.id}/messages/assistant`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': 'http://localhost:3000' },
		body: JSON.stringify({
			content: messagesStore.streamingMessage
		})
	})
		.then((response) => response.json())
		.then((message: Message) => {
			messagesStore.addMessage(message)

			messagesStore.clearStreamingMessage()
		})
		.catch((err) => console.log(err))
}

const sendMessage = (): void => {
	if (chatStore.activeChat === null) {
		return
	}

	isLoading.value = true

	fetch(`http://localhost:3000/api/v1/chats/${chatStore.activeChat.id}/messages`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': 'http://localhost:3000' },
		body: JSON.stringify({
			content: content.value,
			model: model.value
		})
	}).then((response) => {
		if (content.value) {
			messagesStore.addMessage({
				id: 0,
				role: 'user',
				content: content.value.toString(),
				created_at: new Date().toISOString()
			})
		}

		content.value = ''

		if (response.body === null) {
			return
		}

		let receivedChunks = ''

		const reader = response.body.getReader()

		reader.read().then(function processText({ done, value }): any {
			if (done === true) {
				streamCompleted()

				return
			}

			receivedChunks += textDecoder.decode(value)

			console.log('Incoming: ' + receivedChunks)

			messagesStore.streamMessage(receivedChunks)

			return reader.read().then(processText)
		})
	})
}

chatStore.$subscribe((mutation, state) => {
	clearErrors()

	content.value = ''
})

const isSendButtonDisabled = computed((): boolean => {
	return content.value === '' || isLoading.value === true || model.value === null
})

const isFieldInvalid = (field: string): boolean => {
	if (validationErrors.value === null) {
		return false
	}

	return validationErrors.value.some((fieldValidatorError: FieldValidatorError) => fieldValidatorError.name === field)
}

const getValidationError = (field: string): FieldValidatorError | null => {
	if (validationErrors.value === undefined) {
		return null
	}

	const fieldValidatorError = validationErrors.value?.find((fieldValidatorError: FieldValidatorError) => fieldValidatorError.name === field)

	if (fieldValidatorError === undefined) {
		return null
	}

	return fieldValidatorError
}
</script>

<template>
	<div class="mt-2 mb-8">
		<div class="flex flex-wrap">
			<div class="w-full xl:w-3/4">
				<textarea
					v-model="content"
					rows="8"
					placeholder="Hello.."
					class="w-full h-full p-2 border border-gray-300 rounded-lg focus:outline-none"
					:class="{
						'border-red-500': isFieldInvalid('content')
					}"
				/>
				<ErrorMessage :error="getValidationError('content')" />
			</div>

			<div class="flex flex-row-reverse w-full space-y-1 xl:flex-row xl:block xl:w-1/4 xl:pl-4 xl:pt-0">
				<button
					@click="sendMessage"
					:disabled="isSendButtonDisabled"
					class="flex items-center justify-end w-1/3 px-4 py-2 text-gray-700 bg-white border border-gray-300 rounded-lg mt-7 xl:mt-0 xl:w-full disabled:cursor-not-allowed disabled:opacity-50"
				>
					<template v-if="isLoading">Loading..</template>
					<template v-else>
						Send
						<PaperAirplaneIcon class="ml-3 text-xl" />
					</template>
				</button>

				<div class="w-full mr-2 xl:mr-0">
					<label
						for="model"
						class="text-gray-500 text-[10px]"
					>
						OpenAI model
					</label>
					<select
						id="model"
						v-model="model"
						class="w-full px-4 py-2 text-gray-700 border border-gray-300 rounded-lg"
						:class="{
							'border-red-500': isFieldInvalid('model')
						}"
					>
						<template
							v-for="model in props.models"
							:key="model"
						>
							<option :value="model">
								{{ model }}
							</option>
						</template>
					</select>
					<ErrorMessage :error="getValidationError('model')" />
				</div>
			</div>
		</div>
		<div v-if="isError">
			<div class="py-2 mt-4 text-sm text-red-500">Something went wrong. Please try again.</div>
		</div>
	</div>
</template>
