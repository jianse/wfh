import lzma from 'lzma'
import fs from 'node:fs'
import { createHash } from 'node:crypto'
import { cloneDeep  } from 'lodash-es'

let cache_hit = false

const output_dir = './public/data'
const cache_dir = './cache'
// Check if the output directory exists, if not create it
if (!fs.existsSync(output_dir)) {
  fs.mkdirSync(output_dir, { recursive: true })
}
// Check if the cache directory exists, if not create it
if (!fs.existsSync(cache_dir)) {
  fs.mkdirSync(cache_dir, { recursive: true })
}

function loadEndpointCache() {
  console.log('Loading endpoint cache')
  // Check if the cache file exists
  if (fs.existsSync(`${cache_dir}/endpointCache.json`)) {
    // TODO: load the cache from the file
    const json = fs.readFileSync(`${cache_dir}/endpointCache.json`, 'utf-8')
    const obj = JSON.parse(json)
    const endpointCache = new Map()
    for (const [key, value] of Object.entries(obj)) {
      endpointCache.set(key, value)
    }
    console.log('Endpoint cache loaded')
    return endpointCache
  } else {
    return new Map()
  }
}
function saveEndpointCache(endpointCache) {
  const json = Object.fromEntries(endpointCache)
  fs.writeFileSync(`${cache_dir}/endpointCache.json`, JSON.stringify(json))
  console.log('Endpoint cache saved')
}

const endpointCache = loadEndpointCache()
let originServerAvailable = false

async function getJSON(url) {
  const res = await fetch(url)
  return await res.json()
}

const retryAttempts = async (numAttempts, workerFn) => {
  while (numAttempts > 0) {
    try {
      return workerFn()
    } catch (error) {
      if (numAttempts > 0) {
        numAttempts -= 1
      } else {
        throw error
      }
    }
  }
}

async function fetchEndpoints(locale) {
  return retryAttempts(5, async () => {
    let origin = ''

    // Use content.warframe.com if the origin server is not available
    origin = `https://${originServerAvailable ? 'origin' : 'content'}.warframe.com/PublicExport/index_${
      locale || 'en'
    }.txt.lzma`
    const res = await fetch(origin)
    const raw = await res.bytes()

    // Check if the hash is the same as the one in the cache
    const hash = createHash('sha256')

    hash.update(raw)
    const digest = hash.digest('hex')

    let oldHash = endpointCache.get(origin)
    if (oldHash === digest) {
      console.log('Cache hit')
      cache_hit = true
    } else {
      endpointCache.set(origin, digest)
      saveEndpointCache(endpointCache)
    }

    const decompressed = lzma.decompress(raw)
    // endpointCache.set(origin, raw) // Cache endpoint only if lzma.decrompress didn't throw an error
    // console.log(decompressed)
    return decompressed.split(/\r?\n/g)
  })
}

async function checkOriginServerAvailability() {
  originServerAvailable = true
  try {
    // If this call is successful, it means that the origin server is available.
    const _r = await fetchEndpoints(false, 'zh')
  } catch (err) {
    console.error(err)
    // Origin server not available, fall back to content server
    originServerAvailable = false
    console.warn(
      'origin.warframe.com not available. Using content.warframe.com instead. Data might not be up-to-date!!!',
    )
  }
}

// main script start
// Check if the origin server is available
await checkOriginServerAvailability()

async function fetchEndpoint(endpoint) {
  console.log('fetching endpoint: ', endpoint)
  const category = endpoint.replace('Export', '').replace(/(_[a-z]{2})?\.json.*/, '')
  const raw = await getJSON(`https://content.warframe.com/PublicExport/Manifest/${endpoint}`, true)
  fs.writeFileSync(`${cache_dir}/${category}.json`, JSON.stringify(raw))
  return { category, raw }
}

async function loadData(endpoints) {
  let data = new Map()

  if (cache_hit) {
    console.log('Cache hit, load from local fs')
    for (const endpoint of endpoints) {
      const category = endpoint.replace('Export', '').replace(/(_[a-z]{2})?\.json.*/, '')
      const raw = JSON.parse(fs.readFileSync(`${cache_dir}/${category}.json`, 'utf-8'))
      data.set(category, raw)
    }
  } else {
    console.log('Cache miss, load from server')
    for (const endpoint of endpoints) {
      // console.log(endpoint)
      const res = await fetchEndpoint(endpoint)
      // console.log(res)
      data.set(res.category, res.raw)
    }
  }
  return data
}

