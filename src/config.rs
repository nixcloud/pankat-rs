
use std::path::Path;
use std::process;
use std::sync::{Arc, Mutex, Once};

pub struct Config {
    pub input: std::path::PathBuf,
    pub output: std::path::PathBuf,
    pub assets: std::path::PathBuf,
    pub database: std::path::PathBuf,
    pub port: u16,
}

pub struct Singleton {
    config: Mutex<Option<Arc<Config>>>,
    init: Once,
}

impl Singleton {
    pub fn new() -> Singleton {
        Singleton {
            config: Mutex::new(None),
            init: Once::new(),
        }
    }

    pub fn initialize(
        &self,
        input: &Path,
        output: &Path,
        assets: &Path,
        database: &Path,
        port: u16,
    ) {
        self.init.call_once(|| {
            ensure_paths_exist(&input).unwrap();
            ensure_paths_exist(&output).unwrap();
            ensure_paths_exist(&assets).unwrap();
            ensure_paths_exist(&database).unwrap();

            let config = Config {
                input: input.to_path_buf(),
                output: output.to_path_buf(),
                assets: assets.to_path_buf(),
                database: database.to_path_buf(),
                port,
            };
            *self.config.lock().unwrap() = Some(Arc::new(config));
        });
    }

    pub fn instance(&self) -> Arc<Config> {
        if let Some(ref config) = *self.config.lock().unwrap() {
            Arc::clone(config)
        } else {
            eprintln!("Config has not been initialized.");
            process::exit(1);
        }
    }
}

fn ensure_paths_exist(path: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if !path.exists() {
        return Err(Box::<dyn std::error::Error + Send + Sync>::from(format!(
            "Path does not exist: {:?}",
            path
        )));
    }
    Ok(())
}
