import console from 'node:console'
import { readFileSync, writeFileSync } from 'node:fs'
// import {writeFileSync}

const RECIPE_FILE = 'public/data/ExportRecipes_zh.json!00_ECZVW4vmeuBQzuuhf9dh3A'

const s = readFileSync(RECIPE_FILE, 'utf8')

const recipes = JSON.parse(s).ExportRecipes

const wnw = recipes.filter((r) => {
  return r.resultCategory === 'Weapons' && r.ingredients.some((i) => i.category === 'Weapons')
})

writeFileSync('cache/weaponGraph.json', JSON.stringify(wnw, null, 2), 'utf8')

// 需要武器制造的配方
const _wnwNames = wnw.map((r) => r.resultType)

// 武器产物对照表
const ptMap = new Map()

for (const recipe of recipes) {
  if (recipe.resultCategory === 'Weapons') {
    ptMap.set(recipe.resultType, recipe)
  }
}

console.log(ptMap.size)

for (const w of wnw) {
  printNeedsRecursive(w)
  console.log('----')
}

function printNeedsRecursive(w, padLeft = 0) {
  console.log('  '.repeat(padLeft) + w.name)
  for (const weaponNeed of w.ingredients.filter((i) => i.category === 'Weapons')) {
    // console.log('wn', weaponNeed)
    const wn = ptMap.get(weaponNeed.ItemType)
    if (wn) {
      printNeedsRecursive(wn, padLeft + 1)
    }
  }
}
