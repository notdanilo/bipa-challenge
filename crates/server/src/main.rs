use server::prelude::*;

use server::WebServer;
use server::Importer;
use server::database::Database;

#[tokio::main]
async fn main() -> Result<()> {
    let database = Database::from_env().await?;
    Importer::run(&database).await?;
    WebServer::from_env()?.run(&database).await?;
    Ok(())
}
