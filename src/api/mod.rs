use crate::config::Config;

#[cfg(feature = "api-rocket")]
mod rocket;

pub fn init(config: &Config) {
    #[cfg(feature = "api-rocket")]
        rocket::init(config);
}