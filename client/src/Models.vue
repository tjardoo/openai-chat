<script lang="ts">
export interface Chat {
	id: number
	model_id: number | null
	title: string | null
	external_id: string | null
	created_at: string
}

export interface Message {
	id: number
	role: string
	content: string
	created_at: string
}
</script>
