<script setup lang="ts">
import { defineProps, defineEmits } from 'vue'

export interface MomentInfo {
  id: string
  words: string
  screenshot: string
  created_at: string
}

defineProps<{
  moments: MomentInfo[]
  selectedId?: string
}>()

const emit = defineEmits<{
  (e: 'select', moment: MomentInfo): void
}>()

const formatDate = (dateStr: string) => {
  try {
    const d = new Date(dateStr)
    return d.toLocaleString('en-US', {
      weekday: 'short',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    })
  } catch (e) {
    return dateStr
  }
}
</script>

<template>
  <div class="remember-list text-left">
    <div class="text-subtitle-2 font-weight-bold text-grey-darken-1 mb-3 px-1">
      Earlier Moments
    </div>
    
    <v-list class="pa-0 bg-transparent" lines="two">
      <v-list-item
        v-for="moment in moments"
        :key="moment.id"
        :value="moment"
        :active="selectedId === moment.id"
        active-color="primary"
        class="mb-2 rounded-lg border-thin list-item-custom"
        @click="emit('select', moment)"
      >
        <template v-slot:prepend>
          <v-icon icon="mdi-history" class="mr-2 text-grey-darken-1"></v-icon>
        </template>
        
        <v-list-item-title class="font-weight-medium text-body-1 text-white">
          {{ moment.words || '(No message left)' }}
        </v-list-item-title>
        
        <v-list-item-subtitle class="text-caption text-grey-darken-2 mt-1">
          {{ formatDate(moment.created_at) }}
        </v-list-item-subtitle>
      </v-list-item>
    </v-list>
  </div>
</template>

<style scoped>
.list-item-custom {
  background-color: #1A1A1A !important;
  border-color: #2D2D2D !important;
  transition: all 0.2s ease;
}
.list-item-custom:hover {
  background-color: #252525 !important;
  border-color: #3D3D3D !important;
}
</style>
