use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

#[derive(Debug, Clone)]
pub struct Config {
    pub input: PathBuf,
    pub output: PathBuf,
    pub assets: PathBuf,
    pub database: PathBuf,
    pub port: u16,
    pub brand: String,
}

impl Config {
    pub fn new(
        input: PathBuf,
        output: PathBuf,
        assets: PathBuf,
        database: PathBuf,
        port: u16,
        brand: String,
    ) -> Self {
        Config {
            input,
            output,
            assets,
            database,
            port,
            brand,
        }
    }

    pub fn get() -> &'static Arc<Config> {
        SINGLETON.get().expect("Config not initialized")
    }

    pub fn initialize(config: Config) -> Result<(), &'static str> {
        #[cfg(not(debug_assertions))] // This attribute ensures the code runs only in release builds
        {
            ensure_paths_exist(&config.input).unwrap();
            ensure_paths_exist(&config.output).unwrap();
            ensure_paths_exist(&config.assets).unwrap();
            ensure_paths_exist(&config.database).unwrap();
        }

        SINGLETON
            .set(Arc::new(config))
            .map_err(|_| "Config can only be initialized once")
    }
}

static SINGLETON: OnceLock<Arc<Config>> = OnceLock::new();

#[cfg(not(debug_assertions))]
fn ensure_paths_exist(path: &PathBuf) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if !path.exists() {
        return Err(Box::<dyn std::error::Error + Send + Sync>::from(format!(
            "Path does not exist: {:?}",
            path
        )));
    }
    Ok(())
}
