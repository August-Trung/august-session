<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const words = ref('')
const closeEverything = ref(false)
const inputRef = ref<any>(null)

onMounted(() => {
  // Focus the input area on load
  setTimeout(() => {
    if (inputRef.value) {
      const textarea = inputRef.value.$el.querySelector('textarea')
      if (textarea) textarea.focus()
    }
  }, 100)
})

const submitPause = async () => {
  const note = words.value.trim()
  if (!note) return
  try {
    await invoke('save_moment', {
      words: note,
      closeEverything: closeEverything.value
    })
  } catch (e) {
    console.error("Failed to save moment:", e)
  }
}
</script>

<template>
  <v-card class="pa-6 rounded-lg border-thin fill-height d-flex flex-column" color="#1A1A1A" flat>
    <v-card-title class="text-h6 font-weight-bold px-0 pb-1">
      August Session
    </v-card-title>
    <v-card-subtitle class="px-0 pb-3 text-grey-darken-1">
      What should you remember?
    </v-card-subtitle>
    
    <v-textarea
      ref="inputRef"
      v-model="words"
      label="Type your message to future-self..."
      variant="outlined"
      rows="3"
      no-resize
      hide-details
      class="mb-3"
      @keydown.enter.prevent="submitPause"
    ></v-textarea>

    <v-checkbox
      v-model="closeEverything"
      label="Close everything"
      color="primary"
      density="compact"
      hide-details
      class="mb-2"
    ></v-checkbox>

    <v-card-actions class="justify-end px-0 mt-auto">
      <v-btn color="primary" variant="flat" class="px-6 rounded-lg" @click="submitPause">
        Done
      </v-btn>
    </v-card-actions>
  </v-card>
</template>

<style scoped>
.v-card {
  border-radius: 8px !important;
}
</style>
