use std::{env, sync::OnceLock};

#[derive(Debug)]
pub struct Config {
    pub db_url: String,
}

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|e| {
            panic!("could not load config. Error: {e}");
        })
    })
}
impl Config {
    fn load_from_env() -> Result<Self, String> {
        Ok(Config {
            db_url: get_env("DATABASE_URL")?,
        })
    }
}
