use server::prelude::*;

use server::WebServer;

#[tokio::main]
async fn main() -> Result<()> {
    WebServer::new("127.0.0.1", 8080).run().await?;

    Ok(())
}
