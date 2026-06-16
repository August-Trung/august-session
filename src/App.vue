<script setup lang="ts">
import { ref, onMounted } from 'vue'
import PauseOverlay from './views/PauseOverlay.vue'
import ResumeView from './views/ResumeView.vue'

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
    <v-main class="fill-height">
      <!-- PAUSE OVERLAY VIEW -->
      <PauseOverlay v-if="currentView === 'pause'" />

      <!-- RESUME/MAIN VIEW -->
      <ResumeView v-else />
    </v-main>
  </v-app>
</template>

<style>
html, body {
  overflow-y: auto !important;
  user-select: none;
  background-color: #121212 !important;
}
/* Custom premium scrollbar styling */
::-webkit-scrollbar {
  width: 8px;
}
::-webkit-scrollbar-track {
  background: #121212;
}
::-webkit-scrollbar-thumb {
  background: #2D2D2D;
  border-radius: 4px;
}
::-webkit-scrollbar-thumb:hover {
  background: #3D3D3D;
}
</style>