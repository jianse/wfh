use async_compression::{Level, brotli::EncoderParams, tokio::write::BrotliEncoder};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

#[tokio::test]
async fn test_brotli_compress() -> eyre::Result<()> {
    let mut inner = File::open("../ui/cache/worldState_wg.json").await?;
    let mut buf = Vec::new();
    inner.read_to_end(&mut buf).await?;
    println!("original size: {}", buf.len());
    let params = EncoderParams::default()
        .quality(Level::Precise(9))
        .text_mode()
        .size_hint(buf.len());

    let mut out_buf = Vec::new();
    let mut compressor = BrotliEncoder::with_params(&mut out_buf, params);

    compressor.write_all(&buf).await?;
    compressor.shutdown().await?;

    println!("compressed size: {}", out_buf.len());

    // compressor.
    Ok(())
}
