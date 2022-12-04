use std::collections::HashMap;

use rocket::config::{Environment, Value};
use rocket_contrib::helmet::SpaceHelmet;
use rocket_contrib::serve::StaticFiles;

use crate::api::rocket::connection_pool::{CachePool, DbPool};
use crate::api::rocket::fairings::content_security_policy::ContentSecurityPolicy;
use crate::config::Config;

mod routes;
mod fairings;
mod connection_pool;

/// # Panics
///
/// This function will panic if it rocket cannot be launched.
pub fn init(config: &Config) {
    let rocket_config = rocket::config::Config::build(Environment::Development)
        .extra("databases", create_db_config(config))
        .finalize()
        .unwrap();

    // Initialize and launch server
    let err = rocket::custom(rocket_config)
        .attach(DbPool::fairing())
        .attach(CachePool::fairing())
        // .attach(SessionCreator::default())
        // .attach(TrafficLogger::default())
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .mount("/", routes![routes::test])
        // .mount(
        //     "/users",
        //     routes![
        //         routes::users::index,
        //         routes::users::create,
        //        // routes::users::current
        // ],
        // )
        .attach(SpaceHelmet::default())
        .attach(ContentSecurityPolicy::default())
        .launch();

    panic!("Whoops! Rocket didn't launch! {}", err);
}

fn create_db_config(config: &Config) -> HashMap<&str, Value> {
    let mut databases = HashMap::new();
    let mut database_config = HashMap::new();

    #[cfg(feature = "db-dynamodb")]
        database_config.insert("url",Value::from(config.aws.dynamodb.endpoint_url.to_owned()));
        database_config.insert("name",Value::from(config.aws.region.to_owned()));

    #[cfg(feature = "cache-sqlite")]
        database_config.insert("url",Value::from(config.cache.sqlite.path.to_owned()));

    databases.insert("cache", Value::from(database_config));

    databases
}