import { useObservable } from '@vueuse/rxjs'
import Dexie, { liveQuery, type EntityTable } from 'dexie'
import type { Observable } from 'rxjs'

export type StoreName =
  | 'Warframes'
  | 'Weapons'
  | 'Upgrades'
  | 'Arcane'
  | 'Recipes'
  | 'Relics'
  | 'Manifest'
  | 'Resources'
export const RESOURCES_INCLUDED: StoreName[] = [
  'Warframes',
  'Weapons',
  'Upgrades',
  'Arcane',
  'Recipes',
  'Relics',
  'Manifest',
  'Resources',
]

interface LastUpdate {
  category: string
  last_update_time: Date
  total: number
}

interface Index {
  id: number
  content: string
  valid: boolean
}
export interface Warframes {
  uniqueName: string
  name: string
  description: string
  productCategory: 'SpaceSuits' | 'Suits' | 'MechSuits'
  textureLocation: string
}

export interface RelicReward {
  // relic reward uniqueName
  rewardName: string
  rarity: 'UNCOMMON' | 'COMMON' | 'RARE'
  tier: number
  itemCount: number
  name?: string
  primeSellingPrice?: number
  textureLocation?: string
}
export interface Relic {
  uniqueName: string
  name: string
  codexSecret: boolean
  description: string
  relicRewards: RelicReward[]
  tier: string
  textureLocation: string
}

export interface Resources {
  uniqueName: string
  name: string
  description: string
  codexSecret: boolean
  parentName: string
  excludeFromCodex: boolean
  primeSellingPrice?: number
}

export interface Commons {
  uniqueName: string
}

export interface Manifest {
  uniqueName: string
  textureLocation: string
}

export interface Arcane {
  uniqueName: string
  name: string
  description: string
  rarity: string
  textureLocation: string
}

const DB_NAME = 'wf-data'

export const db = new Dexie(DB_NAME) as Dexie & {
  index: EntityTable<Index, 'id'>
  last_update: EntityTable<LastUpdate, 'category'>
  Warframes: EntityTable<Warframes, 'uniqueName'>
  Weapons: EntityTable<Commons, 'uniqueName'>
  Upgrades: EntityTable<Commons, 'uniqueName'>
  Arcane: EntityTable<Arcane, 'uniqueName'>
  Recipes: EntityTable<Commons, 'uniqueName'>
  Relics: EntityTable<Relic, 'uniqueName'>
  Manifest: EntityTable<Manifest, 'uniqueName'>
  Resources: EntityTable<Resources, 'uniqueName'>
}

db.version(1).stores({
  index: 'id, content, valid',
  last_update: 'category',
  Warframes: 'uniqueName, name',
  Weapons: 'uniqueName',
  Upgrades: 'uniqueName',
  Arcane: 'uniqueName',
  Recipes: 'uniqueName',
  Relics: 'uniqueName',
  Manifest: 'uniqueName',
  Resources: 'uniqueName',
})

export async function current_index() {
  const index = await db.index.get(0)
  return index?.content ?? undefined
}

export async function put(storeName: StoreName, params: object) {
  // @ts-expect-error TODO: params type should be more specific
  return await db[storeName].put(params)
}

export async function bulkPut(storeName: StoreName, params: object[]) {
  // @ts-expect-error TODO: params type should be more specific
  return await db[storeName].bulkPut(params)
}

export async function clear(storeName: StoreName) {
  return await db[storeName].clear()
}

export async function save_index(index: string) {
  return await db.index.put({ id: 0, content: index, valid: true })
}

export async function get_last_update_time(category: string) {
  return await db.last_update.get(category)
}

export async function set_last_update_time(
  category: string,
  last_update_time: Date,
  total: number,
) {
  return await db.last_update.put({ category, last_update_time, total })
}

export async function count_in_db(category: StoreName) {
  return await db[category].count()
}

export const useOManifest = () =>
  useObservable(liveQuery(() => db.Manifest.toArray()) as unknown as Observable<Manifest[]>, {
    initialValue: [],
  })

export const useOWarframes = () =>
  useObservable(
    liveQuery(() => db.Warframes.orderBy('name').toArray()) as unknown as Observable<Warframes[]>,
    { initialValue: [] },
  )

export const useOArcane = () =>
  useObservable(liveQuery(() => db.Arcane.toArray()) as unknown as Observable<Arcane[]>, {
    initialValue: [],
  })
