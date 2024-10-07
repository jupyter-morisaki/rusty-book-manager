use anyhow::Result;

use std::env;

pub struct AppConfig {
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub auth: AuthConfig,
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
        let redis = RedisConfig {
            host: env::var("REDIS_HOST")?,
            port: env::var("REDIS_PORT")?.parse()?,
        };
        let auth = AuthConfig {
            ttl: env::var("AUTH_TOKEN_TTL")?.parse()?,
        };

        Ok(Self {
            database,
            redis,
            auth,
        })
    }
}

pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

pub struct RedisConfig {
    pub host: String,
    pub port: u16,
}

pub struct AuthConfig {
    pub ttl: u64,
}
