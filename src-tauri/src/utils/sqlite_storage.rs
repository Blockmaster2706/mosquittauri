//use anyhow::anyhow;
use sqlx;
use sqlx::{
    migrate,
    query,
    //query_as
};
use std::fs::create_dir_all;

use anyhow::{Context, Result};

use crate::model::MsqtDto;

pub type Db = sqlx::SqlitePool;

//Provision and/or connect to Database
pub async fn provision_db(app: &tauri::App) -> Result<Db> {
    let mut db_path = app.path().app_data_dir().context("DB-dir not found")?;

    create_dir_all(&db_path).context("Unable to create dir")?;

    db_path.push("db.sqlite");
    let db_url = format!("sqlite:{}", db_path.display());

    sqlx::Sqlite::create_database(&db_url)
        .await
        .context("Unable to create DB")?;

    let pool = Sqlite::SqlitePoolOptions::new()
        .connect(&db_url)
        .await
        .context("Unable to connect with DB")?;

    migrate!("./migrations")
        .run(&pool)
        .await
        .context("DB-migration failed")?;

    Ok(pool)
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
