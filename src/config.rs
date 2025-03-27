use clap::Parser;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

pub enum ConfigValueType {
    Path(Option<std::path::PathBuf>),
    String(Option<String>),
    Number(Option<u16>),
    Bool(Option<bool>),
}

pub struct ConfigValue {
    pub value: ConfigValueType,
    pub is_default: bool,
}

#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input: Option<PathBuf>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output: Option<PathBuf>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub assets: Option<PathBuf>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub wasm: Option<PathBuf>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub database: Option<PathBuf>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub port: Option<u16>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub static_build_only: Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub flat: Option<bool>,
}

#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub input: PathBuf,
    pub output: PathBuf,
    pub assets: PathBuf,
    pub wasm: PathBuf,
    pub database: PathBuf,
    pub brand: String,
    pub port: u16,
    pub static_build_only: bool,
    pub flat: bool,
}

enum config_creation_mode {
    only_default_values,
    only_set_values,
}

fn create_config(
    config_values: &HashMap<String, ConfigValue>,
    creation_mode: config_creation_mode,
) -> CliConfig {
    CliConfig {
        input: match config_values.get("input") {
            Some(cv) => match &cv.value {
                ConfigValueType::Path(p) => match creation_mode {
                    config_creation_mode::only_default_values => {
                        if cv.is_default {
                            p.clone()
                        } else {
                            None
                        }
                    }
                    config_creation_mode::only_set_values => {
                        if cv.is_default {
                            None
                        } else {
                            p.clone()
                        }
                    }
                },
                _ => None,
            },
            None => None,
        },
        output: match config_values.get("output") {
            Some(cv) => match &cv.value {
                ConfigValueType::Path(p) => match creation_mode {
                    config_creation_mode::only_default_values => {
                        if cv.is_default {
                            p.clone()
                        } else {
                            None
                        }
                    }
                    config_creation_mode::only_set_values => {
                        if cv.is_default {
                            None
                        } else {
                            p.clone()
                        }
                    }
                },
                _ => None,
            },
            None => None,
        },
        assets: match config_values.get("assets") {
            Some(cv) => match &cv.value {
                ConfigValueType::Path(p) => match creation_mode {
                    config_creation_mode::only_default_values => {
                        if cv.is_default {
                            p.clone()
                        } else {
                            None
                        }
                    }
                    config_creation_mode::only_set_values => {
                        if cv.is_default {
                            None
                        } else {
                            p.clone()
                        }
                    }
                },
                _ => None,
            },
            None => None,
        },
        wasm: match config_values.get("wasm") {
            Some(cv) => match &cv.value {
                ConfigValueType::Path(p) => match creation_mode {
                    config_creation_mode::only_default_values => {
                        if cv.is_default {
                            p.clone()
                        } else {
                            None
                        }
                    }
                    config_creation_mode::only_set_values => {
                        if cv.is_default {
                            None
                        } else {
                            p.clone()
                        }
                    }
                },
                _ => None,
            },
            None => None,
        },
        database: match config_values.get("database") {
            Some(cv) => match &cv.value {
                ConfigValueType::Path(p) => match creation_mode {
                    config_creation_mode::only_default_values => {
                        if cv.is_default {
                            p.clone()
                        } else {
                            None
                        }
                    }
                    config_creation_mode::only_set_values => {
                        if cv.is_default {
                            None
                        } else {
                            p.clone()
                        }
                    }
                },
                _ => None,
            },
            None => None,
        },
        brand: match config_values.get("brand") {
            Some(cv) => match &cv.value {
                ConfigValueType::String(p) => match creation_mode {
                    config_creation_mode::only_default_values => {
                        if cv.is_default {
                            p.clone()
                        } else {
                            None
                        }
                    }
                    config_creation_mode::only_set_values => {
                        if cv.is_default {
                            None
                        } else {
                            p.clone()
                        }
                    }
                },
                _ => None,
            },
            None => None,
        },
        port: match config_values.get("port") {
            Some(cv) => match &cv.value {
                ConfigValueType::Number(p) => match creation_mode {
                    config_creation_mode::only_default_values => {
                        if cv.is_default {
                            p.clone()
                        } else {
                            None
                        }
                    }
                    config_creation_mode::only_set_values => {
                        if cv.is_default {
                            None
                        } else {
                            p.clone()
                        }
                    }
                },
                _ => None,
            },
            None => None,
        },
        static_build_only: match config_values.get("static_build_only") {
            Some(cv) => match &cv.value {
                ConfigValueType::Bool(p) => match creation_mode {
                    config_creation_mode::only_default_values => {
                        if cv.is_default {
                            p.clone()
                        } else {
                            None
                        }
                    }
                    config_creation_mode::only_set_values => {
                        if cv.is_default {
                            None
                        } else {
                            p.clone()
                        }
                    }
                },
                _ => None,
            },
            None => None,
        },
        flat: match config_values.get("flat") {
            Some(cv) => match &cv.value {
                ConfigValueType::Bool(p) => match creation_mode {
                    config_creation_mode::only_default_values => {
                        if cv.is_default {
                            p.clone()
                        } else {
                            None
                        }
                    }
                    config_creation_mode::only_set_values => {
                        if cv.is_default {
                            None
                        } else {
                            p.clone()
                        }
                    }
                },
                _ => None,
            },
            None => None,
        },
    }
}

impl Config {
    pub fn new(config_values: HashMap<String, ConfigValue>) -> Self {
        let cli_only_default_values_config =
            create_config(&config_values, config_creation_mode::only_default_values);
        let cli_only_set_values_config =
            create_config(&config_values, config_creation_mode::only_set_values);

        let figment_config: Config = Figment::new()
            .merge(Serialized::defaults(cli_only_default_values_config))
            .merge(Toml::file("pankat.toml"))
            .merge(Env::prefixed("PANKAT_"))
            .merge(Serialized::defaults(cli_only_set_values_config))
            .extract()
            .unwrap();
        figment_config
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
