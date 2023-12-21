<script setup lang="ts">
import type { Message, Chat } from '@/Models.vue'
import type { TextareaHTMLAttributes } from 'vue'
import { computed, ref, watch } from 'vue'
import PaperAirplaneIcon from './Icons/PaperAirplaneIcon.vue'

const props = defineProps({
	selectedChat: {
		type: Object as () => Chat,
		default: null,
		required: true
	},
	models: {
		type: Array as () => Array<String>,
		default: [],
		required: true
	}
})

const isLoading = ref<boolean>(false)
const isError = ref<boolean>(false)
const content = ref<TextareaHTMLAttributes['value']>('')
const model = ref<string | null>(null)
const maxTokens = ref<number | undefined>(undefined)
const temperature = ref<number>(1)

const sendMessage = () => {
	isLoading.value = true

	fetch(`http://localhost:3000/api/v1/chats/${props.selectedChat.id}/messages`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': 'http://localhost:3000' },
		body: JSON.stringify({
			content: content.value,
			model: model.value,
			max_tokens: maxTokens.value,
			temperature: temperature.value
		})
	})
		.then((response) => response.json())
		.then((data: Message) => {
			isLoading.value = false
			isError.value = false

			content.value = ''

			emit('messageSent')
		})
		.catch((err) => {
			isError.value = true
			isLoading.value = false

			console.log(err)
		})
}

watch(
	() => props.selectedChat,
	(first, second) => {
		if (first === undefined) {
			return
		}

		model.value = first.last_used_model
	},
	{ immediate: true }
)

const isSendButtonDisabled = computed(() => {
	return content.value === '' || isLoading.value === true || model.value === undefined
})

const emit = defineEmits(['messageSent'])
</script>

<template>
	<div class="mt-2 mb-8">
		<div class="flex space-x-4">
			<div class="w-3/4">
				<textarea v-model="content" rows="8" placeholder="Hello.." class="w-full h-full p-2 border border-gray-300 rounded-lg focus:outline-none" />
			</div>

			<div class="w-1/4 space-y-1">
				<button @click="sendMessage" :disabled="isSendButtonDisabled" class="flex items-center justify-end w-full h-16 px-4 py-2 text-gray-700 bg-white border border-gray-300 rounded-lg disabled:cursor-not-allowed disabled:opacity-50">
					<template v-if="isLoading">Loading..</template>
					<template v-else> Send <PaperAirplaneIcon class="ml-3" /> </template>
				</button>

				<div>
					<label for="model" class="text-gray-500 text-[10px]">OpenAI model</label>
					<select class="w-full px-4 py-2 text-gray-700 border border-gray-300 rounded-lg" id="model" v-model="model">
						<template v-for="model in models" :key="model">
							<option :value="model">{{ model }}</option>
						</template>
					</select>
				</div>

				<div class="flex space-x-2">
					<div class="w-1/2">
						<label for="max_tokens" class="text-gray-500 text-[10px]">Max. tokens</label>
						<input type="number" id="max_tokens" class="w-full px-4 py-2 text-gray-700 border border-gray-300 rounded-lg" v-model="maxTokens" />
					</div>

					<div class="w-1/2">
						<label for="temperature" class="text-gray-500 text-[10px]">Temperature</label>
						<input type="number" id="temperature" class="w-full px-4 py-2 text-gray-700 border border-gray-300 rounded-lg" v-model="temperature" min="0" max="2" step="0.1" />
					</div>
				</div>
			</div>
		</div>
		<div v-if="isError">
			<div class="py-2 text-sm text-red-500">Something went wrong. Please try again.</div>
		</div>
	</div>
</template>
