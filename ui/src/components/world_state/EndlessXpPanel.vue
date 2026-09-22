<script setup lang="ts">
import { computed } from 'vue'
import { computedAsync } from '@vueuse/core'
import type { RawWorldState } from '@/api/world_state'
import { findWarframe, findResource } from '@/util/localDB'
import WorldStatePanel from './WorldStatePanel.vue'
import CountDown from '@/components/CountDown.vue'

const props = defineProps<{ rawWorldState: RawWorldState | undefined; icon?: string }>()
const emit = defineEmits<{ timeUp: [] }>()

interface ChoiceItem {
  name: string
  uniqueName: string
  url: string
}
interface CatItem {
  Category: string
  items: ChoiceItem[]
}

const sched = computed(() => props.rawWorldState?.EndlessXpSchedule?.[0])

const IMG = 'https://content.warframe.com/PublicExport'

async function iconFor(category: string, uniqueName: string): Promise<string> {
  switch (category) {
    case 'EXC_NORMAL':
      return (await findWarframe(uniqueName))?.icon || ''
    case 'EXC_HARD':
      return IMG + (await findResource(uniqueName))?.textureLocation || ''
  }
  return ''
}

const cats = computedAsync<CatItem[]>(async () => {
  const choices = props.rawWorldState?.EndlessXpSchedule?.[0]?.CategoryChoices ?? []
  const out: CatItem[] = []
  for (const cat of choices) {
    const names = cat.Choices ?? []
    const exs = cat.ChoicesEx ?? []
    const items: ChoiceItem[] = []
    for (let i = 0; i < names.length; i++) {
      const uniqueName = exs[i] ?? ''
      items.push({
        name: names[i],
        uniqueName,
        url: uniqueName ? await iconFor(cat.Category, uniqueName) : '',
      })
    }
    out.push({ Category: cat.Category, items })
  }
  return out
}, [])
</script>

<template>
  <WorldStatePanel :title="'无尽回廊'" :icon="props.icon">
    <div v-if="!cats.length" class="ws-empty">暂无无尽回廊轮换</div>
    <template v-else>
      <div class="mb-sm"><CountDown :time="sched?.Expiry" @hit-zero="emit('timeUp')" />轮换</div>
      <div class="ws-endless">
        <div v-for="c in cats" :key="c.Category">
          <div class="ws-endless-cat">{{ c.Category === 'EXC_NORMAL' ? '普通' : '困难' }}</div>
          <div class="ws-endless-grid">
            <div v-for="it in c.items" :key="it.uniqueName || it.name" class="ws-endless-item">
              <WfPepImg :uniqueName="it.url" :alt="it.name" loading="lazy" />
              <span>{{ it.name }}</span>
            </div>
          </div>
        </div>
      </div>
    </template>
  </WorldStatePanel>
</template>
