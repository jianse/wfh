use service::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    service::serve().await
}
