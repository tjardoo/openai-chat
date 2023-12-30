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

			this.streamingMessage = null
		},
		addMessage(content: string) {
			this.messages.push({
				id: 0,
				role: 'user',
				content: content,
				temperature: 1,
				created_at: new Date().toISOString()
			})
		},
		streamMessage(chunk: string) {
			if (this.streamingMessage === null) {
				this.streamingMessage = chunk
			} else {
				this.streamingMessage += chunk
			}
		}
	}
})
