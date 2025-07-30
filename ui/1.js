import { readFileSync } from 'node:fs'

// const f = readFileSync('./cache/Weapons.json')
// const d = JSON.parse(f)

// const slotValues = []
// const types = []

// for (const weapon of d.ExportWeapons) {
//   if (!types.includes(weapon.productCategory)) {
//     types.push(weapon.productCategory)
//   }
//   if (!slotValues.includes(weapon.slot)) {
//     slotValues.push(weapon.slot)
//   }
// }

// console.log(slotValues)

// console.log(types)
// for (const type of types) {
//   let total = 0
//   let propCount = {}
//   const cat1 = d.ExportWeapons.filter((weapon) => weapon.productCategory === type)
//   for (const weapon of cat1) {
//     Object.keys(weapon).forEach((key) => {
//       if (propCount[key]) {
//         propCount[key] += 1
//       } else {
//         propCount[key] = 1
//       }
//     })
//     total += 1
//   }
//   console.log(`${type}: ${total}\n`, propCount)
// }
const resStat = false

if (resStat) {
  const f = readFileSync('./cache/Resources.json')

  const d = JSON.parse(f)

  const pnCount = {}

  for (const recipe of d.ExportResources) {
    if (pnCount[recipe.parentName]) {
      pnCount[recipe.parentName] += 1
    } else {
      pnCount[recipe.parentName] = 1
    }
  }
  console.log(pnCount)
}

const upgradesStat = true

if (upgradesStat) {
  const f = readFileSync('./cache/Upgrades.json')

  const d = JSON.parse(f)

  const propCount = {}
  let total = 0

  for (const upgrade of d.ExportUpgrades) {
    for (const key in upgrade) {
      if (propCount[key]) {
        propCount[key] += 1
      } else {
        propCount[key] = 1
      }
    }
    total += 1
  }
  console.log('total=', total, '\n', propCount)

  const polarityCount = {}
  const rarityCount = {}
  const typeCount = {}
  for (const upgrade of d.ExportUpgrades) {
    if (polarityCount[upgrade.polarity]) {
      polarityCount[upgrade.polarity] += 1
    } else {
      polarityCount[upgrade.polarity] = 1
    }
    if (rarityCount[upgrade.rarity]) {
      rarityCount[upgrade.rarity] += 1
    } else {
      rarityCount[upgrade.rarity] = 1
    }
    if (typeCount[upgrade.type]) {
      typeCount[upgrade.type] += 1
    } else {
      typeCount[upgrade.type] = 1
    }
  }
  console.log('polarityCount=', polarityCount)
  console.log('rarityCount=', rarityCount)
  console.log('typeCount=', typeCount)
}

// for (const type of types) {
// }
