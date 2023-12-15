<script setup lang="ts">
import type { Message } from '@/Models.vue';
import { ref, watch } from 'vue';

const props = defineProps({
    selectedChat: {
        type: Number,
        default: null,
        required: true
    }
})

const isLoading = ref<boolean>(false)
const messages = ref<Array<Message>>([])

watch(() => props.selectedChat, (first, second) => {
    if(first == undefined) {
        return;
    }

    if(first == second) {
        return;
    }

    isLoading.value = true

    fetch(`http://localhost:3000/api/v1/chats/${first}/messages`, { method: 'GET', headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': 'http://localhost:3000' } })
        .then(response => response.json())
        .then((data: Array<Message>) => {
            messages.value = data
            isLoading.value = false
        })
        .catch(err => console.log(err))
});
</script>

<template>
	<h1 className="text-3xl font-bold underline text-blue-500">Show chat {{ props.selectedChat ?? 'X' }}</h1>

    <div v-if="isLoading">
        Loading...
    </div>

    <div
        v-for="message in messages"
        :key="message.created_at"
    >
        <p>
            <span class="font-bold">{{ message.role }}</span>:
            {{ message.content }}
        </p>
    </div>
</template>
