use std::sync::{Arc, OnceLock};

use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    sqlite::SqlitePoolOptions,
    Pool, Sqlite,
};

use anyhow::{Context, Result};

pub type Db = Arc<Pool<Sqlite>>;

pub const DATABASE_URL: &str = "sqlite://msqt.sqlite";

static MIGRATOR: Migrator = sqlx::migrate!();
pub static POOL: DbOnce = DbOnce::new();

pub struct DbOnce {
    pool: OnceLock<Db>,
}
impl DbOnce {
    pub const fn new() -> Self {
        Self {
            pool: OnceLock::new(),
        }
    }
    pub async fn get(&self) -> Db {
        if let Some(db) = self.pool.get() {
            return db.clone();
        }
        let db = provision_db().await.expect("Failed to provision db");
        self.pool.get_or_init(|| db).clone()
    }
}

//Provision and/or connect to Database
pub async fn provision_db() -> Result<Db> {
    if !Sqlite::database_exists(DATABASE_URL)
        .await
        .context("Failed to check if db exists")?
    {
        Sqlite::create_database(DATABASE_URL)
            .await
            .context("Failed to create DB")?;
    }

    let pool = SqlitePoolOptions::new()
        .connect(DATABASE_URL)
        .await
        .context("Failed to connect with DB")?;
    MIGRATOR
        .run(&pool)
        .await
        .context("Failed to run migrations")?;
    Ok(Arc::new(pool))
}
