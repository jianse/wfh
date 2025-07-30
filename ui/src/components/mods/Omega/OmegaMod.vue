<template>
  <div v-if="!props.mod">??? no mod</div>
  <div v-else class="relative" :style="contStyle">
    <!-- frame top -->
    <img class="absolute" draggable="false" src="./head.png" style="left: -128px; z-index: 6" />

    <!-- frame bg -->
    <img class="absolute" draggable="false" src="./bg.png" style="top: -73px; z-index: 2" />

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
          <div class="text-center px-3" v-for="(stat, index) in effect" :key="index">
            <string-with-icon :text="stat" />
          </div>
        </div>
      </div>
    </div>

    <!-- frame bottom -->
    <img
      class="absolute"
      draggable="false"
      src="./bottom.png"
      style="left: -128px; z-index: 6; top: 184px"
    />

    <!-- side light -->
    <img
      class="absolute"
      draggable="false"
      src="./side.png"
      style="left: -4px; transform: scaleX(-1); top: 50px; z-index: 1"
    />
    <img
      class="absolute"
      draggable="false"
      src="./side.png"
      style="left: 228px; top: 50px; z-index: 1"
    />
    <!-- conner light -->
    <img
      class="absolute"
      draggable="false"
      src="./conner.png"
      style="z-index: 6; transform: scaleX(-1); top: 303px; left: -6px"
    />
    <img
      class="absolute"
      draggable="false"
      src="./conner.png"
      style="z-index: 6; left: 198px; top: 303px"
    />

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
      <img draggable="false" :src="imageSuit.lowerTab.url" />
      <div class="absolute top-0 left-0 right-0 bottom-0 content-center text-center">
        <span class="text-bold" :style="{ color: pColor }">？？？</span>
      </div>
    </div>

    <!-- max level marker -->
    <img
      class="absolute z-6"
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
        v-for="i in fusionLimit - currentRank"
        :key="i"
        src="@/assets/mod/RankSlotEmpty.png"
      />
    </div>
  </div>
  <div class="flex flex-row gap-3 relative z-6">
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
import { computed, ref, shallowRef, watch } from 'vue'
import polarity from '@/assets/polarity'
import type { Upgrade } from '@/util/localDB'
import { rankCompleteLine, OmegaSuit } from 'virtual:mod-image-suit'
import { getModClass } from '@/util/mod'

const props = defineProps<{ mod: Upgrade; maxRank?: boolean }>()

const imageSuit = shallowRef(OmegaSuit)
const pColor = '#ac83d5'

const contStyle = {
  width: imageSuit.value.frameBottom.width + 'px',
  height: '404px',
  userDrag: 'none',
}

const contentStyle = computed(() => ({
  left: '7px',
  top: '14px',
  width: '242px',
  height: '338px',
  zIndex: '5',
  '--rarity-color': pColor,
}))

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

const lowerTabStyle = computed(() => ({
  left: imageSuit.value.background.width / 2 - imageSuit.value.lowerTab.width / 2 + 'px',
  top: '314px',
  // top: 14 + bgSize.height - 42 + 'px',
  // zIndex: '40',
}))

const rankCompleteLineStyle = computed(() => ({
  top: '363px',
  left: 0,
  zIndex: '6',
}))

const rankStartsStyle = computed(() => ({
  top: '353px',
  left: 128 - (12 * fusionLimit.value) / 2 + 'px',
  zIndex: 6,
}))

const fusionLimit = computed(() => {
  const cm = getModClass(props.mod)
  if (cm === 'Omega Unknown') {
    return 5
  }
  if (cm === 'Omega Opened') {
    return 8
  }
  // return props.mod.fusionLimit
  return 5
})

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
  () => {
    if (props.maxRank) {
      currentRank.value = fusionLimit.value
    }
  },
  { immediate: true },
)

const rankMaxed = computed(() => {
  return fusionLimit.value === currentRank.value
})
function increaseRank() {
  if (currentRank.value < fusionLimit.value) {
    currentRank.value++
  }
}

function reduceRank() {
  if (currentRank.value > 0) {
    currentRank.value--
  }
}

const effect = computed(() => {
  return ['激活或装备这个裂隙振幅晶体来揭开它的挑战']
})

const drain = computed(() => {
  // if (props.mod.)
  return Math.abs(props.mod.baseDrain) + currentRank.value
})
</script>
<style scoped>
img {
  max-width: none;
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
