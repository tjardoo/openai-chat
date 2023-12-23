<script setup lang="ts">
import type { Chat, FieldValidatorError } from '@/Models.vue'
import type { TextareaHTMLAttributes } from 'vue'
import { computed, ref, watch } from 'vue'
import PaperAirplaneIcon from './Icons/PaperAirplaneIcon.vue'

const props = defineProps({
	selectedChat: {
		type: Object as () => Chat,
		default: null,
		required: true
	},
	models: {
		type: Array as () => Array<String>,
		default: [],
		required: true
	}
})

const isLoading = ref<boolean>(false)
const isError = ref<boolean>(false)
const validationErrors = ref<Array<FieldValidatorError>|null>(null)
const content = ref<TextareaHTMLAttributes['value']>('')
const model = ref<string | null>(null)
const maxTokens = ref<number | string>("")
const temperature = ref<number>(1)

const clearErrors = (): void => {
    isError.value = false
    isLoading.value = false
    validationErrors.value = null
}

const sendMessage = (): void => {
	isLoading.value = true

	fetch(`http://localhost:3000/api/v1/chats/${props.selectedChat.id}/messages`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': 'http://localhost:3000' },
		body: JSON.stringify({
			content: content.value,
			model: model.value,
			max_tokens: maxTokens.value,
			temperature: temperature.value
		})
	})
		.then((response) => {
            return response.json()
        })
		.then((data) => {
            if(data.errors !== undefined) {
                validationErrors.value = data.errors

                isError.value = true
                isLoading.value = false

                return
            }

			clearErrors()

			content.value = ''

			emit('messageSent')
		})
		.catch((error) => {
			isError.value = true
			isLoading.value = false

			console.log(error)
		})
}

watch(
	() => props.selectedChat,
	(first, second) => {
		if (first === undefined) {
			return
		}

        clearErrors()
        maxTokens.value = ""
        temperature.value = 1
        content.value = ''
		model.value = first.last_used_model
	},
	{ immediate: true }
)

const isSendButtonDisabled = computed((): boolean => {
	return content.value === '' || isLoading.value === true || model.value === null
})

const isFieldInvalid = (field: string): boolean => {
    if (validationErrors.value === null) {
        return false
    }

    return validationErrors.value.some((fieldValidatorError: FieldValidatorError) => fieldValidatorError.name === field)
}

const getValidationErrorMessage = (field: string): String => {
    if (validationErrors.value === undefined) {
        return 'Error'
    }

    const fieldValidatorError = validationErrors.value?.find((fieldValidatorError: FieldValidatorError) => fieldValidatorError.name === field)

    if (fieldValidatorError === undefined) {
        return 'Error'
    }

    return fieldValidatorError.code
}

const emit = defineEmits(['messageSent'])
</script>

<template>
	<div class="mt-2 mb-8">
		<div class="flex space-x-4">
			<div class="w-3/4">
				<textarea
                    v-model="content"
                    rows="8"
                    placeholder="Hello.."
                    class="w-full h-full p-2 border border-gray-300 rounded-lg focus:outline-none"
                    :class="{
                        'border-red-500': isFieldInvalid('content')
                    }"
                />
			</div>

			<div class="w-1/4 space-y-1">
				<button
                    @click="sendMessage"
                    :disabled="isSendButtonDisabled"
                    class="flex items-center justify-end w-full h-16 px-4 py-2 text-gray-700 bg-white border border-gray-300 rounded-lg disabled:cursor-not-allowed disabled:opacity-50"
                >
					<template v-if="isLoading">Loading..</template>
					<template v-else> Send <PaperAirplaneIcon class="ml-3" /> </template>
				</button>

				<div>
					<label for="model" class="text-gray-500 text-[10px]">
                        OpenAI model
                    </label>
					<select
                        id="model"
                        v-model="model"
                        class="w-full px-4 py-2 text-gray-700 border border-gray-300 rounded-lg"
                        :class="{
                            'border-red-500': isFieldInvalid('model')
                        }"
                    >
						<template v-for="model in models" :key="model">
							<option :value="model">
                                {{ model }}
                            </option>
						</template>
					</select>
                    <div class="mt-1 text-xs text-red-500" v-if="isFieldInvalid('model')">
                        {{ getValidationErrorMessage('model') }}
                    </div>
				</div>

				<div class="flex space-x-2">
					<div class="w-1/2">
						<label for="max_tokens" class="text-gray-500 text-[10px]">
                            Max. tokens
                        </label>
						<input
                            type="text"
                            id="max_tokens"
                            v-model="maxTokens"
                            class="w-full px-4 py-2 text-gray-700 border border-gray-300 rounded-lg"
                            :class="{
                                'border-red-500': isFieldInvalid('max_tokens')
                            }"
                        />
                        <div class="mt-1 text-xs text-red-500" v-if="isFieldInvalid('max_tokens')">
                            {{ getValidationErrorMessage('max_tokens') }}
                        </div>
					</div>

					<div class="w-1/2">
						<label for="temperature" class="text-gray-500 text-[10px]">
                            Temperature
                        </label>
						<input
                            type="number"
                            id="temperature"
                            v-model="temperature"
                            class="w-full px-4 py-2 text-gray-700 border border-gray-300 rounded-lg"
                            :class="{
                                'border-red-500': isFieldInvalid('temperature')
                            }"
                            min="0"
                            max="2"
                            step="0.1"
                        />
                        <div class="mt-1 text-xs text-red-500" v-if="isFieldInvalid('temperature')">
                            {{ getValidationErrorMessage('temperature') }}
                        </div>
					</div>
				</div>
			</div>
		</div>
		<div v-if="isError">
			<div class="py-2 text-sm text-red-500">
                Something went wrong. Please try again.
            </div>
		</div>
	</div>
</template>
