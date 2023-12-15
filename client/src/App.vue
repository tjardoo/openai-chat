<script setup lang="ts">
import { ref } from "vue";
import ListChats from "./components/ListChats.vue";
import ShowChat from "./components/ShowChat.vue";
import StoreChat from "./components/StoreChat.vue";

let selectedChat = ref<number|undefined>(undefined)

const setSelectedChat = (id: number) => {
    selectedChat.value = id
}
</script>

<template>
    <div class="flex">
        <div class="h-screen bg-gray-800 w-96">
            <ListChats @selected-chat-changed="setSelectedChat" />
        </div>
        <main class="w-full max-w-4xl mx-auto bg-gray-100">
            <div
                v-if="selectedChat !== undefined"
                class="flex flex-col h-screen"
            >
                <div class="my-6">
                    <h1 class="text-xl font-bold text-center">Show chat {{ selectedChat ?? 'X' }}</h1>
                </div>

                <div class="h-full px-3 overflow-y-auto">
                    <ShowChat :selected-chat="selectedChat" />
                </div>

                <div class="h-auto">
                    <StoreChat :selected-chat="selectedChat" />
                </div>
            </div>
        </main>
    </div>
</template>
