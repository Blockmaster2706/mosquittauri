//use anyhow::anyhow;
use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    sqlite::SqlitePoolOptions,
    Pool,
    Sqlite, //query_as
};

use anyhow::{Context, Result};

const DB_URL: &str = "sqlite://msqt.sqlite";

//Provision and/or connect to Database
pub async fn provision_db() -> Result<()> {
    if !Sqlite::database_exists(DB_URL)
        .await
        .context("Failed to check if db exists")?
    {
        sqlx::Sqlite::create_database(DB_URL)
            .await
            .context("Failed to create DB")?;
    }

    let crate_dir =
        std::env::var("CARGO_MANIFEST_DIR").context("Failed to get cargo manifest dir")?;
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");

    let pool = connect_db().await?;
    let migrator = Migrator::new(migrations)
        .await
        .context("Failed to create migrator ")?;
    migrator.run(&pool).await?;
    Ok(())
}

pub async fn connect_db() -> Result<Pool<Sqlite>> {
    SqlitePoolOptions::new()
        .connect(DB_URL)
        .await
        .context("Failed to connect with DB")
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
    pub async fn gen_id(&self) -> Result<u64> {
        Ok(Self::gen_id_from_data(&self.find_all()?))//redundant?
    }
    //gen_id_from_data
    pub async fn gen_id_from_data(data: &[T]) -> u64 {
        let id = query!(r#SELECT MAX(id) + 1 FROM $1#r, data).await.context();

        Ok(id)
    }
    //insert
    //edit
    //delete
}
*/
