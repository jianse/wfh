<template>
  <div class="relative" :style="contStyle">
    <!-- frame top -->
    <img class="absolute" draggable="false" src="./head.png" :style="topStyle" />

    <!-- frame bg -->
    <img class="absolute" draggable="false" src="./bg.png" style="left: 0; top: -67px" />

    <!-- content bg -->

    <div class="absolute" :style="contentStyle">
      <!-- content body -->
      <div class="flex flex-col h-[295px]">
        <!-- cover image -->
        <div class="shrink-30 overflow-hidden __cover">
          <img
            class="absolute"
            loading="lazy"
            draggable="false"
            :src="cover_img"
            :style="coverStyle"
          />
        </div>

        <!-- title & effect -->
        <div class="w-full" :style="{ color: pColor }">
          <!-- title -->
          <div class="text-center text-[24px]">{{ props.mod.name }}</div>
          <div class="text-center px-3" v-for="(stat, index) in effect" :key="index">
            <string-with-icon :text="stat" />
          </div>
        </div>
      </div>
    </div>

    <!-- frame bottom -->
    <img class="absolute" draggable="false" src="./bottom.png" :style="bottomStyle" />

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
      <img draggable="false" src="@/assets/mod/GoldLowerTab.png" />
      <div class="absolute top-0 left-0 right-0 bottom-0 content-center text-center">
        <span class="text-bold" :style="{ color: pColor }">{{ props.mod.compatName }}</span>
      </div>
    </div>

    <img class="absolute z-3" src="./slots.png" style="left: 64px; top: 355px" />

    <!-- max level marker -->
    <img class="absolute z-3" v-if="rankMaxed" src="./max.png" :style="rankCompleteLineStyle" />

    <!-- rank stars -->
    <div class="absolute z-3" :style="rankStartsStyle">
      <img
        class="inline-block"
        draggable="false"
        v-for="i in currentRank"
        :key="i"
        src="./star2.png"
        style="object-fit: cover; height: 32px"
        :style="{ width: starGap + 'px' }"
        :alt="i + ''"
      />
    </div>

    <!-- utility marker -->
    <div
      draggable="false"
      v-if="props.mod.isUtility"
      class="absolute z-5"
      src="@/assets/mod/IconUtility2.png"
      :style="utilityStyle"
    ></div>
  </div>
  <div class="flex flex-row gap-3 z-6">
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

// const COLORS = { COMMON: '#dda085', UNCOMMON: '#e5e3e4', RARE: '#fffcd4', LEGENDARY: '#f5f9fc' }

const props = defineProps<{ mod: Upgrade; maxRank?: boolean }>()

// const c2 = '#97c2d2'
const c3 = '#63b6d6'
// const c4 = '#6ab6d6'

// const imageSuit = computed(() => {
//   return getImageSuit(props.mod)
// })
import bg2 from './bg2.png'
const contStyle = {
  width: '256px',
  height: '404px',
  userDrag: 'none',
  // background: `url(${bg2})`
  overflow: 'hidden',
}

const topStyle = {
  top: '-11px',
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
  background: `url(${bg2})`,
  backgroundSize: 'contain',
  backgroundRepeat: 'no-repeat',
}))

const bottomStyle = {
  top: '190px',
  zIndex: '3',
  left: '-128px',
}

const coverStyle = computed(() => ({
  width: '120px',
  height: '120px',
  top: '50px',
  left: (242 - 120) / 2 + 'px',
  // backgroundColor: c2,
  // mask: `url(${cover_img.value})`,
  // maskSize: 'cover',
  // opacity: '0.',
  filter: ` drop-shadow(2px 2px 5px ${c3}) drop-shadow(-2px -2px 5px ${c3})`,
}))
import trm from '@/assets/mod/GoldTopRightBacker.png'
const trMarkerStyle = computed(() => ({
  top: '35px',
  right: '8px',
  zIndex: '6',
  '--tr-bg-url': `url(${trm})`,
  backgroundImage: 'var(--tr-bg-url)',
}))

const lowerTabStyle = computed(() => ({
  left: 128 - 105 + 'px',
  bottom: '60px',
  // top: 14 + bgSize.height - 42 + 'px',
  // zIndex: '40',
}))

// const rankCompleteLine = rankCompleteLine

const rankCompleteLineStyle = computed(() => ({
  // top: 14 + bgSize.height + 8 + 'px',
  top: '355px',
  left: (256 - 128) / 2 + 'px',
  // zIndex: '40',
}))

const starGap = 26

const rankStartsStyle = computed(() => ({
  width: starGap * props.mod.fusionLimit + 'px',
  top: '348px',

  left: 128 - (starGap * props.mod.fusionLimit) / 2 + 'px',
  // zIndex: '40',
}))

const pColor = '#fffcd4'

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
  return props.mod?.levelStats?.[currentRank.value]?.stats || props.mod.description
})

const drain = computed(() => {
  // if (props.mod.)
  return Math.abs(props.mod.baseDrain) + currentRank.value
})

import iconUtility2 from '@/assets/mod/IconUtility2.png'
const utilityStyle = computed(() => ({
  width: '64px',
  height: '39px',
  left: 256 / 2 - 64 / 2 + 'px',
  top: '15px',
  backgroundColor: pColor,
  mask: `url(${iconUtility2})`,
}))
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
  width: 242px;
  height: 242px;
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
