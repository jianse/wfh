<script setup lang="ts">
import { computed, ref } from 'vue'
import DataPager from '@/components/DataPager.vue'
import DataTable from '@/components/DataTable.vue'
import DataTableColumn from '@/components/DataTableColumn.vue'
import LangEditDialog from '@/components/dialogs/LangEditDialog.vue'
import LangUploadDialog from '@/components/dialogs/LangUploadDialog.vue'
import { useUiStore } from '@/stores/ui'
import { fmt } from '@/lib/utils'

const ui = useUiStore()

interface LangLocale {
  code: string
  label: string
}
interface LangRow {
  k: string
  zh: string
  en: string
  ja: string
  [key: string]: string | undefined
}
interface LangEntry {
  key: string
  value: string
  overridden: boolean
}
type LangDB = Record<string, Record<string, string | undefined>>

const LANG_LOCALES: LangLocale[] = [
  { code: 'zh', label: '简体中文' },
  { code: 'en', label: 'English' },
  { code: 'ja', label: '日本語' },
  { code: 'ko', label: '한국어' },
  { code: 'de', label: 'Deutsch' }
]
const LANG_DEFAULT_LOCALE = 'zh'

const LANG_SEED: LangRow[] = [
  { k: 'ui.nav.home', zh: '主页', en: 'Home', ja: 'ホーム' },
  { k: 'ui.nav.warframes', zh: '战甲', en: 'Warframes', ja: 'ウォーフレーム' },
  { k: 'ui.nav.weapons', zh: '武器', en: 'Weapons', ja: '武器' },
  { k: 'ui.nav.mods', zh: '振幅晶体', en: 'Mods', ja: 'モッド' },
  { k: 'ui.nav.arcane', zh: '赋能', en: 'Arcanes', ja: 'アルケイン' },
  { k: 'ui.nav.relics', zh: '遗物', en: 'Relics', ja: 'レリック' },
  { k: 'ui.nav.resources', zh: '资源', en: 'Resources', ja: '資源' },
  { k: 'ui.nav.recipes', zh: '配方', en: 'Recipes', ja: 'レシピ' },
  { k: 'ui.nav.worldstate', zh: '事件追踪', en: 'World State', ja: 'ワールドステート' },
  { k: 'ui.nav.about', zh: '关于', en: 'About', ja: '情報' },
  { k: 'ui.nav.login', zh: '登录', en: 'Sign In', ja: 'ログイン' },
  { k: 'ui.nav.settings', zh: '设置', en: 'Settings', ja: '設定' },
  { k: 'ui.common.search', zh: '搜索', en: 'Search', ja: '検索' },
  { k: 'ui.common.refresh', zh: '刷新', en: 'Refresh', ja: '更新' },
  { k: 'ui.common.save', zh: '保存', en: 'Save', ja: '保存' },
  { k: 'ui.common.cancel', zh: '取消', en: 'Cancel', ja: 'キャンセル' },
  { k: 'ui.common.confirm', zh: '确定', en: 'Confirm', ja: '確定' },
  { k: 'ui.common.close', zh: '关闭', en: 'Close', ja: '閉じる' },
  { k: 'ui.common.edit', zh: '编辑', en: 'Edit', ja: '編集' },
  { k: 'ui.common.delete', zh: '删除', en: 'Delete', ja: '削除' },
  { k: 'ui.common.reset', zh: '重置', en: 'Reset', ja: 'リセット' },
  { k: 'ui.common.loading', zh: '加载中…', en: 'Loading…', ja: '読み込み中…' },
  { k: 'ui.common.empty', zh: '暂无数据', en: 'No data', ja: 'データなし' },
  { k: 'ui.common.copy', zh: '复制', en: 'Copy', ja: 'コピー' },
  { k: 'ui.common.export', zh: '导出', en: 'Export', ja: 'エクスポート' },
  { k: 'ui.common.upload', zh: '上传', en: 'Upload', ja: 'アップロード' },
  { k: 'ui.search.placeholder', zh: '搜索战甲、武器、赋能…', en: 'Search warframes, weapons, arcanes…', ja: 'ウォーフレーム、武器、アルケインを検索…' },
  { k: 'ui.search.enter', zh: '输入关键词开始搜索', en: 'Type to start searching', ja: 'キーワードを入力' },
  { k: 'ui.search.noResult', zh: '未找到匹配结果', en: 'No matching results', ja: '一致する結果はありません' },
  { k: 'db.warframe.hp', zh: '生命值', en: 'Health', ja: '体力' },
  { k: 'db.warframe.shield', zh: '护盾', en: 'Shield', ja: 'シールド' },
  { k: 'db.warframe.armor', zh: '护甲', en: 'Armor', ja: 'アーマー' },
  { k: 'db.warframe.energy', zh: '能量', en: 'Energy', ja: 'エネルギー' },
  { k: 'db.warframe.mastery', zh: '段位需求', en: 'Mastery Rank', ja: 'マスタリーランク' },
  { k: 'db.warframe.category', zh: '类型', en: 'Category', ja: 'カテゴリ' },
  { k: 'db.weapon.primary', zh: '主武器', en: 'Primary', ja: 'プライマリ' },
  { k: 'db.weapon.secondary', zh: '副武器', en: 'Secondary', ja: 'セカンダリ' },
  { k: 'db.weapon.melee', zh: '近战', en: 'Melee', ja: '近接' },
  { k: 'db.weapon.amp', zh: '增幅器', en: 'Amp', ja: 'アンプ' },
  { k: 'db.weapon.archgun', zh: '曲翼武器', en: 'Arch-Gun', ja: 'アーチガン' },
  { k: 'db.weapon.archmelee', zh: '曲翼近战', en: 'Arch-Melee', ja: 'アーチ近接' },
  { k: 'db.weapon.sentinel', zh: '守护武器', en: 'Sentinel Weapon', ja: 'センチネル武器' },
  { k: 'db.weapon.kitgun', zh: '组合枪', en: 'Kitgun', ja: 'キットガン' },
  { k: 'db.mod.type', zh: '类型', en: 'Type', ja: 'タイプ' },
  { k: 'db.mod.rarity', zh: '稀有度', en: 'Rarity', ja: 'レアリティ' },
  { k: 'db.mod.rarity.common', zh: '普通', en: 'Common', ja: 'コモン' },
  { k: 'db.mod.rarity.uncommon', zh: '罕见', en: 'Uncommon', ja: 'アンコモン' },
  { k: 'db.mod.rarity.rare', zh: '稀有', en: 'Rare', ja: 'レア' },
  { k: 'db.mod.rarity.legendary', zh: '传说', en: 'Legendary', ja: 'レジェンダリー' },
  { k: 'db.mod.drain', zh: '消耗', en: 'Drain', ja: '消費' },
  { k: 'db.mod.maxRank', zh: '最大等级', en: 'Max Rank', ja: '最大ランク' },
  { k: 'db.arcane.name', zh: '赋能名称', en: 'Arcane Name', ja: 'アルケイン名' },
  { k: 'db.arcane.rank', zh: '等级', en: 'Rank', ja: 'ランク' },
  { k: 'db.arcane.effect', zh: '效果', en: 'Effect', ja: '効果' },
  { k: 'db.arcane.source', zh: '获取来源', en: 'Source', ja: '入手先' },
  { k: 'db.relic.lith', zh: '古痕', en: 'Lith', ja: 'リト' },
  { k: 'db.relic.meso', zh: '中纪', en: 'Meso', ja: 'メソ' },
  { k: 'db.relic.neo', zh: '后纪', en: 'Neo', ja: 'ネオ' },
  { k: 'db.relic.axi', zh: '前纪', en: 'Axi', ja: 'アクシ' },
  { k: 'db.relic.requiem', zh: '安魂', en: 'Requiem', ja: 'レクイエム' },
  { k: 'worldstate.voidTrader', zh: '虚空商人', en: 'Void Trader', ja: 'ヴォイドトレーダー' },
  { k: 'worldstate.sortie', zh: '突击任务', en: 'Sortie', ja: 'ソーティ' },
  { k: 'worldstate.cycle', zh: '周期循环', en: 'Cycle', ja: 'サイクル' },
  { k: 'worldstate.fissure', zh: '虚空裂缝', en: 'Void Fissure', ja: 'ヴォイドリフト' },
  { k: 'worldstate.invasion', zh: '入侵', en: 'Invasion', ja: '侵攻' },
  { k: 'worldstate.alert', zh: '警报', en: 'Alert', ja: 'アラート' },
  { k: 'worldstate.steelPath', zh: '钢铁之路', en: 'Steel Path', ja: 'スチールパス' },
  { k: 'worldstate.nightwave', zh: '夜灵潮', en: 'Nightwave', ja: 'ナイトウェーブ' },
  { k: 'worldstate.dailyDeal', zh: '每日特惠', en: 'Daily Deal', ja: 'デイリーデール' },
  { k: 'worldstate.baro', zh: '巴洛·基·提尔', en: "Baro Ki'Teer", ja: 'バロ・キ・ティア' },
  { k: 'admin.users.title', zh: '用户管理', en: 'User Management', ja: 'ユーザー管理' },
  { k: 'admin.sync.title', zh: '同步记录', en: 'Sync Records', ja: '同期記録' },
  { k: 'admin.langs.title', zh: '语言管理', en: 'Language Management', ja: '言語管理' },
  { k: 'admin.langs.key', zh: '键', en: 'Key', ja: 'キー' },
  { k: 'admin.langs.value', zh: '译文', en: 'Translation', ja: '訳文' },
  { k: 'admin.langs.upload', zh: '上传 JSON 覆写', en: 'Upload JSON Override', ja: 'JSONをアップロード' },
  { k: 'admin.langs.overridden', zh: '已覆写', en: 'Overridden', ja: '上書き済み' },
  { k: 'admin.langs.missing', zh: '缺失', en: 'Missing', ja: '未翻訳' }
]

