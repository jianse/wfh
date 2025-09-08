use async_compression::{
    Level,
    brotli::EncoderParams,
    tokio::bufread::{BrotliDecoder, BrotliEncoder},
};
use thiserror::Error;
use tokio::io::AsyncReadExt;

#[derive(Error, Debug)]
pub enum CompressError {
    #[error("error while io : {0}")]
    Io(#[from] tokio::io::Error),
}

pub async fn brotli_compress(data: String) -> Result<Vec<u8>, CompressError> {
    let params = EncoderParams::default()
        .quality(Level::Precise(9))
        .size_hint(data.len())
        .text_mode();
    let mut encoder = BrotliEncoder::with_params(data.as_bytes(), params);
    let mut compressed = Vec::new();
    encoder.read_to_end(&mut compressed).await?;
    Ok(compressed)
}

pub async fn brotli_decompress(data: Vec<u8>) -> Result<Vec<u8>, CompressError> {
    let mut decoder = BrotliDecoder::new(data.as_slice());
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).await?;
    Ok(decompressed) // Convert to string
}
