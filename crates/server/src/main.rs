use server::prelude::*;

use server::web::WebServer;
use server::importer::Importer;
use server::database::Database;

use std::future::Future;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("Starting server...");
    
    let database = Database::from_env().await?;
    tracing::info!("Database connected.");

    database.migrate().await?;

    let local = tokio::task::LocalSet::new();
    local.run_until(async move {
        let importer_db = database.clone();
        spawn_supervisor("Importer", move || {
            let db = importer_db.clone();
            async move { Importer::run(db).await }
        });

        let web_server = WebServer::from_env()?;
        let web_db = database.clone();
        spawn_supervisor("WebServer", move || {
            let ws = web_server.clone();
            let db = web_db.clone();
            async move { ws.run(&db).await }
        });

        tokio::signal::ctrl_c().await?;
        tracing::info!("Shutting down...");
        Ok(())
    }).await
}

fn spawn_supervisor<F, Fut>(name: &'static str, factory: F)
where
    F: Fn() -> Fut + Send + 'static,
    Fut: Future<Output = Result<()>> + 'static,
{
    tokio::task::spawn_local(async move {
        loop {
            tracing::info!("Starting {}", name);
            if let Err(e) = factory().await {
                tracing::error!("{} crashed: {:?}", name, e);
            } else {
                tracing::warn!("{} finished unexpectedly", name);
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            tracing::info!("Restarting {}", name);
        }
    });
}
