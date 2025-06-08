//use anyhow::anyhow;
use sqlx::migrate;
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
    //find_all
    //gen_id
    //gen_id_from_data
    //insert
    //edit
    //delete
}
*/
