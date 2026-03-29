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
  | 'Bounties'
  | 'Rewards'
export const RESOURCES_INCLUDED: StoreName[] = [
  'Warframes',
  'Weapons',
  'Upgrades',
  'Arcane',
  'Recipes',
  'Relics',
  'Manifest',
  'Resources',
  'Bounties',
  'Rewards',
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
  uniqueName: string
}
export interface Relic {
  uniqueName: string
  name: string
  codexSecret: boolean
  description: string
  relicRewards: RelicReward[]
  tier: string
  prefixChar: string
  serial: number
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
  textureLocation: string
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
  // description: string
  codexSecret: boolean
  // no rarity means common
  rarity?: 'UNCOMMON' | 'RARE' | 'LEGENDARY'
  levelStats: {
    stats: string[]
  }[]
  textureLocation: string
  inGame: boolean
  slot: string
}

interface Ingredient {
  // like uniqueName
  ItemType: string
  ItemCount: number
  ProductCategory: string
  name: string
  textureLocation: string
}

export interface Recipe {
  uniqueName: string
  resultType: string
  buildPrice: number
  buildTime: number
  skipBuildTimePrice: number
  consumeOnUse: boolean
  num: number
  codexSecret: boolean
  ingredients: Ingredient[]
  secretIngredients: Ingredient[]
  name: string
  description: string
  textureLocation: string
}
interface CommonWeapon {
  name: string
  uniqueName: string
  codexSecret: boolean
  damagePerShot: number[]
  totalDamage: number
  description: string
  criticalChance: number
  criticalMultiplier: number
  procChance: number
  fireRate: number
  masteryReq: number
  productCategory: string
  slot?: number
  textureLocation: string
}

export interface WeaponPistols extends CommonWeapon {
  productCategory: 'Pistols'
  accuracy?: number
  omegaAttenuation: number
  noise?: string
  trigger?: string
  magazineSize?: number
  reloadTime?: number
  multishot?: number
  maxLevelCap?: number
  excludeFromCodex?: boolean
  // this field is for kitgun weapons
  primeOmegaAttenuation?: number
}

export interface WeaponMelee extends CommonWeapon {
  productCategory: 'Melee'
  omegaAttenuation: number
  blockingAngle?: number
  comboDuration?: number
  followThrough?: number
  range?: number
  slamAttack?: number
  slamRadialDamage?: number
  slamRadius?: number
  slideAttack?: number
  heavyAttackDamage?: number
  heavySlamAttack?: number
  heavySlamRadialDamage?: number
  heavySlamRadius?: number
  windUp?: number
  maxLevelCap?: number
}

export interface WeaponLongGun extends CommonWeapon {
  productCategory: 'LongGuns'
  accuracy: number
  omegaAttenuation: number
  noise: string
  trigger: string
  magazineSize?: number
  reloadTime: number
  multishot: number
  maxLevelCap?: number
}

export interface WeaponSpaceGun extends CommonWeapon {
  productCategory: 'SpaceGuns'
  accuracy: number
  omegaAttenuation: number
  noise: string
  trigger: string
  magazineSize: number
  reloadTime: number
  multishot: number
  maxLevelCap?: number
}

export interface WeaponSpaceMelee extends CommonWeapon {
  productCategory: 'SpaceMelee'
  omegaAttenuation: number
  blockingAngle: number
  comboDuration: number
  followThrough: number
  range: number
  slamAttack: number
  slamRadialDamage: number
  slamRadius: number
  slideAttack: number
  heavyAttackDamage: number
  heavySlamAttack: number
}

export interface WeaponAmp extends CommonWeapon {
  productCategory: 'OperatorAmps'
  accuracy: number
  omegaAttenuation: number
  noise: string
  trigger: string
  magazineSize: number
  reloadTime: number
  multishot: number
}

export interface WeaponSentinel extends CommonWeapon {
  productCategory: 'SentinelWeapons'
  accuracy?: number
  omegaAttenuation: number
  noise?: string
  trigger?: string
  magazineSize?: number
  reloadTime?: number
  sentinel: boolean
  multishot?: number
  excludeFromCodex?: boolean
  blockingAngle?: number
}

export interface WeaponSpecialItems extends CommonWeapon {
  productCategory: 'SpecialItems'
  omegaAttenuation: number
  blockingAngle?: number
  comboDuration?: number
  followThrough?: number
  range?: number
  slamAttack?: number
  slamRadialDamage?: number
  slamRadius?: number
  slideAttack?: number
  heavyAttackDamage?: number
  heavySlamAttack?: number
  heavySlamRadialDamage?: number
  heavySlamRadius?: number
  windUp?: number
  accuracy?: number
  noise?: string
  trigger?: string
  reloadTime?: number
  multishot?: number
  magazineSize?: number
}

export type Weapon =
  | WeaponPistols
  | WeaponMelee
  | WeaponLongGun
  | WeaponSpaceGun
  | WeaponSpaceMelee
  | WeaponAmp
  | WeaponSentinel
  | WeaponSpecialItems

