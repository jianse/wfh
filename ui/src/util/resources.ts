import { useSyncStore } from '@/stores/sync'
import {
  current_index,
  put,
  bulkPut,
  RESOURCES_INCLUDED,
  save_index,
  set_last_update_time,
  type StoreName,
  clear,
  type Relic,
  db,
} from './localDB'
import { chunk } from 'lodash'

/**
 * 类别到数据源的映射配置
 * 定义哪些类别从哪个数据源同步
 */
const CATEGORY_SOURCE_MAP: Record<string, 'local' | 'wfpep'> = {
  // 从 wfpep 同步的类别
  Rewards: 'wfpep',
  Bounties: 'wfpep',
  FusionBundles: 'wfpep',
  Warframes: 'wfpep',
  Images: 'wfpep',
  // 其他类别默认从 local 同步
}

/**
 * 获取类别的数据源
 * @param category 类别名称
 * @returns 数据源类型
 */
function get_category_source(category: string): 'local' | 'wfpep' {
  return CATEGORY_SOURCE_MAP[category] || 'local'
}

/**
 * 加载本地资源索引
 * @returns resource index
 */
export async function load_index() {
  const resp = await fetch('/data/index.txt')
  const index = await resp.text()
  return index
}

/**
 * 从本地服务器加载资源
 */
async function load_resource(key: string): Promise<Array<object>> {
  const resp = await fetch(`/data/${key}`)
  const root = await resp.json()
  const category = get_category(key)
  if (category == 'Manifest') {
    return root.Manifest
  } else {
    return root[`Export${category}`]
  }
}

const debug = false
const debug_sleep = (ms: number) =>
  new Promise((resolve) => {
    if (debug) {
      setTimeout(resolve, ms)
    } else {
      resolve(undefined)
    }
  })
let inprogress = false

const bulk = true
const chunk_size = 1000

/**
 * 通用的同步数据到本地数据库方法
 * @param category 数据类别
 * @param data 要同步的数据数组
 * @param source 数据来源标识
 */
async function sync_to_local_db(category: StoreName, data: Array<object>, source: string) {
  console.info(`syncing category: ${category} from ${source}`)
  const store = useSyncStore()
  store.setInprogress(category)

  await debug_sleep(1000)
  await clear(category)
  await debug_sleep(1000)

  let arr = data
  store.setCategoryTotal(category, arr.length)

  // 对 Relics 特殊处理，添加排序字段
  if (category === 'Relics') {
    arr = arr.map((item) => {
      const it = item as unknown as Relic
      const order = it.tier + (it.prefixChar || 'A') + new String(it.serial).padStart(3, '0')
      return { ...item, order }
    })
  }

  // 批量写入数据
  if (bulk) {
    for (const shard of chunk(arr, chunk_size)) {
      const _ret = await bulkPut(category as StoreName, shard)
      store.increaseCategoryCompletedBy(category, shard.length)
    }
  } else {
    for (const it of arr) {
      const _ret = await put(category as StoreName, it)
      store.increaseCategoryCompleted(category)
    }
  }

  set_last_update_time(category, new Date(), arr.length)
  store.markCategoryDone(category)
  await debug_sleep(1000)
}
const BASE_URL = 'https://cdn.jsdelivr.net/gh/calamity-inc/warframe-public-export-plus@senpai/'

/**
 * 从 warframe-public-export-plus (jsdelivr) 同步单个文件
 */
async function sync_from_wfpep(filename: string, force = false) {
  const url = `${BASE_URL}${filename}`
  const category = get_category(filename)

  const resp = await fetch(url)

  if (resp.status === 200) {
    if (!force) {
      const localEtag = (await db.wfPepEtag.get(url))?.etag || ''
      if (localEtag === resp.headers.get('ETag')) {
        console.log('no update found for', category)
        return
      }
    }

    console.log('update found for', category)
    const data: Record<string, Object | Array<unknown>> = await resp.json()
    const arr = Object.entries(data).map(([key, value]) => {
      if (Array.isArray(value)) {
        return {
          uniqueName: key,
          value,
        }
      }
      return {
        uniqueName: key,
        ...value,
      }
    })

    await sync_to_local_db(category as StoreName, arr, 'wfpep')
    await db.wfPepEtag.put({ url, etag: resp.headers.get('ETag') || '' })
  } else {
    console.log('failed to fetch', category, resp)
  }
}

