<script setup lang="ts">
import type { Message } from '@/Models.vue';
import type { TextareaHTMLAttributes } from 'vue';
import { ref } from 'vue'

const props = defineProps({
    selectedChat: {
        type: Number,
        default: null,
        required: true
    }
})

const isLoading = ref<boolean>(false)
const content = ref<TextareaHTMLAttributes['value']>('')

const sendMessage = () => {
    isLoading.value = true

    fetch(`http://localhost:3000/api/v1/chats/${props.selectedChat}/messages`, { method: 'POST', headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': 'http://localhost:3000' }, body: JSON.stringify({ content: content.value }) })
        .then(response => response.json())
        .then((data: Message) => {
            isLoading.value = false

            content.value = ''
        })
        .catch(err => console.log(err))
}
</script>

<template>
	<div class="flex mt-2 mb-8 space-x-4">
        <textarea
            v-model="content"
            rows="8"
            placeholder="Hello.."
            class="w-full h-full p-2 border border-gray-300 rounded-lg focus:outline-none"
            >
        </textarea>

        <button
            @click="sendMessage"
            :disabled="isLoading"
            class="w-64 h-16 px-4 py-2 text-gray-700 border border-gray-300 rounded-lg"
            >
                <template v-if="isLoading">
                    Loading..
                </template>
                <template v-else>
                    Send
                </template>
        </button>
    </div>
</template>
