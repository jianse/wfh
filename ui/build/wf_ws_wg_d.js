// another way to get world state
// https://content-zhpt.wf.qq.com/dynamic/worldState.php

const url = 'https://content-zhpt.wf.qq.com/dynamic/worldState.php'

const resp = await fetch(url)

console.log(resp)

const j = await resp.json()

console.log(j)
