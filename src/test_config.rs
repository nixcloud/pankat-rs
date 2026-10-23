//! one config for all tests, the config is a process wide singleton
use crate::config::{self, Config, ConfigValue, ConfigValueType};
use std::collections::HashMap;

pub const ADMIN_PASSWORD: &str = "test-password";
pub const GITHUB_WEBHOOK_SECRET: &str = "test-webhook-secret";

pub fn init() {
    let mut config_values: HashMap<String, ConfigValue> = HashMap::new();
    let mut set = |name: &str, value: ConfigValueType| {
        config_values.insert(
            name.to_string(),
            ConfigValue {
                value,
                is_default: false,
            },
        );
    };
    set("input", ConfigValueType::Path(Some("documents/blog.lastlog.de".into())));
    set("output", ConfigValueType::Path(Some("documents/output".into())));
    set("assets", ConfigValueType::Path(Some("documents/assets".into())));
    set("wasm", ConfigValueType::Path(Some("documents/wasm".into())));
    set("database", ConfigValueType::Path(Some("documents".into())));
    set("subdir", ConfigValueType::Path(Some("".into())));
    set(
        "site_url",
        ConfigValueType::String(Some("https://example.org/blog".to_string())),
    );
    set("jwt_token", ConfigValueType::String(Some("test-signing-secret".to_string())));
    set("admin_password", ConfigValueType::String(Some(ADMIN_PASSWORD.to_string())));
    set("github_webhook_secret", ConfigValueType::String(Some(GITHUB_WEBHOOK_SECRET.to_string())));
    set("port", ConfigValueType::Number(Some(5000)));
    set("static", ConfigValueType::Bool(Some(false)));
    set("flat", ConfigValueType::Bool(Some(false)));
    // a second call is ignored
    config::Config::initialize(Config::new(config_values)).expect("Failed to initialize config");
}
