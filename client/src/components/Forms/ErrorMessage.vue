<script setup lang="ts">
import type { FieldValidatorError } from '../../Models.vue'

const props = defineProps({
	error: {
		type: Object as () => FieldValidatorError | null,
		default: null,
	}
})
</script>

<template>
	<div
		class="mt-1 text-xs text-red-500"
		v-if="error"
	>
		{{ props.error?.code }}
	</div>
</template>
