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
    test
    <aside class="h-full">
		<ListChats @selected-chat-changed="setSelectedChat" />
    </aside>
	<main>
        <div v-if="selectedChat !== undefined">
            <ShowChat :selected-chat="selectedChat" />

            <StoreChat :selected-chat="selectedChat" />
        </div>
	</main>
</template>