/**
 * 从本地服务器同步单个资源
 */
async function sync_from_local(url: string, category: StoreName) {
  const arr = await load_resource(url)
  await sync_to_local_db(category, arr, 'local')
}

/**
 * 根据类别自动选择数据源进行同步
 * @param category 类别名称
 */

export async function sync_category(category: StoreName) {
  const source = get_category_source(category)

  if (source === 'wfpep') {
    const filename = `Export${category}.json`
    await sync_from_wfpep(filename, true)
  } else {
    const index = await load_index()
    if (index) {
      const url = index.split(/\r?\n/g).find((i) => i.includes(category))
      if (!url) {
        console.warn('category not found in index:', category)
        return
      }
      await sync_from_local(url, category)
    }
  }
}

/**
 * 同步所有资源
 * 根据 CATEGORY_SOURCE_MAP 配置自动选择数据源
 * @param force 是否强制重新同步
 */
export async function sync_all(force: boolean = false) {
  console.log('start syncing all')

  if (inprogress) {
    console.warn('another sync process is in progress!')
    return
  }

  try {
    inprogress = true

    // 获取所有需要同步的类别
    const categories = RESOURCES_INCLUDED

    // 分离不同数据源的类别
    const wfpepCategories = categories.filter((cat) => get_category_source(cat) === 'wfpep')
    const localCategories = categories.filter((cat) => get_category_source(cat) === 'local')

    // 同步 wfpep 类别的数据
    if (wfpepCategories.length > 0) {
      console.log('syncing wfpep categories:', wfpepCategories)
      await Promise.all(wfpepCategories.map((cat) => sync_from_wfpep(`Export${cat}.json`)))
    }

    // 同步本地类别的数据
    if (localCategories.length > 0) {
      console.log('syncing local categories:', localCategories)
      const index = await load_index()
      let local_index = (await current_index()) || ''

      if (force) {
        local_index = ''
      }

      const needsUpdate = index !== local_index

      if (needsUpdate || force) {
        const upd_list = force
          ? index.split(/\r?\n/g).filter((i) => i.trim())
          : to_be_update_items(local_index, index)

        const jobs = upd_list
          .map((i) => ({ url: i, category: get_category(i) as StoreName }))
          .filter(({ category }) => localCategories.includes(category))
          .map((i) => sync_from_local(i.url, i.category))

        await Promise.all(jobs)
        await save_index(index)
      }
    }

    console.log('sync all done!')
  } finally {
    inprogress = false
  }
}

/**
 * 强制同步单个类别
 * 自动根据类别选择数据源
 * @param category 类别名称
 */
export async function force_sync_one(category: string) {
  if (!RESOURCES_INCLUDED.includes(category as StoreName)) {
    console.warn('category not included:', category)
    return
  }

  if (inprogress) {
    console.warn('another sync process is in progress!')
    return
  }

  try {
    inprogress = true

    const source = get_category_source(category)

    if (source === 'wfpep') {
      // 清除 etag 强制重新下载
      const url = `${BASE_URL}Export${category}.json`
      await db.wfPepEtag.delete(url)
      await sync_from_wfpep(`Export${category}.json`)
    } else {
      // 从本地服务器强制同步
      const index = await load_index()
      if (index) {
        const url = index.split(/\r?\n/g).find((i) => i.includes(category))
        if (!url) {
          console.warn('category not found in index:', category)
          return
        }
        await sync_from_local(url, category as StoreName)
      }
    }
  } finally {
    inprogress = false
  }
}

function get_category(name: string): string {
  return name.replace('Export', '').replace(/(_[a-z]{2})?\.json.*/, '')
}

function to_be_update_items(old_index: string, new_index: string): string[] {
  const old_items = old_index.split(/\r?\n/g)
  const new_items = new_index.split(/\r?\n/g)
  return new_items.filter((el) => !old_items.includes(el))
}

/**
 * get a string's sha256 hash
 * @param message message to be hashed
 * @returns hex string hash value
 */
export async function sha256(message: string) {
  const msg = new TextEncoder().encode(message)
  const hashBuffer = await crypto.subtle.digest('SHA-256', msg)
  const hashArray = Array.from(new Uint8Array(hashBuffer))
  const hashHex = hashArray.map((b) => b.toString(16).padStart(2, '0')).join('')
  return hashHex
}
