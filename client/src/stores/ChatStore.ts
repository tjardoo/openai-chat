import { defineStore } from 'pinia'
import type { Chat } from '@/Models.vue'

export const useChatStore = defineStore('chatStore', {
	state: () => {
		return {
			activeChat: null as Chat | null
		}
	},
	getters: {
		id: (state) => state.activeChat?.id,
		title: (state) => state.activeChat?.title
	},
	actions: {
		setActiveChat(chat: Chat) {
			this.activeChat = chat
		},
		updateActiveChatTitle(title: string) {
			if (this.activeChat) {
				this.activeChat.title = title
			}
		}
	}
})
