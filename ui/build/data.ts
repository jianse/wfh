import fs from 'node:fs'
import lzma from 'lzma'
import { createHash } from 'node:crypto'

const start_time = Date.now()

const output_dir = 'public/data'
const cache_dir = 'cache'
// Check if the output directory exists, if not create it

// always clear output directory
fs.rmSync(output_dir, { recursive: true, force: true })
fs.mkdirSync(output_dir, { recursive: true })

// Check if the cache directory exists, if not create it
if (!fs.existsSync(cache_dir)) {
  fs.mkdirSync(cache_dir, { recursive: true })
}

/**
 * load endpoint cache if it exist.
 * or create it if not exist.
 */
function load_or_create_endpoint_cache() {
  // Check if the cache file exists
  if (fs.existsSync(`${cache_dir}/endpointCache2.json`)) {
    console.log('Loading endpoint cache')
    // TODO: load the cache from the file
    const json = fs.readFileSync(`${cache_dir}/endpointCache2.json`, 'utf-8')
    const obj = JSON.parse(json)
    const endpointCache = new Map()
    for (const [key, value] of Object.entries(obj)) {
      endpointCache.set(key, value)
    }
    console.log('Endpoint cache loaded')
    return endpointCache
  } else {
    console.log('create empty endpoint cache')
    return new Map()
  }
}

/**
 * save endpoint cache
 * @param {Map<string,string>} cache endpoint cache
 */
function save_endpoint_cache(cache: Map<string, string>) {
  const json = Object.fromEntries(cache)
  fs.writeFileSync(`${cache_dir}/endpointCache2.json`, JSON.stringify(json))
  console.log('Endpoint cache saved')
}

const cache = load_or_create_endpoint_cache()

async function fetch_index() {
  const origin = 'https://origin.warframe.com/PublicExport/index_zh.txt.lzma'
  const res = await fetch(origin)
  const raw = await res.bytes()

  const decompressed = lzma.decompress(raw)
  cache.set(origin, decompressed)
  return decompressed
}

function md5_hash(message: string) {
  const hash = createHash('md5')
  hash.update(message)
  return hash.digest('base64url')
}

async function get_or(
  cache: Map<string, string>,
  key: string,
  cb: (key: string) => Promise<string>,
) {
  const value = cache.get(key)
  if (value === undefined) {
    console.log(`cache miss for: ${key} . using callback to fetch`)
    const value = await cb(key)
    cache.set(key, value)
    save_endpoint_cache(cache)
    return value
  } else {
    // console.log(`cache hit for: ${key}`)
    return value
  }
}

const g_index = await fetch_index()
// console.log(index)

const endpoints = g_index.split(/\r?\n/g)
console.log(endpoints)

const m = new Map<string, object>()

for (const it of endpoints) {
  const url = `https://content.warframe.com/PublicExport/Manifest/${it}`
  const cont = await get_or(cache, url, async () => {
    const resp = await fetch(url)
    return await resp.text()
  })

  const contentJson = JSON.parse(cont)

  const cat = it.replace(/Export([a-zA-Z]+)(_\w{2})?\.json(.*)/, '$1')
  console.log(cat)
  m.set(cat, contentJson)
}
class ObservableMap<K, V> extends Map<K, V> {
  get(key: K, warn: boolean = true) {
    const v = super.get(key)
    if (warn && v === undefined) {
      console.log('can not find key:', key)
    }
    return v
  }
}

interface Manifest {
  Manifest: {
    uniqueName: string
    textureLocation: string
  }[]
}

// manifest
const mf_kv = new ObservableMap<string, string>()
const manifest = m.get('Manifest') as Manifest

manifest.Manifest.forEach((element) => {
  mf_kv.set(element.uniqueName, element.textureLocation)
})

const reverse_map = new ObservableMap<string, ReverseIndex>()

class ReverseIndex {
  constructor(
    readonly category: string,
    readonly prop: string,
    readonly index: number,
  ) {}
}

