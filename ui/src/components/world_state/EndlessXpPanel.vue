<template>
  <pv-card>
    <template #title>
      <div class="flex items-center gap-2">
        <i class="pi pi-infinity"></i>
        <span>无尽回廊</span>
      </div>
    </template>
    <template #content>
      <div class="flex flex-col gap-3">
        <div v-for="cat in exp" :key="cat.Category">
          <div class="font-bold mb-2">{{ cat.Category === 'EXC_NORMAL' ? '普通' : '困难' }}</div>
          <ul class="list-disc list-outside pl-4 text-sm space-y-1">
            <li
              v-for="item in cat.ChoicesComputed"
              :key="item.uniqueName"
              class="flex items-center gap-2"
            >
              <img
                v-if="item.url"
                :src="item.url"
                class="h-5 w-5 inline-block align-text-bottom"
                :alt="item.name"
              />
              <span>{{ item.name }}</span>
            </li>
          </ul>
        </div>
      </div>
    </template>
  </pv-card>
</template>

<script setup lang="ts">
import type { RawWorldState, EndlessXpChoice } from '@/api/world_state'
import { findWarframe, findResource } from '@/util/localDB'
import { cloneDeep } from 'lodash'
import { computedAsync } from '@vueuse/core'

const props = defineProps<{
  rawWorldState: RawWorldState | undefined
}>()

type ExcEx = EndlessXpChoice & {
  ChoicesComputed: {
    name: string
    url: string
    uniqueName: string
  }[]
}

const exp = computedAsync(async () => {
  const res = cloneDeep(props.rawWorldState?.EndlessXpChoices) as ExcEx[]
  if (!res) return []
  for (const cat of res) {
    cat.ChoicesComputed = []
    for (const idx in cat.ChoicesEx) {
      const url = await getItemIcon(cat.Category, cat.ChoicesEx[idx])
      console.log(cat.ChoicesEx[idx], url)
      cat.ChoicesComputed[idx] = {
        name: cat.Choices[idx],
        url: getImageUrl(url),
        uniqueName: cat.ChoicesEx[idx],
      }
    }
  }

  return res
}, [])

async function getItemIcon(category: string, uniqueName: string): Promise<string | undefined> {
  if (category === 'EXC_NORMAL') {
    const warframe = await findWarframe(uniqueName)
    // console.log(warframe)
    return warframe?.textureLocation
  } else if (category === 'EXC_HARD') {
    const resource = await findResource(uniqueName)
    // console.log(resource)
    return resource?.textureLocation
  } else {
    return undefined
  }
}

function getImageUrl(textureLocation: string | undefined): string {
  if (!textureLocation) return ''
  return `https://content.warframe.com/PublicExport${textureLocation}`
}
</script>
