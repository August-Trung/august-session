<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { convertFileSrc, invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import RememberList, { MomentInfo } from '../components/RememberList.vue'

const moments = ref<MomentInfo[]>([])
const activeMoment = ref<MomentInfo | null>(null)
const activeScreenshotUrl = ref('')

const loadMoments = async () => {
  try {
    const list: any[] = await invoke('get_moments')
    
    // Resolve absolute urls for screenshots using paths resolved by the backend
    const mappedList = list.map((m) => {
      return {
        ...m,
        screenshotUrl: convertFileSrc(m.screenshot_path)
      }
    })
    
    moments.value = mappedList
    if (mappedList.length > 0) {
      activeMoment.value = mappedList[0]
      activeScreenshotUrl.value = mappedList[0].screenshotUrl
    } else {
      activeMoment.value = null
      activeScreenshotUrl.value = ''
    }
  } catch (e) {
    console.error("Failed to load moments:", e)
  }
}

onMounted(async () => {
  await loadMoments()

  // Real-time update list when moment is saved or deleted
  await listen('moment_saved', () => {
    loadMoments()
  })

  await listen('moment_deleted', () => {
    loadMoments()
  })
})

const selectMoment = (moment: MomentInfo) => {
  activeMoment.value = moment
  activeScreenshotUrl.value = (moment as any).screenshotUrl
}

const reopenDesk = async () => {
  if (!activeMoment.value) return
  console.log("Restoring desk for moment:", activeMoment.value.id)
  try {
    await invoke('restore_moment', { id: activeMoment.value.id })
  } catch (e) {
    console.error("Failed to restore moment:", e)
  }
}

const deleteMoment = async () => {
  if (!activeMoment.value) return
  if (!confirm("Are you sure you want to delete this Moment?")) return
  try {
    await invoke('delete_moment', { id: activeMoment.value.id })
  } catch (e) {
    console.error("Failed to delete moment:", e)
  }
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
        
        <!-- EMPTY STATE -->
        <div v-if="!activeMoment" class="text-center py-12">
          <v-icon icon="mdi-help-box" size="80" class="text-grey-darken-3 mb-4"></v-icon>
          <div class="text-h5 font-weight-bold text-white mb-2">Your desk is empty.</div>
          <div class="text-subtitle-1 text-grey-darken-1 mb-6">
            Press <v-chip class="mx-1" variant="outlined" density="compact" label>Ctrl+Shift+P</v-chip> to capture your first Moment!
          </div>
        </div>

        <div v-else>
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
                :src="activeScreenshotUrl"
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

          <!-- Action Buttons -->
          <div class="text-center d-flex justify-center align-center gap-4 mb-12">
            <v-btn
              color="primary"
              size="large"
              variant="flat"
              class="rounded-lg px-8 py-3 font-weight-bold mr-4"
              prepend-icon="mdi-arrow-right-drop-circle-outline"
              @click="reopenDesk"
            >
              Reopen My Desk
            </v-btn>

            <v-btn
              color="error"
              size="large"
              variant="outlined"
              class="rounded-lg px-6 py-3 font-weight-bold"
              prepend-icon="mdi-trash-can-outline"
              @click="deleteMoment"
            >
              Delete
            </v-btn>
          </div>
        </div>

        <v-divider v-if="moments.length > 0" class="mb-10 border-opacity-25"></v-divider>

        <!-- History Journal List -->
        <RememberList
          v-if="moments.length > 0 && activeMoment"
          :moments="moments"
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
.gap-4 {
  gap: 16px;
}
</style>
