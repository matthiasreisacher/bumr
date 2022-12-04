#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate async_trait;
extern crate chrono;
extern crate futures;
#[macro_use]
extern crate log;
extern crate r2d2;
#[cfg(feature = "api-rocket")]
#[macro_use]
extern crate rocket;
#[cfg(feature = "api-rocket")]
#[macro_use]
extern crate rocket_contrib;
#[cfg(feature = "cache-sqlite")]
#[macro_use]
extern crate rusqlite;
extern crate serde;
extern crate toml;
extern crate uuid;

use std::rc::Rc;

use anyhow::Result;

use model::session::Session;

use crate::cache::Cache;
use crate::database::Database;
use r2d2::Pool;
use std::ops::Deref;
use crate::config::Config;

// use crate::repository::Repository;

mod api;
mod cache;
mod config;
mod database;
mod model;
mod repository;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let config = config::read_config()?;
    println!("{:?}", config);

    // let manager = cache::init_cache_manager(&config).unwrap();

    // let pool = Pool::builder()
    //     .max_size(8)
    //     .build(manager)
    //     .unwrap();
    //
    // let session = Session::new(&model::generate_id());
    //
    // let cache = pool.get()?;
    let cache: Rc<dyn Cache> = Rc::from(cache::init_cache(&config)?);

    // println!("GET {:?}", cache.session_get(&session.id)?);
    // println!("PUT {:?}", cache.session_put(&session)?);
    // println!("GET {:?}", cache.session_get(&session.id)?);
    // cache.session_delete(&session.id)?;

    // let db: Rc<dyn Database> = Rc::from(database::init(&config));
    // println!("GET {:?}", db.session_get(&session.id).await?);
    // println!("PUT {:?}", db.session_put(&session).await?);
    // println!("GET {:?}", db.session_get(&session.id).await?);
    // db.session_delete(&session.id).await?;

    // let repo = Repository::new(db, cache);

    api::init(&config);
    Ok(())
}
