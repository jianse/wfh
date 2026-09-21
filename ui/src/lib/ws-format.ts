import type { Reward } from '@/api/world_state'

export function rewardText(r?: Reward | null): string {
  if (!r) return ''
  const l: string[] = []
  if (r.credits) l.push(`${r.credits.toLocaleString('zh-CN')} 星币`)
  if (r.xp) l.push(`${r.xp} 经验值`)
  // if (r.items) l.push(...r.items)
  if (r.itemNames) l.push(...r.itemNames)
  if (r.countedItems) l.push(...r.countedItems.map((i) => `${i.ItemCountSum} × ${i.ItemName}`))
  return l.join(' + ')
}

export const TIER_LABEL: Record<string, string> = {
  VoidT1: '古纪',
  VoidT2: '前纪',
  VoidT3: '中纪',
  VoidT4: '后纪',
  VoidT5: '安魂',
  VoidT6: '全能'
}
export const tierLabel = (t?: string): string => (t ? (TIER_LABEL[t] ?? t) : '')

/* 集团赏金任务名（jobType 路径片段 → 中文任务名） */
const BOUNTY_NAME: Record<string, string> = {
  ReclamationBountyCache: '夺回 · 缓存',
  ReclamationBountyTheft: '夺回 · 盗窃',
  CaptureBountyCapTwo: '捕获',
  SabotageBountySab: '破坏',
  RescueBountyResc: '救援',
  AssassinateBountyAss: '刺杀',
  AttritionBountySab: '消耗',
  AttritionBountyExt: '消耗',
  AttritionBountyLib: '消耗',
  VenusIntelJobSpy: '情报 · 间谍',
  VenusIntelJobRecovery: '情报 · 回收',
  VenusCullJobExterminate: '清剿 · 歼灭',
  VenusCullJobAssassinate: '清剿 · 刺杀',
  VenusChaosJobAssassinate: '混乱 · 刺杀',
  VenusHelpingJobSpy: '协助 · 间谍',
  VenusHelpingJobResource: '协助 · 资源',
  VenusTheftJobExcavation: '盗窃 · 挖掘',
  VenusTheftJobResource: '盗窃 · 资源',
  VenusPreservationJobDefense: '保护 · 防御',
  VenusSpyJobSpy: '间谍',
  VenusArtifactJobAmbush: '神器 · 伏击',
  DeimosKeyPiecesBounty: '夺取钥匙',
  DeimosGrnSurvivorBounty: '幸存者',
  DeimosCrpSurvivorBounty: '幸存者',
  DeimosEndlessExcavateBounty: '无尽挖掘',
  DeimosExcavateBounty: '挖掘',
  DeimosPurifyBounty: '净化',
  DeimosAreaDefenseBounty: '区域防御'
}
export function bountyName(jt?: string): string {
  if (!jt) return '宝库赏金'
  const prefix = jt.includes('Narmer') ? '合一众 · ' : ''
  for (const k in BOUNTY_NAME) if (jt.includes(k)) return prefix + BOUNTY_NAME[k]
  return prefix + '赏金任务'
}