const LANG_PARTIAL: Record<string, Record<string, string> | undefined> = {
  ko: {
    'ui.nav.home': '홈', 'ui.nav.warframes': '워프레임', 'ui.nav.weapons': '무기', 'ui.nav.mods': '모드',
    'ui.nav.arcane': '아케인', 'ui.nav.relics': '유물', 'ui.nav.resources': '자원', 'ui.nav.about': '정보',
    'ui.common.search': '검색', 'ui.common.save': '저장', 'ui.common.cancel': '취소', 'ui.common.confirm': '확인',
    'db.warframe.hp': '체력', 'db.warframe.shield': '실드', 'worldstate.sortie': '소티', 'worldstate.cycle': '주기'
  },
  de: {
    'ui.nav.home': 'Startseite', 'ui.nav.warframes': 'Warframes', 'ui.nav.weapons': 'Waffen', 'ui.nav.mods': 'Mods',
    'ui.nav.arcane': 'Arkana', 'ui.nav.relics': 'Relikte', 'ui.nav.resources': 'Ressourcen', 'ui.nav.about': 'Über',
    'ui.common.search': 'Suchen', 'ui.common.save': 'Speichern', 'ui.common.cancel': 'Abbrechen', 'ui.common.confirm': 'Bestätigen',
    'db.warframe.hp': 'Leben', 'db.warframe.shield': 'Schild', 'worldstate.sortie': 'Sortie', 'worldstate.cycle': 'Zyklus'
  }
}

