pub mod mempool;

use crate::prelude::*;
use crate::database::Database;

pub struct Importer;

impl Importer {
    pub async fn run(connection: &Database) -> Result<()> {
        sqlx::migrate!().run(connection.pool()).await?;
        let connection = connection.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
            loop {
                if let Err(_e) = Self::import_routine(&connection).await {
                    // TODO: Do something about it.
                }
                interval.tick().await;
            }
        });
        Ok(())
    }

    async fn import_routine(database: &Database) -> Result<()> {
        let nodes = crate::importer::mempool::Node::get().await?;
        database.upsert_nodes(nodes).await?;
        Ok(())
    }
}
