import fs from 'node:fs'
import lzma from 'lzma'
import { createHash } from 'node:crypto'

const output_dir = 'public/data'
const cache_dir = 'cache'
// Check if the output directory exists, if not create it
if (!fs.existsSync(output_dir)) {
  fs.mkdirSync(output_dir, { recursive: true })
}
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
function save_endpoint_cache(cache) {
  const json = Object.fromEntries(cache)
  fs.writeFileSync(`${cache_dir}/endpointCache2.json`, JSON.stringify(json))
  console.log('Endpoint cache saved')
}

const cache = load_or_create_endpoint_cache()

async function fetch_index() {
  let origin = 'https://origin.warframe.com/PublicExport/index_zh.txt.lzma'
  const res = await fetch(origin)
  const raw = await res.bytes()

  const decompressed = lzma.decompress(raw)
  cache.set(origin, decompressed)
  return decompressed
}

function md5_hash(message) {
  const hash = createHash('md5')
  hash.update(message)
  return hash.digest('base64url')
}

async function get_or(cache, key, cb) {
  const value = cache.get(key)
  if (value === undefined) {
    console.log(`cache miss for: ${key} . using callback to fetch`)
    const value = await cb(key)
    cache.set(key, value)
    save_endpoint_cache(cache)
    return value
  } else {
    console.log(`cache hit for: ${key}`)
    return value
  }
}

const index = await fetch_index()
// console.log(index)

const endpoints = index.split(/\r?\n/g)
console.log(endpoints)

let new_endpoints = []

/**
 * decode origin base64 hash then encode it into base64url
 */
function rehash(hash) {
  return Buffer.from(hash, 'base64').toString('base64url')
}

/**
 *
 * @param {string} name
 */
function rename_item(name) {
  const n = name.replace(/\.json(.*)/, '.json')
  const hash = name.replace(/(.*json!00_)/, '')
  const new_hash = rehash(hash)
  // console.log('rename', n, hash, new_hash)

  return `${n}!00_${new_hash}`
}

for (const it of endpoints) {
  const url = `https://content.warframe.com/PublicExport/Manifest/${it}`
  const cont = await get_or(cache, url, async () => {
    const resp = await fetch(url)
    return await resp.text()
  })
  if (it.startsWith('ExportRelicArcane')) {
    // split relics and arcane
    // relics
    const json = JSON.parse(cont)
    const relicsData = json.ExportRelicArcane.filter(
      (item) =>
        item.uniqueName.startsWith('/Lotus/Types/Game/Projections') &&
        item.uniqueName.endsWith('Bronze'),
    )
    const relics_text = JSON.stringify({ ExportRelics: relicsData })
    const relics_md5 = md5_hash(relics_text)
    const relics_name = `ExportRelics_zh.json!00_${relics_md5.replace(/=/g, '')}`
    fs.writeFileSync(`${output_dir}/${relics_name}`, relics_text)
    new_endpoints.push(relics_name)

    // arcane
    const arcaneData = json.ExportRelicArcane.filter(
      (item) =>
        item.uniqueName.startsWith('/Lotus/Upgrades/CosmeticEnhancers') &&
        item.levelStats !== undefined,
    )

    const arcane_text = JSON.stringify({ ExportArcane: arcaneData })
    const arcane_md5 = md5_hash(arcane_text)
    // console.log(arcane_md5)
    const arcane_name = `ExportArcane_zh.json!00_${arcane_md5.replace(/=/g, '')}`
    fs.writeFileSync(`${output_dir}/${arcane_name}`, arcane_text)
    new_endpoints.push(arcane_name)
  } else {
    // save others
    const new_name = rename_item(it)
    fs.writeFileSync(`${output_dir}/${new_name}`, cont)
    new_endpoints.push(new_name)
  }
}

fs.writeFileSync(`${output_dir}/index.txt`, new_endpoints.join('\r\n'))

// const customs = cache.get(
//   'https://content.warframe.com/PublicExport/Manifest/ExportCustoms_zh.json!00_GLZM2b6tV-x5Vv3uldlllw',
// )
// const c_md5 = md5_hash(customs)

// console.log(c_md5)

save_endpoint_cache(cache)
