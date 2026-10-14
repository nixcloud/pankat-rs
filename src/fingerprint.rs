//! decides what has to be rebuilt: every stage stores a hash of exactly the inputs it uses
//! and only runs again when that hash changes
//!
//! * plugins: the article file, the generator code and pandoc (`articles.hash`)
//! * pandoc: the markdown after the plugins, pandoc itself and its lua filters (`cache.hash`)
//! * html: the generator code, templates, config and the data of the page (`outputs`)
use crate::config;
use crate::renderer::assets::{Asset, AssetKind};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::sync::OnceLock;

/// hash of the plugin, pandoc and rendering code, crate versions and compiler, see build.rs
pub const GENERATOR: &str = env!("PANKAT_GENERATOR_HASH");

/// sha256 over length prefixed parts, so ("ab", "c") and ("a", "bc") differ
pub struct Fingerprint(Sha256);

impl Fingerprint {
    pub fn new(kind: &str) -> Self {
        Fingerprint(Sha256::new()).add(kind)
    }

    pub fn add(mut self, part: impl AsRef<[u8]>) -> Self {
        let part = part.as_ref();
        self.0.update((part.len() as u64).to_le_bytes());
        self.0.update(part);
        self
    }

    pub fn finish(self) -> String {
        self.0.finalize()[..16]
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }
}

/// the inputs which are the same for all articles, computed once per run
pub struct Inputs {
    pub templates: String,
    pub config: String,
    pub pandoc: String,
    /// e.g. "pandoc 3.6.4 (/nix/store/...-pandoc-3.6.4/bin/pandoc)", for the log
    pub pandoc_description: String,
}

impl Inputs {
    /// part of every article hash, a change runs the plugins (and maybe pandoc) again
    pub fn parse(&self) -> String {
        Fingerprint::new("parse")
            .add(GENERATOR)
            .add(&self.pandoc)
            .finish()
    }

    /// part of every output fingerprint, a change writes all html again
    pub fn render(&self) -> String {
        Fingerprint::new("render")
            .add(GENERATOR)
            .add(&self.templates)
            .add(&self.config)
            .finish()
    }
}

static INPUTS: OnceLock<Inputs> = OnceLock::new();

pub fn inputs() -> &'static Inputs {
    INPUTS.get_or_init(|| {
        let (pandoc_identity, pandoc_description) = pandoc_identity();
        Inputs {
            templates: assets_hash(AssetKind::Template),
            config: config_hash(),
            pandoc: Fingerprint::new("pandoc")
                .add(pandoc_identity)
                .add(assets_hash(AssetKind::PandocFilter))
                .finish(),
            pandoc_description,
        }
    })
}

fn assets_hash(kind: AssetKind) -> String {
    Asset::ALL
        .iter()
        .filter(|asset| asset.kind() == kind)
        .fold(Fingerprint::new("assets"), |fingerprint, asset| {
            fingerprint
                .add(asset.relative_path())
                .add(asset.read().unwrap_or_default())
        })
        .finish()
}

/// the config values which end up in the html, e.g. in links and social previews
fn config_hash() -> String {
    let cfg = config::Config::get();
    Fingerprint::new("config")
        .add(&cfg.site_url)
        .add(cfg.subdir.display().to_string())
        .add([cfg.flat as u8])
        .finish()
}

/// the pandoc binary from PATH, the one which is executed; a nix store path identifies pandoc
/// and all its libraries, elsewhere path, size and modification time of the binary have to do,
/// and `pandoc --version` covers the version and the features
fn pandoc_identity() -> (String, String) {
    let path: Option<PathBuf> = std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths)
            .map(|dir| dir.join("pandoc"))
            .find(|candidate| candidate.is_file())
            .and_then(|candidate| candidate.canonicalize().ok())
    });
    let version: String = std::process::Command::new("pandoc")
        .arg("--version")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
        .unwrap_or_default();
    let binary: String = match &path {
        Some(path) if path.starts_with("/nix/store") => path.display().to_string(),
        Some(path) => {
            let metadata = std::fs::metadata(path).ok();
            format!(
                "{} {:?} {:?}",
                path.display(),
                metadata.as_ref().map(|m| m.len()),
                metadata.and_then(|m| m.modified().ok())
            )
        }
        None => String::new(),
    };
    let description = format!(
        "{} ({})",
        version.lines().next().unwrap_or("pandoc not found"),
        path.map_or("not in PATH".to_string(), |p| p.display().to_string())
    );
    (
        Fingerprint::new("pandoc-binary")
            .add(binary)
            .add(version)
            .finish(),
        description,
    )
}

/// tells in the log why a startup rebuilds more than usual
pub fn report_changes(conn: &mut diesel::SqliteConnection) {
    use crate::db::build_state;
    let inputs = inputs();
    let current = [
        ("generator", GENERATOR, "pankat's generator code changed"),
        ("templates", inputs.templates.as_str(), "templates changed"),
        ("config", inputs.config.as_str(), "site_url, subdir or flat changed"),
        ("pandoc", inputs.pandoc.as_str(), "pandoc or its lua filters changed"),
    ];
    for (key, value, message) in current {
        match build_state::get(conn, key) {
            Some(previous) if previous == value => {}
            Some(_) if key == "pandoc" => {
                let previous = build_state::get(conn, "pandoc_description").unwrap_or_default();
                if previous == inputs.pandoc_description {
                    println!("{} ({})", message, inputs.pandoc_description);
                } else {
                    println!("{}: {} -> {}", message, previous, inputs.pandoc_description);
                }
            }
            Some(_) => println!("{}", message),
            None => {}
        }
        build_state::set(conn, key, value);
    }
    build_state::set(conn, "pandoc_description", &inputs.pandoc_description);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fingerprint() {
        let a = Fingerprint::new("x").add("ab").add("c").finish();
        assert_eq!(a, Fingerprint::new("x").add("ab").add("c").finish());
        assert_ne!(a, Fingerprint::new("x").add("a").add("bc").finish());
        assert_ne!(a, Fingerprint::new("y").add("ab").add("c").finish());
        assert_eq!(a.len(), 32);
    }

    #[test]
    fn test_inputs() {
        crate::test_config::init();
        let inputs = inputs();
        assert_ne!(inputs.templates, assets_hash(AssetKind::PandocFilter));
        assert_ne!(inputs.parse(), inputs.render());
        assert_eq!(inputs.render(), inputs.render());
        assert!(!GENERATOR.is_empty());
    }
}
