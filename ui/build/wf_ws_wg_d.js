// another way to get world state
// https://content-zhpt.wf.qq.com/dynamic/worldState.php
import fs from 'node:fs'

const now = new Date()

const timeStr = `${now.getFullYear()}${(now.getMonth() + 1).toString().padStart(2, '0')}${now.getDate().toString().padStart(2, '0')}`

{
  const url = 'https://content-zhpt.wf.qq.com/dynamic/worldState.php'

  const resp = await fetch(url)

  console.log(resp.status)

  const j = await resp.json()

  // console.log(j)
  j.Tmp1 = JSON.parse(j.Tmp)

  fs.writeFileSync(`cache/ws_wg_org-${timeStr}.json`, JSON.stringify(j, null, 2))
}
// warframe stream server
{
  const url = 'https://content.warframe.com/dynamic/worldState.php'

  const resp = await fetch(url)

  console.log(resp.status)

  const j = await resp.json()

  // console.log(j)
  j.Tmp1 = JSON.parse(j.Tmp)

  fs.writeFileSync(`cache/ws_st_org-${timeStr}.json`, JSON.stringify(j, null, 2))
}

// https://content-zhpt.wf.qq.com/dynamic/getProfileViewingData.php?playerId=67ae4d713e435088ea09485d
