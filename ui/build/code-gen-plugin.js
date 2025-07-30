import { imageSizeFromFile } from 'image-size/fromFile'

async function buildImageInfoItem(path) {
  const size = await imageSizeFromFile(path)
  return `{
    url: new URL('${path}', import.meta.url).href,
    width: ${size.width},
    height: ${size.height},
  }`
}

async function suit(quality) {
  return `const ${quality}Suit = {
  name: '${quality}',
  frameTop: ${await buildImageInfoItem(`src/assets/mod/${quality}FrameTop.png`)},
  frameBottom:${await buildImageInfoItem(`src/assets/mod/${quality}FrameBottom.png`)},
  background:${await buildImageInfoItem(`src/assets/mod/${quality}Background.png`)},
  sideLight: ${await buildImageInfoItem(`src/assets/mod/${quality}SideLight.png`)},
  cornerLights: ${await buildImageInfoItem(`src/assets/mod/${quality}CornerLights.png`)},
  lowerTab: ${await buildImageInfoItem(`src/assets/mod/${quality}LowerTab.png`)},
  topRight:${await buildImageInfoItem(`src/assets/mod/${quality}TopRightBacker.png`)},
}`
}

async function gen() {
  return `${await suit('Bronze')}

        ${await suit('Silver')}

        ${await suit('Gold')}

        ${await suit('Legendary')}
       export ${await suit('Omega')}

export function getImageSuit(mod) {
  if(mod.baseDrain === 2490417 || mod.tags?.some(it=>it.includes('Riven'))){
  return OmegaSuit;
  }

  if (mod.rarity === 'COMMON') {
    return BronzeSuit
  } else if (mod.rarity === 'UNCOMMON') {
    return SilverSuit
  } else if (mod.rarity === 'RARE') {
    return GoldSuit
  }else if (mod.rarity === 'LEGENDARY') {
    return LegendarySuit
  }
  console.log(mod.rarity, 'no match suit')
  return BronzeSuit
}

export const rankCompleteLine = ${await buildImageInfoItem('src/assets/mod/RankCompleteLine.png')}
`
}

export default function genImageInfo() {
  const virtualModuleId = 'virtual:mod-image-suit'
  const resolvedVirtualModuleId = '__' + virtualModuleId

  return {
    name: 'mod-image-suit-plugin', // 必须的，将会在 warning 和 error 中显示
    resolveId(id) {
      if (id === virtualModuleId) {
        return resolvedVirtualModuleId
      }
    },
    async load(id) {
      if (id === resolvedVirtualModuleId) {
        return await gen()
      }
    },
  }
}
