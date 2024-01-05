import { defineStore } from 'pinia'
import type { Message } from '@/Models.vue'

export const useMessagesStore = defineStore('messagesStore', {
	state: () => {
		return {
			messages: [] as Message[],
			streamingMessage: null as string | null
		}
	},
	actions: {
		setMessages(messages: Message[]) {
			this.messages = messages

			this.clearStreamingMessage()
		},
		addMessage(message: Message) {
			this.messages.unshift(message)
		},
		streamMessage(chunk: string) {
			this.streamingMessage = chunk
		},
		clearStreamingMessage() {
			this.streamingMessage = null
		}
	}
})