function buildLangDB(): LangDB {
  const db: LangDB = {}
  LANG_LOCALES.forEach((loc) => {
    const map: Record<string, string | undefined> = {}
    LANG_SEED.forEach((row) => {
      let v = row[loc.code]
      const partial = LANG_PARTIAL[loc.code]
      if (v == null && partial) v = partial[row.k]
      if (v) map[row.k] = v
    })
    db[loc.code] = map
  })
  return db
}

const LANG_DB = buildLangDB()

const overrides = ref<Record<string, Record<string, string | undefined>>>({})
const active = ref(LANG_DEFAULT_LOCALE)
const page = ref(1)
const size = ref(20)
const query = ref('')
const loading = ref(false)

const locLabel = (code: string): string => LANG_LOCALES.find((l) => l.code === code)?.label ?? code

function langAll(locale: string): LangEntry[] {
  const base = LANG_DB[locale] || {}
  const over = overrides.value[locale] || {}
  const keys = LANG_SEED.map((r) => r.k)
  Object.keys(over).forEach((k) => {
    if (keys.indexOf(k) < 0) keys.push(k)
  })
  return keys.map((key) => {
    const v = over[key] != null ? over[key] : base[key] != null ? base[key] : ''
    return { key, value: v ?? '', overridden: over[key] != null }
  })
}

