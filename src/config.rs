use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

#[derive(Debug, Clone)]
pub struct Config {
    pub input: PathBuf,
    pub output: PathBuf,
    pub assets: PathBuf,
    pub wasm: PathBuf,
    pub database: PathBuf,
    pub port: u16,
    pub brand: String,
    pub static_build_only: bool,
    pub flat: bool,
}

impl Config {
    pub fn new(
        input: PathBuf,
        output: PathBuf,
        assets: PathBuf,
        wasm: PathBuf,
        database: PathBuf,
        port: u16,
        brand: String,
        static_build_only: bool,
        flat: bool,
    ) -> Self {
        Config {
            input,
            output,
            assets,
            wasm,
            database,
            port,
            brand,
            static_build_only,
            flat,
        }
    }

    pub fn get() -> &'static Arc<Config> {
        SINGLETON.get().expect("Config not initialized")
    }

    pub fn initialize(config: Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        #[cfg(all(not(test)))]
        {
            ensure_paths_exist(&config.input)?;
            ensure_paths_exist(&config.output)?;
            ensure_paths_exist(&config.assets)?;
            ensure_paths_exist(&config.wasm)?;
            ensure_paths_exist(&config.database)?;
        }

        let _ = SINGLETON
            .set(Arc::new(config))
            .map_err(|_| "Config can only be initialized once");
        Ok(())
    }
}

static SINGLETON: OnceLock<Arc<Config>> = OnceLock::new();

#[cfg(all(not(test)))]
fn ensure_paths_exist(path: &PathBuf) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if !path.exists() {
        return Err(Box::<dyn std::error::Error + Send + Sync>::from(format!(
            "Path does not exist: '{}'. Create it manually!",
            path.display()
        )));
    }
    Ok(())
}
