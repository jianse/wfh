<template>
  <div class="relative" :style="contStyle">
    <!-- frame top -->
    <img class="absolute" draggable="false" :src="imageSuit.frameTop.url" :style="topStyle" />

    <!-- frame bg -->
    <img
      class="absolute"
      draggable="false"
      :src="imageSuit.background.url"
      style="left: 0; top: -67px"
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
    <img class="absolute" draggable="false" :src="imageSuit.frameBottom.url" :style="bottomStyle" />

    <!-- side light -->
    <img
      class="absolute"
      draggable="false"
      :src="imageSuit.sideLight.url"
      :style="rightSideStyle"
    />
    <img class="absolute" draggable="false" :src="imageSuit.sideLight.url" :style="leftSideStyle" />
    <!-- conner light -->
    <img
      class="absolute"
      draggable="false"
      :src="imageSuit.cornerLights.url"
      :style="rightCornerStyle"
    />
    <img
      class="absolute"
      draggable="false"
      :src="imageSuit.cornerLights.url"
      :style="leftCornerStyle"
    />

    <!-- top right marker -->
    <div
      class="absolute flex flex-row place-content-center place-items-center pb-1 pr-1 __trMarker"
      v-if="props.mod.polarity !== 'AP_UNIVERSAL'"
      :style="trMarkerStyle"
    >
      <div
        class="me-0.5"
        v-if="props.mod.type === 'STANCE' || props.mod.type === 'AURA'"
        style="width: 11px; height: 26px"
        :style="capacityUpIndicatorStyle"
      ></div>
      <div class="text-[18px] font-bold" style="line-height: 1" :style="{ color: pColor }">
        {{ drain }}
      </div>
      <div class="inline-block z-5 ms-0.5" :style="polarityIconStyle"></div>
    </div>

    <!-- bottom type marker -->
    <div class="absolute z-5" :style="lowerTabStyle">
      <img draggable="false" :src="imageSuit.lowerTab.url" />
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
    <!-- stance marker -->
    <img
      draggable="false"
      v-if="props.mod.type === 'STANCE'"
      class="absolute z-5"
      src="@/assets/mod/SilverStance2.png"
      :style="stanceStyle"
    />

    <!-- aura marker -->
    <img
      draggable="false"
      v-if="props.mod.type === 'AURA'"
      class="absolute z-5"
      src="@/assets/mod/SilverAura2.png"
      :style="stanceStyle"
    />

    <!-- utility marker -->
    <div
      draggable="false"
      v-if="props.mod.isUtility"
      class="absolute z-5"
      src="@/assets/mod/IconUtility2.png"
      :style="utilityStyle"
    ></div>

    <!-- umber marker -->
    <img
      draggable="false"
      v-if="props.mod.modSet === '/Lotus/Upgrades/Mods/Sets/Umbra/UmbraSetMod'"
      class="absolute z-6"
      src="@/assets/mod/UmbraHeader.png"
      :style="modSetStyle"
    />
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
import { getImageSuit, rankCompleteLine } from 'virtual:mod-image-suit'

const COLORS = { COMMON: '#dda085', UNCOMMON: '#e5e3e4', RARE: '#fffcd4', LEGENDARY: '#f5f9fc' }

const props = defineProps<{ mod: Upgrade; maxRank?: boolean }>()

const imageSuit = computed(() => {
  return getImageSuit(props.mod)
})

import capIcon from '@/assets/mod/CapacityUp_d.png'
const capacityUpIndicatorStyle = computed(() => ({
  backgroundColor: pColor.value,
  mask: `url("${capIcon}")`,
}))

const contStyle = {
  width: '256px',
  height: '404px',
  userDrag: 'none',
}

const topStyle = {
  top: '0',
  left: 0,
  zIndex: '6',
}
const contentStyle = computed(() => ({
  left: '7px',
  top: '18px',
  width: '242px',
  height: '338px',
  zIndex: '5',
  '--rarity-color': pColor.value,
}))

const bottomStyle = computed(() => {
  if (props.mod.rarity === 'LEGENDARY') {
    return {
      bottom: '12px',
      zIndex: '3',
    }
  }
  return {
    bottom: '0',
    zIndex: '3',
  }
})

const rightSideStyle = {
  top: '52px',
  right: '2px',
  zIndex: '5',
}

const leftSideStyle = {
  top: '52px',
  left: '2px',
  zIndex: '5',
  transform: ' scaleX(-1)',
}

const coverStyle = {
  width: '242px',
  height: '242px',
}

const trMarkerStyle = computed(() => ({
  top: '22px',
  right: '7px',
  zIndex: '5',
  '--tr-bg-url': `url(${imageSuit.value.topRight.url})`,
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
  left: imageSuit.value.background.width / 2 - imageSuit.value.lowerTab.width / 2 + 'px',
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

  left: imageSuit.value.background.width / 2 - (12 * props.mod.fusionLimit) / 2 + 'px',
  // zIndex: '40',
}))

const pColor = computed(() => {
  return COLORS[props.mod.rarity] || '#fff'
})

const polarityIconStyle = computed(() => {
  const mask_url = polarity[props.mod.polarity] || polarity.AP_ATTACK
  const style = {
    height: '14px',
    width: '14px',
    backgroundColor: pColor.value,
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

const stanceStyle = computed(() => ({
  left: 256 / 2 - 64 / 2 + 'px',
  top: '15px',
}))
import iconUtility2 from '@/assets/mod/IconUtility2.png'
const utilityStyle = computed(() => ({
  width: '64px',
  height: '39px',
  left: 256 / 2 - 64 / 2 + 'px',
  top: '15px',
  backgroundColor: pColor.value,
  mask: `url(${iconUtility2})`,
}))

const modSetStyle = {
  top: '-8px',
  left: 128 - 128 / 2 + 'px',
}
</script>
<style scoped>
img {
  user-select: none;
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
  content: '';
  /* need more colors */
  background: radial-gradient(transparent 65%, var(--rarity-color));
  /* background-position: 0 10%; */
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