const allRows = computed(() => langAll(active.value))
const filtered = computed(() => {
  const q = query.value.trim().toLowerCase()
  return q ? allRows.value.filter((r) => r.key.toLowerCase().includes(q) || String(r.value).toLowerCase().includes(q)) : allRows.value
})
const total = computed(() => filtered.value.length)
const pageRows = computed(() => filtered.value.slice((page.value - 1) * size.value, (page.value - 1) * size.value + size.value))

const stats = computed(() => {
  const rows = allRows.value
  const done = rows.filter((r) => r.value).length
  return { total: rows.length, done, missing: rows.length - done, over: rows.filter((r) => r.overridden).length }
})

function goPage(p: number) {
  page.value = p
}

function onSizeChange(s: number) {
  size.value = s
  page.value = 1
}

function switchLocale(code: string) {
  active.value = code
  page.value = 1
}

/* ── 单条编辑 ── */
function openLangEdit(key: string) {
  const cur = langAll(active.value).find((r) => r.key === key)
  ui.openDialog(LangEditDialog, {
    locale: locLabel(active.value),
    keyName: key,
    initial: cur && cur.value ? cur.value : '',
    onSave: (value: string) => {
      if (!overrides.value[active.value]) overrides.value[active.value] = {}
      overrides.value[active.value][key] = value
      ui.toast('ok', '已保存', `「${key}」已更新。`)
    }
  })
}

function restoreLang(key: string) {
  ui.askConfirm('恢复原始译文', `确定撤销「${key}」的覆写，恢复为后台原始译文吗？`, () => {
    if (overrides.value[active.value]) delete overrides.value[active.value][key]
    ui.toast('info', '已恢复', `「${key}」已恢复为原始译文。`)
  })
}

/* ── JSON 上传 ── */
function openLangUpload() {
  const current: Record<string, string> = {}
  langAll(active.value).forEach((r) => {
    current[r.key] = r.value
  })
  ui.openDialog(LangUploadDialog, {
    locale: locLabel(active.value),
    current,
    onApply: (pairs: [string, string][]) => {
      if (!overrides.value[active.value]) overrides.value[active.value] = {}
      pairs.forEach(([k, v]) => {
        overrides.value[active.value][k] = v
      })
      page.value = 1
      ui.toast('ok', '覆写完成', `已向「${locLabel(active.value)}」写入 ${pairs.length} 条词条。`)
    }
  })
}