const endpoints = await fetchEndpoints('zh')

// console.log('Endpoints: ', endpoints)

const data = await loadData(endpoints)

// console.log(data.keys())

function buildUniqueNameCategory(data) {
  const uniqueNameCategory = new Map()

  // gear
  const gear = data.get('Gear')
  for (const item of gear.ExportGear) {
    uniqueNameCategory.set(item.uniqueName, 'Gear')
  }

  // resources
  const resources = data.get('Resources')
  for (const item of resources.ExportResources) {
    uniqueNameCategory.set(item.uniqueName, 'Resources')
  }

  // sentinels
  const sentinels = data.get('Sentinels')
  for (const item of sentinels.ExportSentinels) {
    uniqueNameCategory.set(item.uniqueName, 'Sentinels')
  }
  // warframes
  const warframes = data.get('Warframes')
  for (const item of warframes.ExportWarframes) {
    uniqueNameCategory.set(item.uniqueName, 'Warframes')
  }
  // weapons
  const weapons = data.get('Weapons')
  for (const item of weapons.ExportWeapons) {
    uniqueNameCategory.set(item.uniqueName, 'Weapons')
  }

  // keys
  const keys = data.get('Keys')
  for (const item of keys.ExportKeys) {
    uniqueNameCategory.set(item.uniqueName, 'Keys')
  }

  // Upgrades
  const upgrades = data.get('Upgrades')
  for (const item of upgrades.ExportUpgrades) {
    uniqueNameCategory.set(item.uniqueName, 'Upgrades')
  }

  // Customs
  const customs = data.get('Customs')
  for (const item of customs.ExportCustoms) {
    uniqueNameCategory.set(item.uniqueName, 'Customs')
  }

  // drones
  const drones = data.get('Drones')
  for (const item of drones.ExportDrones) {
    uniqueNameCategory.set(item.uniqueName, 'Drones')
  }

  // relicArcane
  const relicArcane = data.get('RelicArcane')
  for (const item of relicArcane.ExportRelicArcane) {
    uniqueNameCategory.set(item.uniqueName, 'RelicArcane')
  }

  // recipes
  const recipes = data.get('Recipes')
  for (const item of recipes.ExportRecipes) {
    uniqueNameCategory.set(item.uniqueName, 'Recipes')
  }
  return uniqueNameCategory
}

const uniqueNameCategory = buildUniqueNameCategory(data)
// console.log(uniqueNameCategory)

function findImageByUniqueName(uniqueName, options = { warn: true }) {
  const mfi = data.get('Manifest').Manifest.find((item) => item.uniqueName === uniqueName)
  if (!mfi) {
    if (options.warn) {
      console.warn('Manifest item not found', uniqueName)
    }
    // return recipe
    return ''
  } else {
    return `https://content.warframe.com/PublicExport${mfi.textureLocation}`
  }
}

function findItemByUniqueName(uniqueName, options = { warn: true }) {
  const category = uniqueNameCategory.get(uniqueName)
  if (!category) {
    if (options.warn) {
      console.warn('Item not found', uniqueName)
    }
    return {
      name: uniqueName,
      category: 'Unknown',
      imgUrl: findImageByUniqueName(uniqueName, options),
    }
  }
  const item = cloneDeep(
    data.get(category)[`Export${category}`].find((item) => item.uniqueName === uniqueName),
  )

  item.category = category

  item.imgUrl = findImageByUniqueName(uniqueName, options)

  return item
}

function fillRecipe(recipe) {
  recipe.resultItem = findItemByUniqueName(recipe.resultType)
  // const mfi = data.get('Manifest').Manifest.find((item) => item.uniqueName === recipe.resultType)

  for (const ingredient of recipe.ingredients) {
    ingredient.item = findItemByUniqueName(ingredient.ItemType)
  }

  for (const secretIngredient of recipe.secretIngredients) {
    secretIngredient.item = findItemByUniqueName(secretIngredient.ItemType)
  }

  return recipe
}

// post process
// 1. recipes
console.log('Processing recipes')
const recipes = data.get('Recipes')

// fill recipe name
recipes.ExportRecipes.forEach((recipe) => {
  const resultItem = findItemByUniqueName(recipe.resultType)
  recipe.name = `${resultItem.name} 蓝图`
})

const recipesData = cloneDeep(recipes)

recipesData.ExportRecipes = recipesData.ExportRecipes.map(fillRecipe)

