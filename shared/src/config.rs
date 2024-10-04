use anyhow::Result;

use std::env;

pub struct AppConfig {
    pub database: DatabaseConfig,
}

impl AppConfig {
    pub fn build() -> Result<Self> {
        let database = DatabaseConfig {
            host: env::var("DATABASE_HOST")?,
            port: env::var("DATABASE_PORT")?.parse()?,
            username: env::var("DATABASE_USERNAME")?,
            password: env::var("DATABASE_PASSWORD")?,
            database: env::var("DATABASE_NAME")?,
        };

        Ok(Self { database })
    }
}

pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}