function exportJson() {
  const out: Record<string, string> = {}
  langAll(active.value).forEach((r) => {
    if (r.value) out[r.key] = r.value
  })
  const blob = new Blob([JSON.stringify(out, null, 2)], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `wfh-${active.value}.json`
  document.body.appendChild(a)
  a.click()
  a.remove()
  setTimeout(() => URL.revokeObjectURL(url), 800)
  ui.toast('ok', '已导出', `${locLabel(active.value)} 的 ${Object.keys(out).length} 条词条已导出。`)
}

function refresh() {
  ui.toast('info', '已刷新', '语言数据已重新载入。')
}
</script>

<template>
  <section class="view active" id="view-languages" data-od-id="languages">
    <div class="page-head">
      <p class="eyebrow">管理后台 · 语言管理</p>
      <h1 class="h2">语言管理</h1>
      <p class="lead">从后台分页读取各语言的键值对，支持搜索、单条编辑，或上传 JSON 覆写当前语言。</p>
    </div>

    <div class="stat-grid" id="lang-stats">
      <div class="stat"><div class="stat-num">{{ fmt(stats.total) }}</div><div class="stat-label">词条总数</div></div>
      <div class="stat"><div class="stat-num ok">{{ fmt(stats.done) }}</div><div class="stat-label">已翻译</div></div>
      <div class="stat"><div class="stat-num bad">{{ fmt(stats.missing) }}</div><div class="stat-label">缺失</div></div>
      <div class="stat"><div class="stat-num">{{ fmt(stats.over) }}</div><div class="stat-label">已覆写</div></div>
    </div>

    <div class="tabs" id="lang-locale-tabs" role="tablist" aria-label="选择语言" data-od-id="lang-locale-tabs">
      <button
        v-for="l in LANG_LOCALES"
        :key="l.code"
        class="tab"
        :class="{ active: l.code === active }"
        role="tab"
        :aria-selected="l.code === active"
        @click="switchLocale(l.code)"
      >
        {{ l.label }}<span class="tab-count">{{ langAll(l.code).filter((r) => r.value).length }}/{{ langAll(l.code).length }}</span>
      </button>
    </div>

    <div class="page-tools">
      <input
        v-model="query"
        class="input"
        id="lang-search"
        type="text"
        placeholder="搜索键名或译文…"
        @input="page = 1"
      />
      <button class="btn btn-secondary" id="lang-refresh" @click="refresh">刷新</button>
      <button class="btn btn-secondary" id="lang-export" @click="exportJson">导出 JSON</button>
      <button class="btn btn-primary" id="lang-upload" data-od-id="lang-upload" @click="openLangUpload">上传 JSON 覆写</button>
      <div class="spacer"></div>
      <span class="meta" id="lang-total">共 {{ total }} 条 · {{ locLabel(active) }}</span>
    </div>

    <DataTable
      :rows="pageRows"
      row-key="key"
      :loading="loading"
      loading-text="正在从后台加载语言数据…"
      empty-text="没有匹配的词条。"
    >
      <DataTableColumn prop="key" label="键（key）" width="38%" cell-class="lang-key">
        <template #default="{ row }">{{ row.key }}</template>
      </DataTableColumn>
      <DataTableColumn prop="value" label="译文（value）">
        <template #default="{ row }">
          <span class="lang-val" :class="{ over: row.overridden, missing: !row.value }">{{
            row.value ? row.value : '未翻译'
          }}</span>
        </template>
      </DataTableColumn>
      <DataTableColumn label="状态" width="90px">
        <template #default="{ row }">
          <span v-if="row.overridden" class="lang-badge over">已覆写</span>
          <span v-else-if="row.value" class="lang-badge">已翻译</span>
          <span v-else class="lang-badge missing">缺失</span>
        </template>
      </DataTableColumn>
      <DataTableColumn label="操作" width="130px">
        <template #default="{ row }">
          <button class="btn btn-ghost" @click="openLangEdit(row.key)">编辑</button>
          <button v-if="row.overridden" class="btn btn-ghost" @click="restoreLang(row.key)">恢复</button>
        </template>
      </DataTableColumn>
    </DataTable>
    <DataPager
      :page="page"
      :total="total"
      :size="size"
      :page-sizes="[20, 50, 100]"
      :disabled="loading"
      @size-change="onSizeChange"
      @go="goPage"
    />
  </section>
</template>
