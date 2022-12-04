use crate::{database, model, Result};
use crate::api::rocket::connection_pool::{CachePool, DbPool};
use crate::cache::Cache;
use crate::database::Database;
use crate::model::session::Session;

#[get("/test")]
pub async fn test(cache_conn: CachePool, db_conn: DbPool) -> Result<String> {
    let cache: &dyn Cache = &*cache_conn;
    let session = Session::new(&model::generate_id());
    println!("GET {:?}", cache.session_get(&session.id)?);
    println!("PUT {:?}", cache.session_put(&session)?);
    println!("GET {:?}", cache.session_get(&session.id)?);
    cache.session_delete(&session.id)?;

    let db: &dyn Database = &*db_conn;
    println!("GET {:?}", db.session_get(&session.id).await?);
    println!("PUT {:?}", db.session_put(&session).await?);
    println!("GET {:?}", db.session_get(&session.id).await?);
    db.session_delete(&session.id).await?;

    Ok("Roger that".to_owned())
}