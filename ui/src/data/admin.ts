// AUTO-EXTRACTED from the wfh-helper.html prototype.

export type Role = 'normal' | 'maintainer' | 'admin'

export interface SeedUser {
  id: number
  username: string
  role: Role
  password: string
}

export interface Passkey {
  id: number
  name: string
  created: string
}

export interface SortieModifier {
  id: string
  name: string
  description: string
  vars: Record<string, string>
}

export interface Replacement {
  path: string
  origin: string
  replacement: string
  remark: string
}

export interface SyncRecord {
  id: number
  sync_time: number
  is_success: boolean
  time_spent_ms: number
  warnings: string[]
  error_msg: string
  nearest_expiry: number
}

const SYNC_RECORDS: SyncRecord[] = (() => {
  const now = Date.now()
  const samples: [boolean, number, string[], string][] = [
    [true, 842, [], ''],
    [true, 917, ['节点 "ConclaveChallenges" 为空，已跳过'], ''],
    [true, 776, [], ''],
    [false, 30012, [], '请求超时：worldState.php 在 30000ms 内未响应'],
    [true, 1188, ['字段 "SyndicateMissions" 结构变更，已自动兼容', '字段 "FlashSales" 为空'], ''],
    [true, 903, [], ''],
    [true, 861, ['任务 "Sorties" 缺少 Modifier 列表，已使用默认值'], ''],
    [true, 734, [], ''],
    [false, 1204, [], 'HTTP 500：官方导出服务暂时不可用'],
    [true, 987, [], ''],
    [true, 1052, ['节点 "Invasions" 中的 Faction 字段缺失'], ''],
    [true, 812, [], ''],
    [true, 899, [], ''],
    [true, 1340, ['字段 "VoidTrader" 激活时间早于当前时间，已忽略'], ''],
    [false, 20004, [], '连接被重置：TLS 握手失败'],
    [true, 765, [], ''],
    [true, 1088, ['奖励表 "MissionRewards" 新增 3 个未识别项'], ''],
    [true, 923, [], ''],
    [true, 848, [], ''],
    [true, 1101, ['活动 "Events" 剩余时间单位为负，已规整'], ''],
    [true, 790, [], ''],
    [true, 946, [], ''],
    [true, 1023, ['节点 "PersistentEnemies" 为空'], ''],
    [true, 877, [], ''],
  ]
  return samples.map((s, i) => ({
    id: 2400 - i,
    sync_time: now - i * 5 * 60 * 1000,
    is_success: s[0],
    time_spent_ms: s[1],
    warnings: s[2],
    error_msg: s[3],
    nearest_expiry: now + (22 + ((i * 7) % 88)) * 60 * 1000,
  }))
})()

const USERS: SeedUser[] = [
  { id: 1, username: 'admin', role: 'admin', password: 'admin' },
  { id: 2, username: 'maintainer', role: 'maintainer', password: 'maint' },
  { id: 3, username: 'tenno', role: 'normal', password: 'tenno' },
  { id: 4, username: 'orokin_archivist', role: 'maintainer', password: 'orokin' },
  { id: 5, username: 'lotus_liaison', role: 'normal', password: 'lotus' },
  { id: 6, username: 'dax_soldier', role: 'normal', password: 'dax' },
  { id: 7, username: 'cephalon_simaris', role: 'normal', password: 'simaris' },
  { id: 8, username: 'nightwave_cred', role: 'normal', password: 'nightwave' },
  { id: 9, username: 'railjack_engineer', role: 'maintainer', password: 'railjack' },
  { id: 10, username: 'ostron_fisher', role: 'normal', password: 'ostron' },
  { id: 11, username: 'entrati_kalymos', role: 'normal', password: 'kalymos' },
  { id: 12, username: 'zariman_angel', role: 'maintainer', password: 'zariman' },
  { id: 13, username: 'duviri_drifter', role: 'normal', password: 'duviri' },
]
const nextUserId = USERS.reduce((m, u) => Math.max(m, u.id), 0) + 1

const SOM_SEED: SortieModifier[] = [
  { id: 'SOModifierDoubleArmor', name: '敌人护甲强化', description: '敌人的护甲值大幅提高，攻击需要更多的护甲穿透。', vars: {} },
  { id: 'SOModifierDoubleShields', name: '敌人护盾强化', description: '敌人的护盾容量翻倍，护盾恢复速度提升。', vars: {} },
  { id: 'SOModifierEnergyReduction', name: '能量上限减少', description: '战甲的能量上限被削减，技能释放将更加捉襟见肘。', vars: { ENERGY_PERCENT: '50' } },
  { id: 'SOModifierEximus', name: '卓越者大本营', description: '任务中卓越者敌人的生成数量大幅增加。', vars: {} },
  { id: 'SOModifierWeaponRestriction', name: '仅限|WEAPON_TYPE|', description: '任务限定只能使用指定类型的武器完成。', vars: { WEAPON_TYPE: '突击步枪' } },
  { id: 'SOModifierExtremeRadiation', name: '辐射灾害', description: '任务环境中充满辐射，辐射伤害持续侵蚀生命值。', vars: {} },
  { id: 'SOModifierFog', name: '浓雾', description: '浓雾笼罩战场，视野范围被大幅压缩。', vars: {} },
  { id: 'SOModifierEnemyElementalDamage', name: '敌人元素强化', description: '敌人的攻击附加额外元素伤害。', vars: { ELEMENT: '腐蚀' } },
]
const cloneSom = (): SortieModifier[] => SOM_SEED.map((s) => ({ ...s, vars: { ...s.vars } }))
const SOM = cloneSom()

const REP_SEED: Replacement[] = [
  { path: '/RawWorldState/Sorties/0/Modifiers/0', origin: 'SOModifierDoubleArmor', replacement: 'SOModifierDoubleShields', remark: '修正错误的突击修饰符。' },
  { path: '/RawWorldState/VoidTrader/Activation', origin: '2026-09-10T00:00:00Z', replacement: '2026-09-14T00:00:00Z', remark: '手动修正虚空商人抵达时间。' },
  { path: '/RawWorldState/Cycle/Expiry', origin: 'auto', replacement: '2026-09-14T04:20:00Z', remark: '补齐缺失的昼夜循环过期时间。' },
  { path: '/RawWorldState/Invasions/0/Faction', origin: '', replacement: 'Grineer', remark: '官方导出缺少阵营字段，补充为 Grineer。' },
  { path: '/RawWorldState/Sorties/0/MissionType', origin: 'Titania', replacement: 'Omnia Fissure', remark: '修正突击任务名称拼写。' },
  { path: '/RawWorldState/Alerts/0/Reward', origin: 'unknown', replacement: 'Orokin Catalyst Blueprint', remark: '补全缺失的警报奖励。' },
]
const REP = REP_SEED.map((r) => ({ ...r }))

const PASSKEYS: Passkey[] = [
  { id: 1, name: 'Windows Hello · 台式机', created: '2026-08-12' },
  { id: 2, name: 'iPhone · Face ID', created: '2026-07-30' },
  { id: 3, name: 'YubiKey 5C NFC', created: '2026-06-18' },
]
const nextPasskeyId = PASSKEYS.reduce((m, p) => Math.max(m, p.id), 0) + 1

export {
  SYNC_RECORDS,
  USERS,
  nextUserId,
  SOM_SEED,
  SOM,
  REP_SEED,
  REP,
  PASSKEYS,
  nextPasskeyId,
}
