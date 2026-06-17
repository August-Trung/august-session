<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { appWindow } from '@tauri-apps/api/window'

const words = ref('')
const closeEverything = ref(false)
const inputRef = ref<any>(null)

const cancelPause = async () => {
  try {
    await appWindow.close()
  } catch (e) {
    console.error("Failed to close window:", e)
  }
}

onMounted(() => {
  // Focus the input area on load
  setTimeout(() => {
    if (inputRef.value) {
      const textarea = inputRef.value.$el.querySelector('textarea')
      if (textarea) textarea.focus()
    }
  }, 100)

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === 'Escape') {
      cancelPause()
    }
  }
  window.addEventListener('keydown', handleKeyDown)

  onUnmounted(() => {
    window.removeEventListener('keydown', handleKeyDown)
  })
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
      <v-btn color="grey-darken-1" variant="text" class="px-6 rounded-lg mr-2" @click="cancelPause">
        Cancel
      </v-btn>
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
