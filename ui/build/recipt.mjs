import { readFileSync } from 'node:fs'

const f = readFileSync('./public/data/ExportRecipes_zh.json!00_TmZNOp8ApSWIBwA6emKOVg')

const recipes = JSON.parse(f)

const rcs = recipes.ExportRecipes.map((r) => r.resultCategory)

const urcs = new Set(rcs)

console.log(urcs)

// console.log(recipes)
