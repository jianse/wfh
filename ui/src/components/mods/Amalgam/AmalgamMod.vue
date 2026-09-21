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

    <!-- content bg -->

    <div class="absolute" :style="contentStyle">
      <!-- content body -->
      <div class="flex flex-col h-[295px]">
        <!-- cover image -->
        <div class="shrink-30 overflow-hidden __cover">
          <img
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

    <!-- side light -->
    <img class="absolute" draggable="false" src="./side.png" :style="rightSideStyle" />
    <img class="absolute" draggable="false" src="./side.png" :style="leftSideStyle" />
    <!-- conner light -->
    <img class="absolute" draggable="false" src="./conner.png" :style="rightCornerStyle" />
    <img class="absolute" draggable="false" src="./conner.png" :style="leftCornerStyle" />

    <!-- top right marker -->
    <div
      class="absolute flex flex-row place-content-center place-items-center pb-1 pr-1 __trMarker"
      v-if="props.mod.polarity !== 'AP_UNIVERSAL'"
      :style="trMarkerStyle"
    >
      <div class="text-[18px] font-bold" style="line-height: 1" :style="{ color: pColor }">
        {{ drain }}
      </div>
      <div class="inline-block z-5 ms-0.5" :style="polarityIconStyle"></div>
    </div>

    <!-- bottom type marker -->
    <div class="absolute z-5" :style="lowerTabStyle">
      <img draggable="false" src="@/assets/mod/LegendaryLowerTab.png" />
      <div class="absolute top-0 left-0 right-0 bottom-0 content-center text-center">
        <span class="text-bold" :style="{ color: pColor }">{{ props.mod.compatName }}</span>
      </div>
    </div>

    <!-- max level marker -->
    <img
      class="absolute z-3"
      v-if="rankMaxed"
      :src="rankCompleteLine.url"
      :style="rankCompleteLineStyle"
    />

    <!-- rank stars -->
    <div class="absolute z-3" :style="rankStartsStyle">
      <img
        class="inline-block"
        draggable="false"
        v-for="i in currentRank"
        :key="i"
        src="@/assets/mod/RankSlotActive.png"
      />
      <img
        class="inline-block"
        draggable="false"
        v-for="i in props.mod.fusionLimit - currentRank"
        :key="i"
        src="@/assets/mod/RankSlotEmpty.png"
      />
    </div>
  </div>
  <div class="flex flex-row gap-3">
    <button class="btn btn-secondary" :disabled="currentRank <= 0" @click="reduceRank()">降低</button>
    <button class="btn btn-secondary" :disabled="currentRank >= props.mod.fusionLimit" @click="increaseRank()">升高</button>
  </div>
  {{ props.mod.rarity }}
  {{ props.mod.polarity }}
</template>
<script lang="ts" setup>
import { computed, ref, watch } from 'vue'
import polarity from '@/assets/polarity'
import type { Upgrade } from '@/util/localDB'
import { rankCompleteLine } from 'virtual:mod-image-suit'

// const COLORS = { COMMON: '#dda085', UNCOMMON: '#e5e3e4', RARE: '#fffcd4', LEGENDARY: '#f5f9fc' }

const props = defineProps<{ mod: Upgrade; maxRank?: boolean }>()

// const imageSuit = computed(() => {
//   return getImageSuit(props.mod)
// })

const contStyle = {
  width: '256px',
  height: '404px',
  userDrag: 'none',
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
  top: '192px',
  left: '-128px',
  zIndex: '3',
}

const rightSideStyle = {
  top: '52px',
  left: '230px',
  zIndex: '2',
}

const leftSideStyle = {
  top: '52px',
  left: '-6px',
  zIndex: '2',
  transform: ' scaleX(-1)',
}

const coverStyle = {
  width: '242px',
  height: '242px',
}

import markerBg from '@/assets/mod/LegendaryTopRightBacker.png?url'
const trMarkerStyle = computed(() => ({
  top: '25px',
  right: '7px',
  zIndex: '5',
  '--tr-bg-url': `url("${markerBg}")`,
  backgroundImage: 'var(--tr-bg-url)',
}))

const rightCornerStyle = {
  right: '-5px',
  bottom: '32px',
  zIndex: '3',
}

const leftCornerStyle = {
  left: '-5px',
  bottom: '32px',
  zIndex: '3',
  transform: ' scaleX(-1)',
}

const lowerTabStyle = computed(() => ({
  left: 128 - 105 + 'px',
  bottom: '60px',
  // top: 14 + bgSize.height - 42 + 'px',
  // zIndex: '40',
}))

// const rankCompleteLine = rankCompleteLine

const rankCompleteLineStyle = computed(() => ({
  // top: 14 + bgSize.height + 8 + 'px',
  bottom: '24px',
  left: 0,
  // zIndex: '40',
}))

const rankStartsStyle = computed(() => ({
  bottom: '18px',

  left: 128 - (12 * props.mod.fusionLimit) / 2 + 'px',
  // zIndex: '40',
}))

const pColor = '#ffffff'

const polarityIconStyle = computed(() => {
  const mask_url = polarity[props.mod.polarity] || polarity.AP_ATTACK
  const style = {
    height: '14px',
    width: '14px',
    backgroundColor: pColor,
    top: '5px',
    right: '4px',
    mask: `url(${mask_url})`,
    maskSize: 'cover',
  }
  return style
})

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

const rankMaxed = computed(() => {
  return props.mod?.fusionLimit === currentRank.value
})
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
  if (props.mod?.type === 'STANCE') {
    return props.mod.description || props.mod?.levelStats?.[currentRank.value]?.stats
  }
  return props.mod?.levelStats?.[currentRank.value]?.stats
})

const drain = computed(() => {
  // if (props.mod.)
  return Math.abs(props.mod.baseDrain) + currentRank.value
})
</script>
<style scoped>
img {
  user-select: none;
  max-width: none;
}
/* #cc {
  mask: linear-gradient(transparent, #000 5px);
} */
.__cover {
  mask: linear-gradient(to top, transparent, #000 3px);
}

.__cover {
  position: relative;
}

.__cover::after {
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  right: 0;
  /* box-shadow: inset 0 0 10px 24px #dda085,
   0 0 10px 24px #dda085; */
  content: '';
  /* need more colors */
  background: radial-gradient(transparent 65%, var(--rarity-color));
}

.__trMarker {
  background-position-x: -20px;
  height: 26px;
  background-repeat: no-repeat;
  background-size: 64px 26px;
}
.__trMarker::before {
  content: '';
  position: absolute;
  background-image: var(--tr-bg-url);
  width: 15px;
  height: 26px;
  top: 0;
  left: -15px;
}
</style>
