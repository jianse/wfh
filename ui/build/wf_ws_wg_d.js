// another way to get world state
// https://content-zhpt.wf.qq.com/dynamic/worldState.php
import fs from 'node:fs'

const url = 'https://content-zhpt.wf.qq.com/dynamic/worldState.php'

const resp = await fetch(url)

console.log(resp)

const j = await resp.json()

// console.log(j)
j.Tmp1 = JSON.parse(j.Tmp)

fs.writeFileSync('cache/ws_wg_org.json', JSON.stringify(j, null, 2))
