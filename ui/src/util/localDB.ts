import Dexie, { type EntityTable } from 'dexie'

export type StoreName =
  | 'Warframes'
  | 'Weapons'
  | 'Upgrades'
  | 'Arcane'
  | 'Recipes'
  | 'Relics'
  | 'Manifest'
export const RESOURCES_INCLUDED: StoreName[] = [
  'Warframes',
  'Weapons',
  'Upgrades',
  'Arcane',
  'Recipes',
  'Relics',
  'Manifest',
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
}
export interface Relics {
  uniqueName: string
}
export interface Commons {
  uniqueName: string
}

export interface Manifest {
  uniqueName: string
  textureLocation: string
}

const DB_NAME = 'wf-data'

export const db = new Dexie(DB_NAME) as Dexie & {
  index: EntityTable<Index, 'id'>
  last_update: EntityTable<LastUpdate, 'category'>
  Warframes: EntityTable<Warframes, 'uniqueName'>
  Weapons: EntityTable<Commons, 'uniqueName'>
  Upgrades: EntityTable<Commons, 'uniqueName'>
  Arcane: EntityTable<Commons, 'uniqueName'>
  Recipes: EntityTable<Commons, 'uniqueName'>
  Relics: EntityTable<Relics, 'uniqueName'>
  Manifest: EntityTable<Manifest, 'uniqueName'>
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
})

export async function current_index() {
  const index = await db.index.get(0)
  return index?.content ?? undefined
}

export async function put(storeName: StoreName, params: object) {
  // @ts-expect-error TODO: params type should be more specific
  return await db[storeName].put(params)
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