export interface Upgrade {
  uniqueName: string
  name: string
  polarity:
    | 'AP_POWER'
    | 'AP_DEFENSE'
    | 'AP_TACTIC'
    | 'AP_ATTACK'
    | 'AP_WARD'
    | 'AP_UNIVERSAL'
    | 'AP_UMBRA'
    | 'AP_PRECEPT'
    | 'AP_ANY'
  rarity: 'COMMON' | 'UNCOMMON' | 'RARE' | 'LEGENDARY'
  codexSecret: boolean
  baseDrain: number
  fusionLimit: number
  compatName?: string
  type?: string
  description?: string[]
  tags?: string[]
  isUtility?: boolean
  levelStats?: {
    stats: string[]
  }[]
  modSet?: string
  modSetValues?: number[]
  subType?: string
  excludeFromCodex?: boolean
  // riven mod only
  upgradeEntries?: {
    tag: string
    prefixTag: string
    suffixTag: string
    upgradeValues: {
      value: number
      locTag: string
    }[]
  }[]
  availableChallenges: {
    fullName: string
    description: string
    complications: {
      fullName: string
      description: string
    }[]
  }[]
  textureLocation: string
}

export interface WfPepEtag {
  url: string
  etag: string
}

/**
 * 奖励物品
 */
export interface RewardItem {
  /** 物品类型/唯一标识 */
  type: string
  /** 物品数量 */
  itemCount: number
  /** 掉落概率 (0-1) */
  probability: number
}

/**
 * 奖励表数据结构
 * 多个奖励以及每个奖励的掉落概率构成了一个奖励表
 */
export type RewardTable = RewardItem[]

/**
 * Rewards 数据结构
 * 键为奖励表唯一标识，值为奖励物品数组的数组（支持多轮奖励）
 */
export interface Rewards {
  uniqueName: string
  /**
   * 一个任务奖励可以由一个或多个奖励表构成，多个奖励表时，表示该任务包含多个奖励阶级，达成不同的条件会适用不同的奖励表中的奖励
   */
  value: RewardTable[]
}

/**
 * 赏金任务阶段，由一个或多个可能的赏金组成
 */
export type BountyStage = string[]

/**
 * Bounties 数据结构
 * 键为赏金任务唯一标识
 */
export interface Bounties {
  uniqueName: string
  value: {
    /** 赏金任务名称（本地化键） */
    name: string
    /** 赏金任务描述（本地化键） */
    description: string
    /** 赏金任务图标路径 */
    icon: string
    /** 赏金任务阶段配置 */
    stages: BountyStage[]
  }
}

const DB_NAME = 'wf-data'

export const db = new Dexie(DB_NAME) as Dexie & {
  index: EntityTable<Index, 'id'>
  wfPepEtag: EntityTable<WfPepEtag, 'url'>
  last_update: EntityTable<LastUpdate, 'category'>
  Warframes: EntityTable<Warframes, 'uniqueName'>
  Weapons: EntityTable<Weapon, 'uniqueName'>
  Upgrades: EntityTable<Upgrade, 'uniqueName'>
  Arcane: EntityTable<Arcane, 'uniqueName'>
  Recipes: EntityTable<Recipe, 'uniqueName'>
  Relics: EntityTable<Relic, 'uniqueName'>
  Manifest: EntityTable<Manifest, 'uniqueName'>
  Resources: EntityTable<Resources, 'uniqueName'>
  Rewards: EntityTable<Rewards, 'uniqueName'>
  Bounties: EntityTable<Bounties, 'uniqueName'>
}

db.version(1).stores({
  index: 'id, content, valid',
  wfPepEtag: 'url',
  last_update: 'category',
  Warframes: 'uniqueName, name',
  Weapons: 'uniqueName',
  Upgrades: 'uniqueName',
  Arcane: 'uniqueName',
  Recipes: 'uniqueName',
  Relics: 'uniqueName,order',
  Manifest: 'uniqueName',
  Resources: 'uniqueName',
  Rewards: 'uniqueName',
  Bounties: 'uniqueName',
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

export const useORelics = () =>
  useObservable(
    liveQuery(async () => await db.Relics.orderBy('order').toArray()) as unknown as Observable<
      Relic[]
    >,
    { initialValue: [] },
  )

export const useORecipes = () =>
  useObservable(liveQuery(() => db.Recipes.toArray()) as unknown as Observable<Recipe[]>, {
    initialValue: [],
  })

export const useOWeapons = () =>
  useObservable(liveQuery(() => db.Weapons.toArray()) as unknown as Observable<Weapon[]>, {
    initialValue: [],
  })

export const useOUpgrades = () =>
  useObservable(liveQuery(() => db.Upgrades.toArray()) as unknown as Observable<Upgrade[]>, {
    initialValue: [],
  })

export const useOResources = () =>
  useObservable(liveQuery(() => db.Resources.toArray()) as unknown as Observable<Resources[]>, {
    initialValue: [],
  })

export const useORewards = () =>
  useObservable(liveQuery(() => db.Rewards.toArray()) as unknown as Observable<Rewards[]>, {
    initialValue: [],
  })

export const useOBounties = () =>
  useObservable(liveQuery(() => db.Bounties.toArray()) as unknown as Observable<Bounties[]>, {
    initialValue: [],
  })

/**
 * Find a Warframe by its uniqueName
 */
export async function findWarframe(uniqueName: string): Promise<Warframes | undefined> {
  return await db.Warframes.get(uniqueName)
}

/**
 * Find a Resource by its uniqueName
 */
export async function findResource(uniqueName: string): Promise<Resources | undefined> {
  return await db.Resources.get(uniqueName)
}

/**
 * Find Rewards by its uniqueName
 */
export async function findRewards(uniqueName: string): Promise<Rewards | undefined> {
  return await db.Rewards.get(uniqueName)
}

/**
 * Find Bounties by its uniqueName
 */
export async function findBounties(uniqueName: string): Promise<Bounties | undefined> {
  return await db.Bounties.get(uniqueName)
}
