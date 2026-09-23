//! Fetch and decode the in-game language table from the official CDN.
//!
//! The pipeline mirrors what the game client does:
//!
//! 1. download the client index (`index.txt.lzma`) and locate the
//!    `H.Misc_zh.toc` / `H.Misc_zh.cache` bundle,
//! 2. download those two files (xz compressed) and cache them on disk,
//! 3. read `Languages.bin` out of the cache bundle (Oodle / LZ) and decode it.
//!
//! Both the index and the bundle are cached in `cache_dir`, so an unchanged
//! remote is not downloaded again.

use std::collections::HashMap;
use std::io::BufRead;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use indexmap::IndexMap;
use reqwest::Client;
use reqwest::StatusCode;
use reqwest::header::ETAG;
use reqwest::header::IF_NONE_MATCH;

use crate::CachePair;
use crate::decode_languages;
use crate::error::CacheErr;
use crate::error::CacheResult;

/// Base URL of the CN game CDN.
const CDN_BASE: &str = "https://content-zhpt.wf.qq.com";

/// Name of the cache bundle that contains `Languages.bin`.
const LANG_BUNDLE: &str = "H.Misc_zh";

/// Prefix of the bundle inside the client index.
const CLIENT_PREFIX: &str = "/Cache.Windows";

/// Maximum time for a single CDN request. Large cache files need more than the
/// short timeout of the shared client.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(120);

/// Download and decode the language table from the game CDN.
///
/// Returns the decoded `key -> text` map, sorted by key. The `H.Misc_zh.toc` /
/// `H.Misc_zh.cache` bundle (and the client index) are cached inside
/// `cache_dir`.
pub async fn fetch_languages<P: AsRef<Path>>(
    client: &Client,
    cache_dir: P,
) -> CacheResult<IndexMap<String, String>> {
    let cache_dir = cache_dir.as_ref();
    std::fs::create_dir_all(cache_dir)?;

    let toc_file = cache_dir.join(format!("{LANG_BUNDLE}.toc"));
    let cache_file = cache_dir.join(format!("{LANG_BUNDLE}.cache"));
    let etag_file = cache_dir.join("index.etag");

    update_bundle(client, cache_dir, &toc_file, &cache_file, &etag_file).await?;

    decode_bundle(&toc_file, &cache_file)
}

/// Make sure the local `toc`/`cache` files are present and up to date.
async fn update_bundle(
    client: &Client,
    cache_dir: &Path,
    toc_file: &Path,
    cache_file: &Path,
    etag_file: &Path,
) -> CacheResult<()> {
    let have_local = toc_file.exists() && cache_file.exists();
    let known_etag = std::fs::read_to_string(etag_file).ok();
    let known_etag = known_etag
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());

    // the path component is arbitrary; the CDN serves the same index
    let index_url = format!("{CDN_BASE}/origin/{:08X}/index.txt.lzma", rand::random::<u32>());
    let mut req = client.get(index_url).timeout(REQUEST_TIMEOUT);
    if have_local {
        if let Some(etag) = known_etag {
            req = req.header(IF_NONE_MATCH, etag);
        }
    }

    let resp = req.send().await?;

    if have_local && resp.status() == StatusCode::NOT_MODIFIED {
        return Ok(());
    }
    if !resp.status().is_success() {
        return Err(CacheErr::HttpStatus(resp.status().as_u16()));
    }

    let etag = resp
        .headers()
        .get(ETAG)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);
    let body = resp.bytes().await?;

    let manifest = String::from_utf8(xz_decode(body.as_ref())?)
        .map_err(|e| CacheErr::InvalidIndex(e.to_string()))?;
    std::fs::write(cache_dir.join("index.txt"), manifest.as_bytes())?;

    let index = ClientIndex::try_from(manifest.as_str())?;
    download_if_needed(client, &index, &format!("{LANG_BUNDLE}.cache"), cache_file).await?;
    download_if_needed(client, &index, &format!("{LANG_BUNDLE}.toc"), toc_file).await?;

    if let Some(etag) = etag {
        std::fs::write(etag_file, etag)?;
    }
    Ok(())
}

/// Download one bundle file when it is missing or its hash changed.
async fn download_if_needed(
    client: &Client,
    index: &ClientIndex<'_>,
    name: &str,
    target: &Path,
) -> CacheResult<()> {
    let item = index
        .get(&format!("{CLIENT_PREFIX}/{name}"))
        .ok_or_else(|| CacheErr::InvalidIndex(name.to_owned()))?;

    let hash_file = PathBuf::from(format!("{}.hash", target.display()));
    if target.exists() {
        if let Ok(hash) = std::fs::read_to_string(&hash_file) {
            if hash.trim() == item.hash {
                return Ok(());
            }
        }
    }

    let url = format!("{CDN_BASE}{}", item.download_path());
    let body = client
        .get(url)
        .timeout(REQUEST_TIMEOUT)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;

    std::fs::write(target, xz_decode(body.as_ref())?)?;
    std::fs::write(hash_file, item.hash)?;
    Ok(())
}

/// Read `Languages.bin` out of the cache bundle and decode it.
fn decode_bundle(toc_file: &Path, cache_file: &Path) -> CacheResult<IndexMap<String, String>> {
    let mut pair = CachePair::new(toc_file, cache_file);
    let raw = pair.get_data_and_decompress("Languages.bin");
    let mut langs = decode_languages(&mut Cursor::new(raw))?;
    langs.sort_keys();
    Ok(langs)
}

fn xz_decode<R: BufRead>(reader: R) -> CacheResult<Vec<u8>> {
    let mut out = Vec::new();
    liblzma::bufread::XzDecoder::new_multi_decoder(reader).read_to_end(&mut out)?;
    Ok(out)
}

/// Parsed client index (a `path -> entry` map).
struct ClientIndex<'a>(HashMap<&'a str, ClientIndexItem<'a>>);

impl<'a> ClientIndex<'a> {
    fn get(&self, path: &str) -> Option<&ClientIndexItem<'a>> {
        self.0.get(path)
    }
}

impl<'a> TryFrom<&'a str> for ClientIndex<'a> {
    type Error = CacheErr;

    fn try_from(manifest: &'a str) -> Result<Self, Self::Error> {
        let mut map = HashMap::new();
        for line in manifest.lines() {
            let item = parse_index_line(line)?;
            map.insert(item.client_path, item);
        }
        Ok(ClientIndex(map))
    }
}

#[derive(Debug)]
struct ClientIndexItem<'a> {
    client_path: &'a str,
    hash: &'a str,
}

impl ClientIndexItem<'_> {
    fn download_path(&self) -> String {
        format!("{}.{}.lzma", self.client_path, self.hash)
    }
}

/// Parse one index line: `{client_path}.{hash}.lzma,{size}`.
fn parse_index_line(value: &str) -> CacheResult<ClientIndexItem<'_>> {
    let invalid = || CacheErr::InvalidIndex(value.to_owned());

    let (path_and_hash, size) = value.split_once(',').ok_or_else(invalid)?;
    size.parse::<usize>().map_err(|_| invalid())?;

    // strip ".{32-char hash}.lzma"
    const SUFFIX_LEN: usize = 5 + 32 + 1;
    let idx = path_and_hash
        .len()
        .checked_sub(SUFFIX_LEN)
        .ok_or_else(invalid)?;
    let (client_path, suffix) = path_and_hash.split_at(idx);
    let hash = suffix.get(1..33).ok_or_else(invalid)?;

    Ok(ClientIndexItem { client_path, hash })
}
