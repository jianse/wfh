import OSS from 'ali-oss'
import CONFIG from './oss-config.mjs'

import fs from 'node:fs/promises'
import path from 'node:path'
import { createGzip } from 'node:zlib'
import { createReadStream } from 'node:fs'
import { pipeline } from 'node:stream/promises'
import { duplexPair } from 'node:stream'

const DIST_DIR = 'dist'

const store = new OSS(CONFIG)

// const objs = await store.list()

async function walk(root, walkFn) {
  const items = await fs.readdir(root)

  for (const item of items) {
    let itemPath = path.posix.join(root, item)

    const stats = await fs.stat(itemPath)

    if (stats.isDirectory()) {
      walk(itemPath, walkFn)
    } else {
      await walkFn(itemPath)
    }
  }
  // console.log(dir)
}

async function doGzip(path, dest) {
  const gzip = createGzip()
  const source = createReadStream(path)
  // const d
  await pipeline(source, gzip, dest)
}

/**
 *
 * @param {string} path
 */
async function onFile(path) {
  const ossKey = path.replace('dist/', '')
  if (path.startsWith('dist/data/')) {
    console.log('visit path :', path)
    const [rx, tx] = duplexPair() // const dest = new FastDupStream()
    doGzip(path, rx)
    const opts = {}
    if (!path.endsWith('index.txt')) {
      opts.mime = 'application/json'
    }
    opts.headers = {
      'Content-Encoding': 'gzip',
    }
    const r = await store.putStream(ossKey, tx, opts)
    console.log(r.name, r.res.statusMessage)
  } else {
    const r = await store.put(ossKey, path)
    console.log(r.name, r.res.statusMessage)
  }
}

await walk(DIST_DIR, onFile)
