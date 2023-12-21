<script lang="ts">
export interface Chat {
	id: number
	model_id: number | null
	title: string | null
	created_at: string
	last_used_model: string | null
}

export interface Message {
	id: number
	role: string
	content: string
	prompt_tokens: number | null
	completion_tokens: number | null
	created_at: string
}
</script>
