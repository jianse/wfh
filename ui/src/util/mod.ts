import type { Upgrade } from './localDB'

export type ModClass =
  // 紫卡
  | 'Omega Unknown'
  | 'Omega Opened'
  // 并和
  | 'Amalgam'
  // 镀层
  | 'Galvanized'
  // 航道星舰
  | 'Railjack COMMON'
  | 'Railjack UNCOMMON'
  | 'Railjack RARE'
  // not used
  // | 'Railjack LEGENDARY'
  // 怪奇
  | 'Peculiar'
  // 典籍
  | 'Tome'
  // 执刑官
  | 'Archon'
  // 安魂
  | 'Immortal'
  // 杀毒
  | 'Antivirus'
  // 效价
  | 'Potency'
  // 常见
  | 'COMMON'
  | 'UNCOMMON'
  | 'RARE'
  // Umbral and Sacrificial are included in here
  | 'LEGENDARY'
  | 'Unknown'

export function getModClass(mod: Upgrade): ModClass {
  if (mod.upgradeEntries != undefined) {
    return 'Omega Unknown'
  }

  if (mod.tags?.some((tag) => tag.includes('Riven'))) {
    return 'Omega Opened'
  }

  if (mod.tags?.some((tag) => tag.includes('Amalgam'))) {
    return 'Amalgam'
  }

  if (mod.tags?.some((tag) => tag.includes('Galvanized'))) {
    return 'Galvanized'
  }

  if (mod.uniqueName.startsWith('/Lotus/Upgrades/Mods/Railjack')) {
    switch (mod.rarity) {
      case 'COMMON':
        return 'Railjack COMMON'
      case 'UNCOMMON':
        return 'Railjack UNCOMMON'
      case 'RARE':
        return 'Railjack RARE'
      case 'LEGENDARY':
        // this mod class is not used
        return 'Unknown'
    }
    // return 'Railjack ' + mod.rarity
  }

  if (mod.tags?.some((tag) => tag.includes('Peculiar'))) {
    return 'Peculiar'
  }

  if (mod.compatName === '典籍') {
    return 'Tome'
  }

  if (mod.tags?.some((tag) => tag.includes('Archon'))) {
    return 'Archon'
  }

  if (mod.uniqueName.startsWith('/Lotus/Upgrades/Mods/Immortal/Immortal')) {
    return 'Immortal'
  }

  if (mod.uniqueName.startsWith('/Lotus/Upgrades/Mods/Immortal/Antivirus')) {
    return 'Antivirus'
  }

  if (mod.uniqueName.startsWith('/Lotus/Upgrades/Mods/DataSpike/Potency')) {
    return 'Potency'
  }

  return mod.rarity
}
