use server::prelude::*;

use server::web::WebServer;
use server::importer::Importer;
use server::database::Database;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("Starting server...");
    
    let database = Database::from_env().await?;
    tracing::info!("Database connected.");

    Importer::run(&database).await?;
    tracing::info!("Importer started.");

    WebServer::from_env()?.run(&database).await?;
    Ok(())
}
