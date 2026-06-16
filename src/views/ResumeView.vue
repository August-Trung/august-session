<script setup lang="ts">
import { ref } from 'vue'
import RememberList, { MomentInfo } from '../components/RememberList.vue'
import mockDesktop from '../assets/mock_desktop.png'

// Mock data for M3 layout validation
const mockMoments = ref<MomentInfo[]>([
  {
    id: '1',
    words: 'Finish the last 3 slides. Print worksheets for Period 3.',
    screenshot: mockDesktop,
    created_at: '2026-06-16T17:18:00+07:00',
  },
  {
    id: '2',
    words: 'Check if the Airbnb near Sagrada Família has a crib.',
    screenshot: mockDesktop,
    created_at: '2026-06-15T20:45:00+07:00',
  },
  {
    id: '3',
    words: 'Dado blade width matters — check before ordering lumber.',
    screenshot: mockDesktop,
    created_at: '2026-06-14T18:30:00+07:00',
  },
])

const activeMoment = ref<MomentInfo>(mockMoments.value[0])

const selectMoment = (moment: MomentInfo) => {
  activeMoment.value = moment
}

const reopenDesk = () => {
  console.log("Reopening desk layout for:", activeMoment.value.words)
  // In M4: connect restore Tauri IPC command
}

const formatDateRelative = (dateStr: string) => {
  try {
    const d = new Date(dateStr)
    return d.toLocaleDateString('en-US', {
      weekday: 'long',
      hour: '2-digit',
      minute: '2-digit',
    })
  } catch (e) {
    return dateStr
  }
}
</script>

<template>
  <v-container class="py-12 px-6 main-container" fluid>
    <v-row justify="center">
      <v-col cols="12" sm="10" md="8" lg="6">
        <!-- Main Words Display -->
        <div class="text-center mb-8">
          <div class="text-subtitle-1 text-grey-darken-1 mb-2 font-weight-medium uppercase-label">
            Welcome back. You left off here:
          </div>
          <div class="text-h4 font-weight-bold text-white px-4 py-2 words-highlight">
            "{{ activeMoment.words || '(No message left)' }}"
          </div>
          <div class="text-caption text-grey-darken-2 mt-2">
            — Captured {{ formatDateRelative(activeMoment.created_at) }}
          </div>
        </div>

        <!-- Desk Photograph Preview -->
        <v-hover v-slot="{ isHovering, props }">
          <v-card
            v-bind="props"
            class="mx-auto rounded-lg border-thin overflow-hidden mb-8 desk-card"
            elevation="12"
            max-width="560"
            @click="reopenDesk"
          >
            <v-img
              :src="activeMoment.screenshot"
              aspect-ratio="16/9"
              cover
              class="desk-img"
            >
              <div
                class="fill-height d-flex align-center justify-center overlay-effect"
                :class="{ 'show-overlay': isHovering }"
              >
                <v-icon icon="mdi-play-circle" size="72" color="white"></v-icon>
              </div>
            </v-img>
          </v-card>
        </v-hover>

        <!-- Main Action Button -->
        <div class="text-center mb-12">
          <v-btn
            color="primary"
            size="large"
            variant="flat"
            class="rounded-lg px-8 py-3 font-weight-bold"
            prepend-icon="mdi-arrow-right-drop-circle-outline"
            @click="reopenDesk"
          >
            Reopen My Desk
          </v-btn>
        </div>

        <v-divider class="mb-10 border-opacity-25"></v-divider>

        <!-- History Journal List -->
        <RememberList
          :moments="mockMoments"
          :selectedId="activeMoment.id"
          @select="selectMoment"
        />
      </v-col>
    </v-row>
  </v-container>
</template>

<style scoped>
.main-container {
  max-width: 900px;
  margin: 0 auto;
}
.uppercase-label {
  letter-spacing: 1.5px;
  font-size: 0.85rem !important;
}
.words-highlight {
  line-height: 1.4;
  word-break: break-word;
}
.desk-card {
  border-color: #2D2D2D !important;
  cursor: pointer;
  background-color: #121212 !important;
}
.desk-img {
  transition: transform 0.4s cubic-bezier(0.25, 0.8, 0.25, 1);
}
.desk-card:hover .desk-img {
  transform: scale(1.03);
}
.overlay-effect {
  background: rgba(0, 0, 0, 0.4);
  opacity: 0;
  transition: opacity 0.3s ease;
}
.show-overlay {
  opacity: 1;
}
</style>
