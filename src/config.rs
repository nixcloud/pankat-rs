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
    pub jwt_token: Option<String>,
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
    pub jwt_token: String,
    pub port: u16,
    pub static_build_only: bool,
    pub flat: bool,
}

enum OnlyDefaultValues {
    OnlyDefaultValues,
    OnlySetValues,
}

fn create_config(
    config_values: &HashMap<String, ConfigValue>,
    creation_mode: OnlyDefaultValues,
) -> CliConfig {
    CliConfig {
        input: config_values.get("input").and_then(|cv| match &cv.value {
            ConfigValueType::Path(p) => match creation_mode {
                OnlyDefaultValues::OnlyDefaultValues if cv.is_default => p.clone(),
                OnlyDefaultValues::OnlySetValues if !cv.is_default => p.clone(),
                _ => None,
            },
            _ => None,
        }),
        output: config_values.get("output").and_then(|cv| match &cv.value {
            ConfigValueType::Path(p) => match creation_mode {
                OnlyDefaultValues::OnlyDefaultValues if cv.is_default => p.clone(),
                OnlyDefaultValues::OnlySetValues if !cv.is_default => p.clone(),
                _ => None,
            },
            _ => None,
        }),
        assets: config_values.get("assets").and_then(|cv| match &cv.value {
            ConfigValueType::Path(p) => match creation_mode {
                OnlyDefaultValues::OnlyDefaultValues if cv.is_default => p.clone(),
                OnlyDefaultValues::OnlySetValues if !cv.is_default => p.clone(),
                _ => None,
            },
            _ => None,
        }),
        wasm: config_values.get("wasm").and_then(|cv| match &cv.value {
            ConfigValueType::Path(p) => match creation_mode {
                OnlyDefaultValues::OnlyDefaultValues if cv.is_default => p.clone(),
                OnlyDefaultValues::OnlySetValues if !cv.is_default => p.clone(),
                _ => None,
            },
            _ => None,
        }),
        database: config_values.get("database").and_then(|cv| match &cv.value {
            ConfigValueType::Path(p) => match creation_mode {
                OnlyDefaultValues::OnlyDefaultValues if cv.is_default => p.clone(),
                OnlyDefaultValues::OnlySetValues if !cv.is_default => p.clone(),
                _ => None,
            },
            _ => None,
        }),
        brand: config_values.get("brand").and_then(|cv| match &cv.value {
            ConfigValueType::String(p) => match creation_mode {
                OnlyDefaultValues::OnlyDefaultValues if cv.is_default => p.clone(),
                OnlyDefaultValues::OnlySetValues if !cv.is_default => p.clone(),
                _ => None,
            },
            _ => None,
        }),
        jwt_token: config_values.get("jwt_token").and_then(|cv| match &cv.value {
            ConfigValueType::String(p) => match creation_mode {
                OnlyDefaultValues::OnlyDefaultValues if cv.is_default => p.clone(),
                OnlyDefaultValues::OnlySetValues if !cv.is_default => p.clone(),
                _ => None,
            },
            _ => None,
        }),
        port: config_values.get("port").and_then(|cv| match &cv.value {
            ConfigValueType::Number(p) => match creation_mode {
                OnlyDefaultValues::OnlyDefaultValues if cv.is_default => p.clone(),
                OnlyDefaultValues::OnlySetValues if !cv.is_default => p.clone(),
                _ => None,
            },
            _ => None,
        }),
        static_build_only: config_values.get("static_build_only").and_then(|cv| {
            if let ConfigValueType::Bool(p) = &cv.value {
                match creation_mode {
                    OnlyDefaultValues::OnlyDefaultValues if cv.is_default => p.clone(),
                    OnlyDefaultValues::OnlySetValues if !cv.is_default => p.clone(),
                    _ => None,
                }
            } else {
                None
            }
        }),
        flat: config_values.get("flat").and_then(|cv| {
            if let ConfigValueType::Bool(p) = &cv.value {
                match creation_mode {
                    OnlyDefaultValues::OnlyDefaultValues if cv.is_default => p.clone(),
                    OnlyDefaultValues::OnlySetValues if !cv.is_default => p.clone(),
                    _ => None,
                }
            } else {
                None
            }
        }),
    }
}

impl Config {
    pub fn new(config_values: HashMap<String, ConfigValue>) -> Self {
        let cli_only_default_values_config =
            create_config(&config_values, OnlyDefaultValues::OnlyDefaultValues);
        let cli_only_set_values_config =
            create_config(&config_values, OnlyDefaultValues::OnlySetValues);

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
