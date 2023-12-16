<script setup lang="ts">
import type { Message, Chat } from '@/Models.vue'
import type { TextareaHTMLAttributes } from 'vue'
import { ref } from 'vue'

const props = defineProps({
	selectedChat: {
		type: Object as () => Chat,
		default: null,
		required: true
	}
})

const isLoading = ref<boolean>(false)
const content = ref<TextareaHTMLAttributes['value']>('')
const model = ref<string>('gpt-4-0613')
const maxTokens = ref<number|undefined>(undefined)

const sendMessage = () => {
	isLoading.value = true

	fetch(`http://localhost:3000/api/v1/chats/${props.selectedChat.id}/messages`, { method: 'POST', headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': 'http://localhost:3000' }, body: JSON.stringify({
            content: content.value,
            model: model.value,
            max_tokens: maxTokens.value,
        })
    })
		.then((response) => response.json())
		.then((data: Message) => {
			isLoading.value = false

			content.value = ''

			emit('messageSent')
		})
		.catch((err) => console.log(err))
}

const emit = defineEmits(['messageSent'])
</script>

<template>
	<div class="flex mt-2 mb-8 space-x-4">
		<textarea
            v-model="content"
            rows="8"
            placeholder="Hello.."
            class="w-full h-full p-2 border border-gray-300 rounded-lg focus:outline-none"
        />

        <div class="space-y-1">
            <button
                @click="sendMessage"
                :disabled="isLoading || content === ''"
                class="w-64 h-16 px-4 py-2 text-gray-700 border border-gray-300 rounded-lg disabled:cursor-not-allowed disabled:opacity-50"
            >
                <template v-if="isLoading"> Loading.. </template>
                <template v-else> Send </template>
            </button>

            <div>
                <label for="model" class="text-gray-500 text-[10px]">OpenAI model</label>
                <select
                    class="w-64 px-4 py-2 text-gray-700 border border-gray-300 rounded-lg"
                    id="model"
                    v-model="model"
                >
                    <option value="gpt-4-32k-0613">gpt-4-32k-0613</option>
                    <option value="gpt-4-0613">gpt-4-0613</option>
                    <option value="gpt-3.5-turbo-1106">gpt-3.5-turbo-1106</option>
                </select>
            </div>


            <div>
                <label for="max_tokens" class="text-gray-500 text-[10px]">Max. tokens</label>
                <input
                    type="number"
                    id="max_tokens"
                    class="w-64 px-4 py-2 text-gray-700 border border-gray-300 rounded-lg"
                    v-model="maxTokens"
                />
            </div>
        </div>
	</div>
</template>
