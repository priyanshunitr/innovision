use std::env;

use anyhow::{Context, Result, bail};

pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub rust_log: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        match dotenvy::dotenv() {
            Ok(_) => {}
            Err(error) if error.not_found() => {}
            // Do not print dotenv parse errors: they can contain secret values.
            Err(_) => bail!("Could not load .env; check its syntax and file permissions"),
        }

        let port = optional_env("PORT", "3000")?
            .parse::<u16>()
            .context("PORT must be an integer between 1 and 65535")?;
        if port == 0 {
            bail!("PORT must be an integer between 1 and 65535");
        }

        let database_url = env::var("DATABASE_URL").map_err(|_| {
            anyhow::anyhow!("DATABASE_URL is required; set it in .env or the environment")
        })?;
        if database_url.trim().is_empty() {
            bail!("DATABASE_URL must not be empty");
        }

        Ok(Self {
            port,
            database_url,
            rust_log: optional_env("RUST_LOG", "info")?,
        })
    }
}

fn optional_env(name: &str, default: &str) -> Result<String> {
    match env::var(name) {
        Ok(value) => Ok(value),
        Err(env::VarError::NotPresent) => Ok(default.to_owned()),
        Err(env::VarError::NotUnicode(_)) => bail!("{name} must contain valid Unicode"),
    }
}
