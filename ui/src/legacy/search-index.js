import { WARFRAMES, WEAPONS, MODS, ARCANE, RELICS, RESOURCES, RECIPES } from '@/data/registry'
import { WF_CAT_LABEL } from '@/lib/utils'

/** 复刻原型的全局搜索索引（分类、名称、目标路由）。 */
export const SEARCH_INDEX = [
  ...WARFRAMES.map((w) => [WF_CAT_LABEL[w.cat], w.name, 'warframes']),
  ...WEAPONS.map((w) => ['武器', w.name, 'weapons']),
  ...MODS.map((m) => ['振幅晶体', m.name, 'mods']),
  ...ARCANE.map((a) => ['赋能', a.name, 'arcane']),
  ...RELICS.map((r) => ['遗物', r.name, 'relics']),
  ...RESOURCES.map((r) => ['资源', r.name, 'resources']),
  ...RECIPES.map((r) => ['配方', r.name, 'recipes'])
]

export const WF_CAT_BY_LABEL = { 战甲: 'Suits', 曲翼: 'SpaceSuits', 殁世机甲: 'MechSuits' }

export function searchAll(query) {
  const q = String(query || '').toLowerCase()
  return SEARCH_INDEX.filter(([, name]) => name.toLowerCase().includes(q))
}

export function groupResults(rows) {
  const groups = []
  let last = null
  for (const [cat, name, view] of rows) {
    if (!last || last.cat !== cat) {
      last = { cat, items: [] }
      groups.push(last)
    }
    last.items.push({ cat, name, view })
  }
  return groups
}