fs.writeFileSync(`${output_dir}/Recipes-filled.json`, JSON.stringify(recipesData, undefined, 2))

// 2. relics and arcane
const relics = cloneDeep(data.get('RelicArcane'))

// split relics and arcane
// 2.1 relics
console.log('Processing relics')
const relicsData = relics.ExportRelicArcane.filter(
  (item) =>
    item.uniqueName.startsWith('/Lotus/Types/Game/Projections') &&
    item.uniqueName.endsWith('Bronze'),
).map((it) => {
  const prop = processRelicName(it.uniqueName, it.name)
  it.tier = prop[0]
  it.middleName = prop[1]
  it.quality = prop[2]
  it.series = prop[3]
  it.indexNum = prop[4]
  return it
})

function processRelicName(uniqueName, name) {
  const lastUniName = uniqueName.substring(uniqueName.lastIndexOf('/') + 1)
  const tier = lastUniName.substring(0, 2)
  // console.log('tire', tire)
  let quality = ''
  let len = 0
  if (lastUniName.endsWith('Bronze')) {
    quality = 'Bronze'
    len = 6
  }
  if (lastUniName.endsWith('Silver')) {
    quality = 'Silver'
    len = 6
  }
  if (lastUniName.endsWith('Gold')) {
    quality = 'Silver'
    len = 4
  }
  if (lastUniName.endsWith('Platinum')) {
    quality = 'Platinum'
    len = 8
  }
  let middle = name.substring(2, name.length - len)

  let series = ''
  let indexNum = 0
  if (tier === 'T5') {
    series = name.split(' ')[1]
  } else {
    series = name.charAt(3)
    indexNum = Number(name.split(' ')[1].substring(1))
  }

  return [tier, middle, quality, series, indexNum]
}
function quality2Num(quality) {
  switch (quality) {
    case 'Bronze':
      return 0
    case 'Silver':
      return 1
    case 'Gold':
      return 2
    case 'Platinum':
      return 3
    default:
      return 0
  }
}

relicsData.sort((a, b) => {
  if (a.tier !== b.tier) {
    return a.tier < b.tier ? -1 : 1
  } else if (a.series !== b.series) {
    return a.series > b.series ? 1 : -1
  } else if (a.indexNum !== b.indexNum) {
    return a.indexNum > b.indexNum ? 1 : -1
  } else if (a.quality !== b.quality) {
    return quality2Num(a.quality) > quality2Num(b.quality) ? 1 : -1
  }
  return 0
})

function rarity2num(rarity) {
  switch (rarity) {
    case 'COMMON':
      return 0
    case 'UNCOMMON':
      return 1
    case 'RARE':
      return 2
    case 'LEGENDARY':
      return 3
    default:
      return 0
  }
}

// fill relic rewards
relicsData.forEach((item) => {
  for (const reward of item.relicRewards) {
    let item = findItemByUniqueName(reward.rewardName, { warn: false })
    if (item.category === 'Unknown') {
      // try
      const rewardName = reward.rewardName.replace('/StoreItems', '')
      item = findItemByUniqueName(rewardName)
    }
    reward.item = item
    if (item.category === 'Recipes') {
      delete item.ingredients
    }
  }

  // rewards rarity order
  item.relicRewards.sort((a, b) => {
    const rna = rarity2num(a.rarity)
    const rnb = rarity2num(b.rarity)
    return rnb - rna
  })

  item.imgUrl = findImageByUniqueName(item.uniqueName)
})

fs.writeFileSync(
  `${output_dir}/Relics-filled.json`,
  JSON.stringify({ ExportRelic: relicsData }, undefined, 2),
)

const relicsStatus = relicsData
  .filter((it) => it.uniqueName.endsWith('Bronze'))
  .map((it) => {
    return { name: it.name, status: 'InVault' }
  })

fs.writeFileSync(
  `${output_dir}/Relics-status-tpl.json`,
  JSON.stringify({ ExportRelicStatus: relicsStatus }, undefined, 2),
)

// 2.2 arcane
console.log('Processing arcane')
const arcaneData = relics.ExportRelicArcane.filter(
  (item) =>
    item.uniqueName.startsWith('/Lotus/Upgrades/CosmeticEnhancers') &&
    item.levelStats !== undefined,
).map((item) => {
  item.imgUrl = findImageByUniqueName(item.uniqueName)
  return item
})

fs.writeFileSync(
  `${output_dir}/Arcane-filled.json`,
  JSON.stringify({ ExportArcane: arcaneData }, undefined, 2),
)
