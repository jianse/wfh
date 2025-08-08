interface ImageInfoItem {
  url: string
  width: number
  height: number
}

interface ImageSuit {
  name: string
  frameTop: ImageInfoItem
  frameBottom: ImageInfoItem
  background: ImageInfoItem
  sideLight: ImageInfoItem
  cornerLights: ImageInfoItem
  cornerLights: ImageInfoItem
  lowerTab: ImageInfoItem
  topRight: ImageInfoItem
}

declare module 'virtual:mod-image-suit' {
  export function getImageSuit(mod: object): ImageSuit
  export const rankCompleteLine: ImageInfoItem

  export const OmegaSuit: ImageSuit
}
