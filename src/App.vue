<script setup lang="ts">
import { ref, onMounted } from 'vue'

const currentView = ref('resume')

onMounted(() => {
  const updateRoute = () => {
    const hash = window.location.hash
    if (hash.includes('/pause')) {
      currentView.value = 'pause'
    } else {
      currentView.value = 'resume'
    }
  }

  window.addEventListener('hashchange', updateRoute)
  updateRoute()
})
</script>

<template>
  <v-app theme="dark">
    <v-main class="d-flex align-center justify-center fill-height">
      <!-- PAUSE OVERLAY VIEW -->
      <v-container v-if="currentView === 'pause'" class="text-center pa-4">
        <v-card class="mx-auto pa-6 rounded-lg border-thin" max-width="460" color="#1A1A1A" elevation="12">
          <v-card-title class="text-h6 font-weight-bold text-left px-0 pb-1">
            August Session
          </v-card-title>
          <v-card-subtitle class="text-left px-0 pb-4 text-grey-darken-1">
            Pause Workspace (M1 Backend Connected)
          </v-card-subtitle>
          <v-textarea
            label="What should you remember?"
            variant="outlined"
            placeholder="Write a sentence to future-you..."
            rows="3"
            hide-details
            disabled
            class="mb-4"
          ></v-textarea>
          <v-card-actions class="justify-end px-0">
            <v-btn color="primary" variant="flat" disabled class="px-6">Done</v-btn>
          </v-card-actions>
        </v-card>
      </v-container>

      <!-- RESUME/MAIN VIEW -->
      <v-container v-else class="text-center pa-4">
        <v-card class="mx-auto pa-8 rounded-lg border-thin" max-width="620" color="#1A1A1A" elevation="8">
          <v-card-title class="text-h4 font-weight-black text-left px-0 pb-2">
            August Session
          </v-card-title>
          <v-card-subtitle class="text-subtitle-1 text-left px-0 pb-6 text-grey-darken-1">
            A Workspace Time Machine
          </v-card-subtitle>
          
          <v-alert
            type="info"
            color="blue-darken-4"
            variant="tonal"
            class="text-left mb-6 rounded-lg"
            title="Milestone 1 Backend Loaded"
          >
            SQLite schema initialization is active. System tray icon and Ctrl+Shift+P global hotkey are running. The interactive user forms and controls will be wired up during Milestone 3.
          </v-alert>

          <v-btn color="grey-darken-3" disabled block size="large" variant="flat" class="rounded-lg">
            Reopen My Desk
          </v-btn>
        </v-card>
      </v-container>
    </v-main>
  </v-app>
</template>

<style>
html, body {
  overflow: hidden;
  user-select: none;
  background-color: #121212 !important;
}
</style>