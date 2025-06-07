export type ManifestItem = {
  uniqueName: string
  textureLocation: string
}

export function imageUrl(uniqueName: string, mfItems: ManifestItem[]): string {

  const textureLocation = mfItems.find((it) => it.uniqueName === uniqueName)?.textureLocation

  return textureLocation ? `https://content.warframe.com/PublicExport${textureLocation}` : ''
}


// export warframe
