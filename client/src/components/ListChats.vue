<script setup lang="ts">
import type { Chat } from '@/Models.vue'
import { ref } from 'vue'

let chats = ref<Array<Chat>>([])

fetch(`http://localhost:3000/api/v1/chats`, { method: 'GET', headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': 'http://localhost:3000' } })
    .then(response => response.json())
    .then((data: Array<Chat>) => chats.value = data)
    .catch(err => console.log(err))
</script>

<template>
	<div>
        <div
            v-for="chat in chats"
            :key="chat.id"
        >
            <button
                @click="$emit('selectedChatChanged', chat.id)"
            >
                {{ chat.title }}
            </button>
        </div>
    </div>
</template>
