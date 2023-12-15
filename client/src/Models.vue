<script lang="ts">
    export interface Chat {
        id: number
        model_id: number
        title: string,
        created_at: string
        external_id: string
    }

    export interface Message {
        id: number
        role: string
        content: string,
        created_at: string
    }
</script>
