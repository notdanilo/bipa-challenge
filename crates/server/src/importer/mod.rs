pub mod mempool;

use crate::prelude::*;
use crate::database::Database;

pub struct Importer;

impl Importer {
    pub async fn run(connection: &Database) -> Result<()> {
        tracing::info!("Running database migrations...");
        sqlx::migrate!().run(connection.pool()).await?;
        tracing::info!("Database migrated.");

        let connection = connection.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
            loop {
                tracing::info!("Starting import routine...");
                if let Err(e) = Self::import_routine(&connection).await {
                    tracing::error!("Import routine failed: {:?}", e);
                } else {
                    tracing::info!("Import routine finished successfully.");
                }
                interval.tick().await;
            }
        });
        Ok(())
    }

    async fn import_routine(database: &Database) -> Result<()> {
        let nodes = crate::importer::mempool::Node::get().await?;
        tracing::info!("Fetched {} nodes from mempool", nodes.len());
        database.upsert_nodes(nodes).await?;
        Ok(())
    }
}
