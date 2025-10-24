use eyre::Result;
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Clone)]
pub struct Db {
    pool: SqlitePool,
}

#[derive(Debug, FromRow)]
struct Agent {
    agent_id: String,
}

impl Db {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn get_all_agent_ids(&self) -> Result<Vec<String>> {
        let agents = sqlx::query_as::<_, Agent>("SELECT agent_id FROM agents")
            .fetch_all(&self.pool)
            .await?;

        let agent_ids = agents.into_iter().map(|a| a.agent_id).collect();

        Ok(agent_ids)
    }
}
