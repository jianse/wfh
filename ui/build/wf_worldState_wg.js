import { writeFileSync } from 'node:fs'
import { createHash } from 'node:crypto'

function md5(data) {
  return createHash('md5').update(data).digest('hex')
}

function serializeData(data) {
  let t = ''
  for (const n in data) {
    const r = data[n]
    if ($.isPlainObject(r)) {
      for (const e in r) {
        const o = `${n}[${e}]=${r[e]}`
        t += t === '' ? o : `&${o}`
      }
    } else {
      t += t === '' ? `${n}=${r}` : `&${n}=${r}`
    }
  }
  return t
}

function sign_req(req) {
  if (undefined === (req = req || {}).url) return void console.error('base.ajax need url')

  const method = req.method || 'get'
  if (!req.hasOwnProperty('noSign') || 1 !== req.noSign) {
    let { url: n } = req
    const uuid = ([1e7] + -1e3 + -4e3 + -8e3 + -1e11).replace(/[018]/g, (match) =>
      (match ^ (crypto.getRandomValues(new Uint8Array(1))[0] & (15 >> (match / 4)))).toString(16),
    )
    if ('get' === method.toLowerCase()) {
      let t = ''
      if (req.data && Object.keys(req.data).length != 0) {
        const serializedData = serializeData(req.data)
        if (serializedData) {
          n = n.includes('?') ? `${n}&${serializedData}` : `${n}?${serializedData}`
        }
        req.data = {}
      }
    }
    const params = `uuid=${uuid}&version=2&appid=10003&t=${Math.round(new Date().getTime() / 1e3).toString()}`
    n += n.includes('?') ? `&${params}` : `?${params}`
    const i = 'index.php/'
    let sub_path_and_query = n.split(i)[1],
      sub_path = sub_path_and_query.split('?')[0]
    // remove trailing slash
    if ('/' === sub_path[sub_path.length - 1]) {
      sub_path = sub_path.slice(0, sub_path.length - 1)
      sub_path_and_query = `${sub_path}?${sub_path_and_query.split('?')[1]}`
    }

    const s = md5(`/${decodeURI(sub_path_and_query)}&appkey=2020#$wegame!act$#v2`)
    req.url = `${n}&sign=${s}`
  }
  return req
}

// asserts
{
  // this example is taken from real world request
  const a =
    'xjzj/xjzj20220330/index/ajax_get_worldState?uuid=b3a24b1c-d543-4a56-ac20-24331f4d9cef&version=2&appid=10003&t=1752638525'

  const s = md5(`/${decodeURI(a)}&appkey=2020#$wegame!act$#v2`)
  console.assert(s === '7d05d0a644b1307b085cf20ea67fe505', 'md5 test error')
}

const req = {
  url: 'https://www.wegame.com.cn/api/act/index.php/xjzj/xjzj20220330/index/ajax_get_worldState',
  method: 'get',
  data: {},
}

// maybe we need sync time from server for now it is no need

// const t = await fetch('https://act.wegame.com.cn/service/sys/v1/get_server_time_and_update')
// const t_text = await t.text()

// console.log(t_text)

// const time = t_text.substring(23, 23 + 10)

// console.log(time)
// console.log(new Date().getTime() / 1000)

// window.serverTime = time

const new_req = sign_req(req)
// console.log(new_req)

const headers = {}
// these headers is copy from a real request
// without this headers, the request will be rejected
// may be some of it's headers is not needed but i didn't want to try. so just keep them all
const oh = {
  requestHeaders: {
    headers: [
      {
        name: 'Accept',
        value: 'application/json, text/javascript, */*; q=0.01',
      },
      {
        name: 'Accept-Encoding',
        value: 'gzip, deflate, br, zstd',
      },
      {
        name: 'Accept-Language',
        value: 'zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2',
      },
      {
        name: 'Connection',
        value: 'keep-alive',
      },
      {
        name: 'Host',
        value: 'www.wegame.com.cn',
      },
      {
        name: 'Referer',
        value: 'https://www.wegame.com.cn/act/xjzj/xjzj20220330/index.html?hcfrom=WeGame.helper',
      },
      {
        name: 'Sec-Fetch-Dest',
        value: 'empty',
      },
      {
        name: 'Sec-Fetch-Mode',
        value: 'cors',
      },
      {
        name: 'Sec-Fetch-Site',
        value: 'same-origin',
      },
      {
        name: 'User-Agent',
        value: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:140.0) Gecko/20100101 Firefox/140.0',
      },
      {
        name: 'X-Requested-With',
        value: 'XMLHttpRequest',
      },
    ],
  },
}
oh.requestHeaders.headers.forEach((header) => {
  headers[header.name] = header.value
})

const res = await fetch(new_req.url, {
  method: new_req.method,
  headers,
})

// the response is gb2312 encoded, so we need to decode it
// but i don't know is it always gb2312
// maybe we can use charset to detect it
const gb2312_dec = new TextDecoder('gb2312')
const str = gb2312_dec.decode(await res.arrayBuffer())
// console.log(JSON.parse(str))

const cache_dir = 'cache'
writeFileSync(`${cache_dir}/worldState_wg.json`, str)
