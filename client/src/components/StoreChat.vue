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
        })
        .catch(err => console.log(err))
}
</script>

<template>
	<div>
        <textarea v-model="content" placeholder="Hello.."></textarea>

        <button @click="sendMessage" :disabled="isLoading">Send</button>
    </div>
</template>
