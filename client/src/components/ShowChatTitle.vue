<script setup lang="ts">
import type { Chat } from '@/Models.vue'
import { ref, watch } from 'vue'

const props = defineProps({
	selectedChat: {
		type: Object as () => Chat,
		default: null,
		required: true
	}
})

const chatName = ref<string | undefined>()

watch(
	() => props.selectedChat,
	(first, second) => {
		if (first === undefined) {
			return
		}

		if (first !== second) {
			chatName.value = first.title ?? 'Chat ' + first.id
		}
	},
	{ immediate: true }
)

defineEmits(['updateChatTitle'])
</script>

<template>
	<div class="flex space-x-4">
		<input type="text" v-model="chatName" placeholder="Chat name" class="w-full px-4 py-2 text-gray-700 border border-gray-300 rounded-lg" @change="$emit('updateChatTitle', chatName)" />

		<button :disabled="chatName === ''" class="px-4 py-2 text-gray-700 border border-gray-300 rounded-lg disabled:cursor-not-allowed disabled:opacity-50">Update</button>
	</div>
</template>