// split relics and arcane
{
  const relicArcane = m.get('RelicArcane') as ExportRelicArcane

  const relic: Relic[] = []
  const arcane: Arcane[] = []
  relicArcane.ExportRelicArcane.forEach((element) => {
    if (
      [
        '/Lotus/Upgrades/CosmeticEnhancers/Offensive/AbilityStrengthOnSummonAttackSub',
        '/Lotus/Upgrades/CosmeticEnhancers/Offensive/PermaComboOnFinisherSub',
        '/Lotus/Upgrades/CosmeticEnhancers/Utility/NoCostCastChanceAbility1Listener',
        '/Lotus/Upgrades/CosmeticEnhancers/Utility/NoCostCastChanceAbility2Listener',
        '/Lotus/Upgrades/CosmeticEnhancers/Utility/NoCostCastChanceAbility3Listener',
        '/Lotus/Upgrades/CosmeticEnhancers/Utility/NoCostCastChanceAbility4Listener',
      ].includes(element.uniqueName)
    ) {
      // these are safe to ignore
    } else if ('relicRewards' in element) {
      if (element.uniqueName.endsWith('Bronze')) {
        // only add bronze relics
        relic.push(element)
      }
    } else if ('levelStats' in element) {
      arcane.push(element)
    } else {
      console.warn('unknown item :', element)
    }
  })

  m.set('Relics', { ExportRelics: relic })
  m.set('Arcane', { ExportArcane: arcane })
  m.delete('RelicArcane')
}

// resources
interface ResourceItem {
  uniqueName: string
  textureLocation?: string
}
interface Resources {
  ExportResources: ResourceItem[]
}

const resources = m.get('Resources') as Resources

resources.ExportResources.forEach((element, index) => {
  reverse_map.set(element.uniqueName, new ReverseIndex('Resources', 'ExportResources', index))
  element.textureLocation = mf_kv.get(element.uniqueName)
})

// gear
interface Gear {
  ExportGear: {
    uniqueName: string
    name: string
    description: string
    codexSecret: boolean
    parentName: string
    textureLocation?: string
  }[]
}

const gear = m.get('Gear') as Gear

gear.ExportGear.forEach((element, index) => {
  reverse_map.set(element.uniqueName, new ReverseIndex('Gear', 'ExportGear', index))
  element.textureLocation = mf_kv.get(element.uniqueName)
})

// customs
interface Customs {
  ExportCustoms: {
    uniqueName: string
    name: string
    codexSecret: boolean
    description: string
    textureLocation?: string
  }[]
}

const customs = m.get('Customs') as Customs

customs.ExportCustoms.forEach((element, index) => {
  reverse_map.set(element.uniqueName, new ReverseIndex('Customs', 'ExportCustoms', index))
  element.textureLocation = mf_kv.get(element.uniqueName)
})

// warframes
interface WarframeItem {
  uniqueName: string
  textureLocation?: string
}
interface ExportAbilityItem {
  uniqueName: string
}

interface Warframes {
  ExportWarframes: WarframeItem[]
  ExportAbilities: ExportAbilityItem[]
}

const warframes = m.get('Warframes') as Warframes

warframes.ExportWarframes.forEach((element, index) => {
  reverse_map.set(element.uniqueName, new ReverseIndex('Warframes', 'ExportWarframes', index))
  element.textureLocation = mf_kv.get(element.uniqueName)
})

// TODO: ExportAbilities in warframes

// weapons

interface Weapons {
  ExportWeapons: {
    uniqueName: string
    textureLocation?: string
  }[]
  ExportRailjackWeapons: {
    uniqueName: string
    textureLocation?: string
  }[]
}
const weapons = m.get('Weapons') as Weapons

weapons.ExportWeapons.forEach((element, index) => {
  reverse_map.set(element.uniqueName, new ReverseIndex('Weapons', 'ExportWeapons', index))
  element.textureLocation = mf_kv.get(element.uniqueName)
})

weapons.ExportRailjackWeapons.forEach((element, index) => {
  reverse_map.set(element.uniqueName, new ReverseIndex('Weapons', 'ExportRailjackWeapons', index))
  element.textureLocation = mf_kv.get(element.uniqueName)
})

// upgrades
interface Upgrades {
  ExportUpgrades: {
    uniqueName: string
    name: string
    textureLocation?: string
  }[]
}
const upgrades = m.get('Upgrades') as Upgrades

upgrades.ExportUpgrades.forEach((element, index) => {
  reverse_map.set(element.uniqueName, new ReverseIndex('Upgrades', 'ExportUpgrades', index))
  element.textureLocation = mf_kv.get(element.uniqueName)
})

