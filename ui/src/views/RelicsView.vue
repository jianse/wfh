<script setup lang="ts">
import { computed, ref } from 'vue'
import RarityIconCommon from '@/assets/icons/RarityIconCommon.png'
import RarityIconUncommon from '@/assets/icons/RarityIconUncommon.png'
import RarityIconRare from '@/assets/icons/RarityIconRare.png'
import { useORelics } from '@/util/localDB'
import type { RelicReward } from '@/util/localDB'
import { useTransStore } from '@/stores/translations'
import { useCatalogStore } from '@/stores/catalog'
import { useAuthStore } from '@/stores/auth'
import { useUiStore } from '@/stores/ui'
import RelicTransDialog from '@/components/dialogs/RelicTransDialog.vue'

const catalog = useCatalogStore()
const relics = useORelics()
const { wf_t } = useTransStore()
const auth = useAuthStore()
const ui = useUiStore()

const IMG = 'https://content.warframe.com/PublicExport'

interface RewardRow extends RelicReward {
  name_t?: string
}
interface RelicRow {
  uniqueName: string
  name: string
  img: string
  tier: string
  rewards: RewardRow[]
}

const tier = ref('Any')
const TABS = [
  { cat: 'Any', label: '全部' },
  { cat: 'T1', label: '古纪' },
  { cat: 'T2', label: '前纪' },
  { cat: 'T3', label: '中纪' },
  { cat: 'T4', label: '后纪' },
  { cat: 'T5', label: '安魂' }
]

function suffix(name: string): string {
  if (name.endsWith('Bronze')) return ''
  if (name.endsWith('Silver')) return '[优良]'
  if (name.endsWith('Gold')) return '[无暇]'
  if (name.endsWith('Platinum')) return '[光辉]'
  return ''
}

function rarityIcon(rarity: string): string {
  switch (rarity) {
    case 'COMMON':
      return RarityIconCommon
    case 'UNCOMMON':
      return RarityIconUncommon
    case 'RARE':
      return RarityIconRare
  }
  return ''
}

const mapped = computed<RelicRow[]>(() =>
  relics.value.map((r): RelicRow => ({
    uniqueName: r.uniqueName,
    name: (wf_t(r.uniqueName, 'name') || r.name) + ' ' + suffix(r.uniqueName),
    img: IMG + r.textureLocation,
    tier: r.tier,
    rewards: (r.relicRewards || []).map((reward): RewardRow => ({
      ...reward,
      name_t: wf_t(reward.uniqueName, 'name') || reward.name
    }))
  }))
)

const list = computed(() => {
  const q = catalog.relicQuery.trim().toLowerCase()
  return mapped.value.filter((r) => {
    if (tier.value !== 'Any' && r.tier !== tier.value) return false
    if (q && !(r.name + r.rewards.map((x) => x.name_t).join(' ')).toLowerCase().includes(q)) return false
    return true
  })
})

const totalCount = computed(() => mapped.value.length)
</script>

<template>
  <section class="view active" id="view-relics" data-od-id="relics">
    <div class="page-head">
      <p class="eyebrow">数据库 · 遗物</p>
      <h1 class="h2">遗物</h1>
      <p class="lead">通过虚空裂缝开启，获得 Prime 部件，来自本地缓存数据。</p>
    </div>

    <div class="tabs" id="relics-tabs" role="tablist" aria-label="遗物纪元">
      <button
        v-for="t in TABS"
        :key="t.cat"
        class="tab"
        :class="{ active: tier === t.cat }"
        role="tab"
        :aria-selected="tier === t.cat ? 'true' : 'false'"
        @click="tier = t.cat"
      >
        {{ t.label }}
      </button>
    </div>

    <div class="page-tools">
      <input v-model="catalog.relicQuery" class="input" id="relics-search" type="text" placeholder="搜索遗物或奖励…" />
      <div class="spacer"></div>
      <span class="meta" id="relics-count">{{ list.length }} / {{ totalCount }}</span>
    </div>

    <div class="db-grid relics" id="relics-grid">
      <article v-for="r in list" :key="r.uniqueName" class="db-card relics" :data-tier="r.tier">
        <div class="db-media icon">
          <img :src="r.img" :alt="r.name" loading="lazy" />
          <button
            v-if="auth.canAdmin"
            class="icon-btn relic-edit"
            title="编辑译文"
            @click="ui.openDialog(RelicTransDialog, { relicName: r.name, rewards: r.rewards })"
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><path d="M4 20h4L19 9l-4-4L4 16v4z" /><path d="M14 6l4 4" /></svg>
          </button>
        </div>
        <div class="db-body">
          <div class="db-name"><span class="h3">{{ r.name }}</span></div>
          <div class="db-meta"><span class="db-rarity">{{ r.tier }} 纪元</span></div>
          <ul class="relic-rewards">
            <li v-for="reward in r.rewards" :key="reward.rewardName">
              <img v-if="rarityIcon(reward.rarity)" class="relic-rarity" :src="rarityIcon(reward.rarity)" :alt="reward.rarity" />
              <span class="relic-rname">
                <span v-if="reward.itemCount > 1">{{ reward.itemCount }} × </span>{{ reward.name_t || reward.name }}
              </span>
              <span v-if="reward.primeSellingPrice != null" class="relic-price">{{ reward.primeSellingPrice }}</span>
            </li>
          </ul>
        </div>
      </article>
      <p v-if="!list.length" class="db-empty">没有匹配的遗物，或本地数据尚未同步。</p>
    </div>
  </section>
</template>

<style scoped>
.relic-rewards {
  list-style: none;
  margin: 0.5rem 0 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
  font-size: 12px;
}
.relic-rewards li {
  display: flex;
  align-items: center;
  gap: 6px;
}
.relic-rarity {
  height: 14px;
  width: 14px;
  flex: none;
}
.relic-rname {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.relic-price {
  font-weight: 700;
}
.db-media {
  position: relative;
}
.relic-edit {
  position: absolute;
  top: 6px;
  right: 6px;
}
</style>
