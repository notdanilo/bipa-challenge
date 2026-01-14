use server::prelude::*;

use server::web::WebServer;
use server::importer::Importer;
use server::database::Database;

#[tokio::main]
async fn main() -> Result<()> {
    let database = Database::from_env().await?;
    Importer::run(&database).await?;
    WebServer::from_env()?.run(&database).await?;
    Ok(())
}
