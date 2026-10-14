use crate::config;
use std::path::{Path, PathBuf};

/// every file pankat reads from the assets directory at runtime; the loaders only
/// accept an `Asset`, so a new template has to be added here and is then checked on startup
/// and part of the fingerprints which decide what has to be rebuilt
macro_rules! assets {
    ($($name:ident => $path:literal ($kind:ident),)*) => {
        #[derive(Debug, Clone, Copy)]
        pub enum Asset { $($name,)* }

        impl Asset {
            pub const ALL: &'static [Asset] = &[$(Asset::$name,)*];

            pub fn relative_path(self) -> &'static str {
                match self { $(Asset::$name => $path,)* }
            }

            pub fn kind(self) -> AssetKind {
                match self { $(Asset::$name => AssetKind::$kind,)* }
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetKind {
    /// used when rendering the html of a page
    Template,
    /// used by pandoc, changes require running pandoc again
    PandocFilter,
}

assets! {
    StandaloneTemplate         => "templates/standalone-template.html"           (Template),
    OpenGraphTemplate          => "templates/open-graph-template.html"           (Template),
    ContentTemplate            => "templates/content-template.html"              (Template),
    NavigationArticlesTemplate => "templates/navigation-articles-template.html"  (Template),
    NavigationSeriesTemplate   => "templates/navigation-series-template.html"    (Template),
    MostRecentArticleTemplate  => "templates/most-recent-article.html"           (Template),
    ShiftedNumberedHeadingsLua => "pandoc-lua/shifted-numbered-headings.lua"     (PandocFilter),
    CreateAnchorsLua           => "pandoc-lua/create-anchors.lua"                (PandocFilter),
}

impl Asset {
    pub fn path(self) -> PathBuf {
        config::Config::get().assets.join(self.relative_path())
    }

    pub fn read(self) -> Result<String, String> {
        read_in(self, &config::Config::get().assets)
    }

    /// startup check: all assets exist and the templates are valid handlebars
    pub fn check_all() -> Result<(), String> {
        check_in(&config::Config::get().assets)
    }
}

fn read_in(asset: Asset, dir: &Path) -> Result<String, String> {
    let path = dir.join(asset.relative_path());
    std::fs::read_to_string(&path).map_err(|e| format!("{}: {}", path.display(), e))
}

/// reports all broken assets at once, not only the first one
pub fn check_in(dir: &Path) -> Result<(), String> {
    let errors: Vec<String> = Asset::ALL
        .iter()
        .filter_map(|&asset| {
            let content = match read_in(asset, dir) {
                Ok(content) => content,
                Err(e) => return Some(e),
            };
            if !asset.relative_path().ends_with(".html") {
                return None;
            }
            handlebars::Template::compile(&content)
                .err()
                .map(|e| format!("{}: {}", dir.join(asset.relative_path()).display(), e))
        })
        .collect();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(format!("broken or missing assets:\n  {}", errors.join("\n  ")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assets_in_repo() {
        check_in(Path::new("documents/assets")).unwrap();
    }

    #[test]
    fn test_assets_missing() {
        let dir = std::env::temp_dir().join("pankat-test-assets-missing");
        let err = check_in(&dir).unwrap_err();
        assert_eq!(err.lines().count(), 1 + Asset::ALL.len());
        assert!(err.contains("open-graph-template.html"));
    }

    #[test]
    fn test_assets_invalid_template() {
        let dir = std::env::temp_dir().join("pankat-test-assets-invalid");
        for asset in Asset::ALL {
            let path = dir.join(asset.relative_path());
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, "ok").unwrap();
        }
        std::fs::write(dir.join(Asset::ContentTemplate.relative_path()), "{{#if x}}").unwrap();
        let err = check_in(&dir).unwrap_err();
        assert!(err.contains("content-template.html"));
        assert!(!err.contains("standalone-template.html"));
    }
}