// keys
interface ExportKeys {
  ExportKeys: {
    uniqueName: string
    name: string
    description: string
    parentName: string
    codexSecret: boolean
    textureLocation?: string
  }[]
}
const keys = m.get('Keys') as ExportKeys

keys.ExportKeys.forEach((element, index) => {
  reverse_map.set(element.uniqueName, new ReverseIndex('Keys', 'ExportKeys', index))
  element.textureLocation = mf_kv.get(element.uniqueName)
})

// drones
interface ExportDrones {
  ExportDrones: {
    uniqueName: string
    name: string
    description: string
    binCount: number
    binCapacity: number
    fillRate: number
    durability: number
    repairRate: number
    codexSecret: boolean
    capacityMultiplier: number[]
    specialities: number[]
    textureLocation?: string
  }[]
}

const drones = m.get('Drones') as ExportDrones

drones.ExportDrones.forEach((element, index) => {
  reverse_map.set(element.uniqueName, new ReverseIndex('Drones', 'ExportDrones', index))
  element.textureLocation = mf_kv.get(element.uniqueName)
})

// sentinels
interface ExportSentinels {
  ExportSentinels: {
    uniqueName: string
    name: string
    health: number
    shield: number
    armor: number
    stamina: number
    power: number
    codexSecret: boolean
    excludeFromCodex: boolean
    description: string
    productCategory: 'SpecialItems'
    textureLocation?: string
  }[]
}
const sentinels = m.get('Sentinels') as ExportSentinels

sentinels.ExportSentinels.forEach((element, index) => {
  reverse_map.set(element.uniqueName, new ReverseIndex('Sentinels', 'ExportSentinels', index))
  element.textureLocation = mf_kv.get(element.uniqueName)
})

import arcaneExt from './arcane-ext.ts'

const arcane = m.get('Arcane') as ExportArcane
arcane.ExportArcane.forEach((element, index) => {
  reverse_map.set(element.uniqueName, new ReverseIndex('Arcane', 'ExportArcane', index))
  element.textureLocation = mf_kv.get(element.uniqueName)

  const ext = arcaneExt.find((ext) => ext.uniqueName === element.uniqueName)
  if (ext) {
    element.inGame = ext.inGame
    if (ext.inGame) {
      element.slot = ext?.slot
    } else {
      element.slot = 'Unknown'
    }
  } else {
    element.inGame = true
    element.slot = 'Unknown'
  }
})

// recipes
interface Ingredient {
  ItemType: string
  ItemCount: number
  ProductCategory: 'Consumables' | 'MiscItems'
  name?: string
  textureLocation?: string
}

interface Recipes {
  ExportRecipes: {
    uniqueName: string
    resultType: string
    buildPrice: number
    buildTime: number
    skipBuildTimePrice: number
    consumeOnUse: boolean
    num: number
    codexSecret: boolean
    ingredients: Ingredient[]
    // this is always be a mission requirements
    secretIngredients: Ingredient[]
    name?: string
    description?: string
    textureLocation?: string
  }[]
}
const recipes = m.get('Recipes') as Recipes

recipes.ExportRecipes.forEach((element, index) => {
  reverse_map.set(element.uniqueName, new ReverseIndex('Recipes', 'ExportRecipes', index))

  const r_idx = reverse_map.get(element.resultType, false)
  if (r_idx === undefined) {
    console.warn('can not find reverse index for: ', element.resultType)
  } else {
    const r = (m.get(r_idx.category) as object)[r_idx.prop][r_idx.index]
    // console.log(r)
    element.name = r.name + ' 设计图'
    element.description = r.description
  }

  element.textureLocation = mf_kv.get(element.uniqueName)
  element.ingredients.forEach((i) => {
    const idx = reverse_map.get(i.ItemType, false)
    if (idx === undefined) {
      console.warn('can not find reverse index for: ', i.ItemType)
    } else {
      const item = (m.get(idx.category) as object)[idx.prop][idx.index]
      i.name = item.name
      i.textureLocation = item.textureLocation
    }
  })
})

