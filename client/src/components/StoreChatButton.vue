<script setup lang="ts">
import PlusIcon from './Icons/PlusIcon.vue'

defineEmits(['createChat'])
</script>

<template>
	<button
		@click="$emit('createChat')"
		class="flex items-center w-full px-8 py-2 text-left text-white hover:bg-gray-600"
	>
		<PlusIcon class="inline-block w-4 h-4 mr-2 stroke-gray-500" />
		New chat
	</button>
</template>
