<template>
  <div class="relative" :style="contStyle">
    <!-- frame top -->
    <img class="absolute" draggable="false" src="./head.png" :style="topStyle" />

    <!-- frame bg -->
    <img
      class="absolute"
      draggable="false"
      src="./bg.png"
      style="left: 0; top: -67px; z-index: 3"
    />
    <img
      class="absolute"
      draggable="false"
      src="./bg2.png"
      style="left: 0; top: -67px; z-index: 3"
    />

    <!-- content bg -->

    <div class="absolute" :style="contentStyle">
      <!-- content body -->
      <div class="flex flex-col h-[295px]">
        <!-- cover image -->
        <div class="relative shrink-30 overflow-hidden w-[242px] h-[140px] __cover">
          <img
            class="absolute"
            loading="lazy"
            draggable="false"
            v-if="props.mod?.textureLocation"
            :src="cover_img"
            :style="coverStyle"
          />
        </div>

        <!-- title & effect -->
        <div class="w-full" :style="{ color: pColor }">
          <!-- title -->
          <div class="text-center text-[24px]">{{ props.mod.name }}</div>
          <div
            class="text-center px-3"
            v-if="props.mod.uniqueName.startsWith('/Lotus/Upgrades/Mods/Warframe/Beginner')"
          >
            瑕疵的
          </div>
          <div class="text-center px-3" v-for="(stat, index) in effect" :key="index">
            <string-with-icon :text="stat" />
          </div>
        </div>
      </div>
    </div>

    <!-- frame bottom -->
    <img class="absolute" draggable="false" src="./bottom.png" :style="bottomStyle" />

    <!-- bottom type marker -->
    <div class="absolute z-5" :style="lowerTabStyle">
      <!-- <img draggable="false" src="@/assets/mod/LegendaryLowerTab.png" /> -->
      <div class="absolute top-0 left-0 right-0 bottom-0 content-center text-center">
        <span class="text-bold" :style="{ color: pColor }">{{ props.mod.compatName }}</span>
      </div>
    </div>

    <img class="absolute z-3" src="./slots.png" style="left: 64px; top: 282px" />

    <!-- rank stars -->
    <div class="absolute z-3" :style="rankStartsStyle">
      <img
        class="inline-block"
        draggable="false"
        v-for="i in currentRank"
        :key="i"
        src="./star.png"
        style="object-fit: cover; height: 32px"
        :style="{ width: starGap + 'px' }"
      />
    </div>
  </div>
  <div class="flex flex-row gap-3 z-6">
    <pv-button :disabled="currentRank <= 0" label="降低" @click="reduceRank()"></pv-button>
    <pv-button
      :disabled="currentRank >= props.mod.fusionLimit"
      label="升高"
      @click="increaseRank()"
    ></pv-button>
  </div>
  {{ props.mod.rarity }}
  {{ props.mod.polarity }}
</template>
<script lang="ts" setup>
import { computed, ref, watch } from 'vue'
import type { Upgrade } from '@/util/localDB'

const props = defineProps<{ mod: Upgrade; maxRank?: boolean }>()

const contStyle = {
  width: '256px',
  height: '404px',
  userDrag: 'none',
  overflow: 'hidden',
}

const topStyle = {
  top: '0',
  left: '-128px',
  zIndex: '6',
}
const contentStyle = computed(() => ({
  left: '7px',
  top: '18px',
  width: '242px',
  height: '338px',
  zIndex: '5',
  '--rarity-color': pColor,
}))

const bottomStyle = {
  top: '280px',
  left: '-128px',
  zIndex: '3',
}

const coverImageSize = 100

const coverStyle = {
  width: coverImageSize + 'px',
  height: coverImageSize + 'px',
  top: '46px',
  left: (242 - 100) / 2 + 'px',
}

const lowerTabStyle = computed(() => ({
  left: 128 - 105 + 'px',
  top: '290px',
  // top: 14 + bgSize.height - 42 + 'px',
  // zIndex: '40',
  width: '210px',
  height: '42px',
}))

const starGap = 24

const rankStartsStyle = computed(() => ({
  width: starGap * props.mod.fusionLimit + 'px',
  top: '275px',
  left: 128 - (starGap * props.mod.fusionLimit) / 2 + 'px',
}))

const pColor = '#9af5f7'

const cover_img = computed(() => {
  // console.log(props.mod)
  return 'https://content.warframe.com/PublicExport' + props.mod?.textureLocation
})
const currentRank = ref(0)
watch(
  () => props.mod,
  (v) => {
    if (props.maxRank) {
      currentRank.value = v.fusionLimit
    }
  },
  { immediate: true },
)
function increaseRank() {
  if (currentRank.value < props.mod?.fusionLimit) {
    currentRank.value++
  }
}

function reduceRank() {
  if (currentRank.value > 0) {
    currentRank.value--
  }
}

const effect = computed(() => {
  return props.mod?.levelStats?.[currentRank.value]?.stats || props.mod.description
})
</script>
<style scoped>
img {
  user-select: none;
  max-width: none;
}

.__cover {
  mask: linear-gradient(to top, transparent, #000 3px);
}

.__cover {
  position: relative;
}
</style>
