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

// Methods for Datatypes
/*
pub struct SqliteStorage<T: MsqtDto> {

}

impl<T: MsqtDto> SqliteStorage<T> {
    //try_new
    //update
    pub async fn update(db: &Db, &mut self, action: impl FnOnce(&mut Vec<T>) -> Result<()>) -> Result<Vec<T>, anyhow::Error> {//parse action
        let update = query!(r#UPDATE $1 SET
                                #r, self, action).await.context("Unable to Update");
        Ok(update)
    }
    //find_all
    pub async fn find_all(&self) -> Result<Vec<T>> {
        if self.db.exists() {//skeleton

            Ok(db)
        } else {
            Ok(Vec::new())
        }
    }
    //gen_id
    pub async fn gen_id(&self) -> Result<u32> {
        Ok(Self::gen_id_from_data(&self.find_all()?))//redundant?
    }
    //gen_id_from_data
    pub async fn gen_id_from_data(data: &[T]) -> u32 {
        let id = query!(r#SELECT MAX(id) + 1 FROM $1#r, data).await.context();

        Ok(id)
    }
    //insert
    //edit
    //delete
}
*/
