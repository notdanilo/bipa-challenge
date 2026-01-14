use crate::prelude::*;

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::Executor;
use std::{sync::Arc, time::Duration};

#[derive(Clone)]
pub struct Database {
    pool: Arc<SqlitePool>,
}

impl Database {
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }


    pub async fn from_env() -> Result<Self> {
        let url = dotenv::var("DATABASE_URL")?;
        Self::new(&url).await
    }

    pub async fn new(url: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(5))
            .connect(url)
            .await?;

        let pool = Arc::new(pool);

        Ok(Self { pool })
    }

    pub async fn upsert_nodes(&self, nodes: Vec<crate::data::mempool::Node>) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        for node in nodes {
            Self::upsert_node(&mut tx, node).await?;
        }
        tx.commit().await?;
        Ok(())
    }

    pub async fn read_nodes(&self) -> Result<Vec<crate::data::mempool::Node>> {
        let nodes = sqlx::query_as!(
            crate::data::mempool::Node,
            r#"
            SELECT
                public_key as "public_key!",
                alias as "alias!",
                capacity,
                first_seen,
                updated_at
            FROM nodes
            ORDER BY capacity DESC
            "#
        )
        .fetch_all(&*self.pool)
        .await?;
        Ok(nodes)
    }

    async fn upsert_node(tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>, node: crate::data::mempool::Node) -> Result<()> {
        tx.execute(
            sqlx::query!(
                r#"
                INSERT INTO nodes (
                    public_key,
                    alias,
                    capacity,
                    first_seen,
                    updated_at
                )
                VALUES (?, ?, ?, ?, ?)
                ON CONFLICT(public_key) DO UPDATE SET
                    alias = excluded.alias,
                    capacity = excluded.capacity,
                    first_seen = excluded.first_seen,
                    updated_at = excluded.updated_at
                "#,
                node.public_key,
                node.alias,
                node.capacity,
                node.first_seen,
                node.updated_at
            )
        ).await?;
        Ok(())
    }
}
