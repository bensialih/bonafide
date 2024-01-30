use deadpool_postgres::{Pool, PoolBuilder};
use deadpool_postgres::{Config, ConfigError};
use postgres::NoTls;
use crate::models::settings::DatabaseSettings;

fn get_db_config(db_settings: DatabaseSettings) -> Result<PoolBuilder, ConfigError>  {
    let mut config = Config::new();
    config.host = Some(db_settings.host.to_string());
    config.user = Some(db_settings.user.to_string());
    config.password = db_settings.password;
    config.dbname = Some(db_settings.dbname.to_string());
    config.port = Some(db_settings.port);

    let mut config = Config::builder(&config, NoTls);
    config
}

pub fn create_pool(db_settings: DatabaseSettings) -> Pool {
    get_db_config(db_settings)
        .unwrap()
        .build()
        .unwrap()
}
