use std::process;
use std::sync::{Arc, Mutex, Once};
use std::fs;

pub struct Config {
    pub input: String,
    pub output: String,
    pub assets: String,
    pub database: String,
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
        input: String,
        output: String,
        assets: String,
        database: String,
        port: u16,
    ) {
        self.init.call_once(|| {
            ensure_paths_exist(input.clone()).unwrap();
            ensure_paths_exist(output.clone()).unwrap();
            ensure_paths_exist(assets.clone()).unwrap();
            ensure_paths_exist(database.clone()).unwrap();

            let config = Config {
                input,
                output,
                assets,
                database,
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

fn ensure_paths_exist(path: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if !fs::metadata(path.clone()).is_ok() {
        return Err(Box::<dyn std::error::Error + Send + Sync>::from(format!(
            "Path does not exist: {:?}",
            path
        )));
    }
    Ok(())
}
