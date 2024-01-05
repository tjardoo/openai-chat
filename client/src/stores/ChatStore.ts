import { defineStore } from 'pinia'
import type { Chat } from '@/Models.vue'

export const useChatStore = defineStore('chatStore', {
	state: () => {
		return {
			activeChat: null as Chat | null
		}
	},
	actions: {
		setActiveChat(chat: Chat) {
            // @todo when null, it needs to triggers for the other components to update
			this.activeChat = chat
		},
        updateActiveChatTitle(title: string) {
            if (this.activeChat) {
                this.activeChat.title = title
            }
        }
	}
})
