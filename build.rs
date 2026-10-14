//! PANKAT_GENERATOR_HASH: a hash of everything in pankat that shapes the generated html (the
//! plugin, pandoc and rendering code, the crate versions and the compiler). Articles are parsed
//! and their html written again when it changes, changes elsewhere (e.g. handlers.rs) don't.
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

const GENERATOR_SOURCES: &[&str] = &[
    "src/articles",
    "src/renderer",
    "src/db/article.rs",
    "src/db/cache.rs",
    "Cargo.lock",
];

fn collect(path: &Path, files: &mut Vec<PathBuf>) {
    if path.is_dir() {
        for entry in std::fs::read_dir(path).unwrap() {
            collect(&entry.unwrap().path(), files);
        }
    } else if path.file_name().map_or(false, |name| name != "tests.rs") {
        files.push(path.to_path_buf());
    }
}

fn main() {
    let mut files: Vec<PathBuf> = Vec::new();
    for source in GENERATOR_SOURCES {
        println!("cargo:rerun-if-changed={}", source);
        collect(Path::new(source), &mut files);
    }
    files.sort();

    let mut hasher = Sha256::new();
    for file in files {
        let contents = std::fs::read(&file).unwrap();
        hasher.update(file.display().to_string().as_bytes());
        hasher.update((contents.len() as u64).to_le_bytes());
        hasher.update(&contents);
    }
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let rustc_version = std::process::Command::new(rustc)
        .arg("-V")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
        .unwrap_or_default();
    hasher.update(rustc_version.as_bytes());

    let hash: String = hasher.finalize().iter().map(|b| format!("{:02x}", b)).collect();
    println!("cargo:rustc-env=PANKAT_GENERATOR_HASH={}", &hash[..16]);
}