// relics and arcane
interface Relic {
  uniqueName: string
  name: string
  codexSecret: boolean
  description: string
  relicRewards: {
    rewardName: string
    rarity: 'RARE' | 'UNCOMMON' | 'COMMON'
    tier: number
    itemCount: number
    name?: string
    primeSellingPrice?: number
    textureLocation?: string
    uniqueName?: string
  }[]
  tier: string
  prefixChar: string
  serial: number
  textureLocation?: string
}

interface Arcane {
  uniqueName: string
  name: string
  codexSecret: boolean
  rarity?: 'RARE' | 'UNCOMMON' | 'COMMON' | 'LEGENDARY'
  levelStats: { stats: string[] }[]
  textureLocation?: string
  inGame?: boolean
  slot?: string
}

interface ExportRelicArcane {
  ExportRelicArcane: (Arcane | Relic)[]
}

// arcane
interface ExportArcane {
  ExportArcane: Arcane[]
}

// relics
interface ExportRelics {
  ExportRelics: Relic[]
}
function rarity2Num(rarity: string): number {
  switch (rarity) {
    case 'COMMON':
      return 0
    case 'UNCOMMON':
      return 1
    case 'RARE':
      return 2
  }
  return 0
}
function roman2num(roman: string): number {
  switch (roman) {
    case 'I':
      return 1
    case 'II':
      return 2
    case 'III':
      return 3
    case 'IV':
      return 4
    case 'V':
      return 5
  }
  return 0
}

const relics = m.get('Relics') as ExportRelics
relics.ExportRelics.forEach((element, index) => {
  reverse_map.set(element.uniqueName, new ReverseIndex('Relics', 'ExportRelics', index))
  element.tier = element.uniqueName.replace('/Lotus/Types/Game/Projections/', '').substring(0, 2)
  const mid = element.name.split(' ')[1]
  if (element.tier === 'T5') {
    element.prefixChar = ''
    element.serial = roman2num(mid)
  } else {
    element.prefixChar = mid.charAt(0)
    element.serial = parseInt(mid.substring(1))
  }
  // console.log(element.prefixChar, '  ', element.serial)
  element.textureLocation = mf_kv.get(element.uniqueName)
  element.relicRewards.forEach((i) => {
    const key = i.rewardName.replace('/StoreItems', '')
    const idx = reverse_map.get(key, false)
    if (idx === undefined) {
      console.warn('can not find reverse index for: ', i.rewardName)
    } else {
      const item = (m.get(idx.category) as object)[idx.prop][idx.index]
      if (item === undefined) {
        console.warn('can not find item: ', idx)
      }
      i.name = item.name
      i.primeSellingPrice = item.primeSellingPrice
      i.textureLocation = item.textureLocation
      i.uniqueName = item.uniqueName
    }
  })
  element.relicRewards.sort((a, b) => {
    const ra = rarity2Num(a.rarity)
    const rb = rarity2Num(b.rarity)
    return Math.sign(rb - ra)
  })
})
const comp = new Intl.Collator('en')
function relicComparator(a: Relic, b: Relic) {
  if (a.tier === b.tier) {
    if (a.prefixChar === b.prefixChar) {
      return Math.sign(a.serial - b.serial)
    } else {
      return comp.compare(a.prefixChar, b.prefixChar)
    }
  } else {
    return comp.compare(a.tier, b.tier)
  }
}

relics.ExportRelics.sort(relicComparator)

// process.exit(0)

for (const [k, v] of m.entries()) {
  fs.writeFileSync(`${cache_dir}/${k}.json`, JSON.stringify(v, null, 2))
}

console.log('reverse index size: ', reverse_map.size)

const include_cats = ['Warframes', 'Weapons', 'Upgrades', 'Recipes', 'Relics', 'Arcane']
const i_list: string[] = []

for (const cat of include_cats) {
  const data = m.get(cat)
  if (data === undefined) {
    console.warn('can not find category: ', cat)
    continue
  }
  const text = JSON.stringify(data)
  const md5 = md5_hash(text)
  const filename = `Export${cat}_zh.json!00_${md5}`
  fs.writeFileSync(`${output_dir}/${filename}`, text)
  i_list.push(filename)
}

fs.writeFileSync(`${output_dir}/index.txt`, i_list.join('\r\n'))
save_endpoint_cache(cache)
console.log('used time :', Date.now() - start_time, 'ms')
