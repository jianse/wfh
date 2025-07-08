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
} from './localDB'
import { chunk } from 'lodash'

/**
 * load index for all resources
 * @returns resource index
 */
export async function load_index() {
  const resp = await fetch('/data/index.txt')
  const index = await resp.text()
  return index
}

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

export async function check_update_available(force: boolean = false) {
  console.log('start checking update')
  if (inprogress) {
    console.warn('another update process is inprogress!')
  }
  try {
    inprogress = true

    const index = await load_index()
    let local_index = (await current_index()) || ''
    if (force) {
      local_index = ''
    }
    const update = index !== local_index

    if (update) {
      console.log('update found!')
      // do update
      const upd_list = to_be_update_items(local_index, index)

      const jobs = upd_list
        .map((i) => ({ url: i, category: get_category(i) as StoreName }))
        .filter(({ category }) => RESOURCES_INCLUDED.includes(category))
        .map((i) => sync_one(i.url, i.category))

      await Promise.all(jobs)
      await save_index(index)
      console.log('updating done!')
    } else {
      console.log('already up to date!')
    }
  } finally {
    inprogress = false
  }
}

const bulk = true
const chunk_size = 1000

// sync one of the resource type
async function sync_one(item: string, category: StoreName) {
  console.info('syncing category: ', category)
  const store = useSyncStore()
  store.setInprogress(category)

  await debug_sleep(1000)

  const arr = await load_resource(item)
  await clear(category)
  await debug_sleep(1000)
  store.setCategoryTotal(category, arr.length)
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

export async function update_one(category: string) {
  if (!RESOURCES_INCLUDED.includes(category as StoreName)) {
    return
  }
  const index = await current_index()
  if (index) {
    try {
      inprogress = true
      const url = index.split(/\r?\n/g).find((i) => i.includes(category))
      if (!url) {
        return
      }
      await sync_one(url, category as StoreName)
    } finally {
      inprogress = false
    }
  } else {
    check_update_available()
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
