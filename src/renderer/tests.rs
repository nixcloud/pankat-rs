#[cfg(test)]
mod tests {
    use crate::articles::ArticleWithTags;
    use crate::config;
    use crate::renderer::html::create_html_from_standalone_template_by_article;
    use crate::Config;
    use crate::ConfigValue;
    use std::collections::HashMap;

    fn create_hacky_config() -> Config {
        let mut config_values: HashMap<String, ConfigValue> = HashMap::new();

        config_values.insert(
            "input".to_string(),
            ConfigValue {
                value: config::ConfigValueType::Path(Some("documents/blog.lastlog.de".into())),
                is_default: false,
            },
        );

        config_values.insert(
            "output".to_string(),
            ConfigValue {
                value: config::ConfigValueType::Path(Some("documents/output".into())),
                is_default: false,
            },
        );

        config_values.insert(
            "assets".to_string(),
            ConfigValue {
                value: config::ConfigValueType::Path(Some("documents/assets".into())),
                is_default: false,
            },
        );

        config_values.insert(
            "wasm".to_string(),
            ConfigValue {
                value: config::ConfigValueType::Path(Some("documents/wasm".into())),
                is_default: false,
            },
        );

        config_values.insert(
            "database".to_string(),
            ConfigValue {
                value: config::ConfigValueType::Path(Some("documents".into())),
                is_default: false,
            },
        );

        config_values.insert(
            "site_url".to_string(),
            ConfigValue {
                value: config::ConfigValueType::String(Some("https://example.org/blog".to_string())),
                is_default: false,
            },
        );

        config_values.insert(
            "port".to_string(),
            ConfigValue {
                value: config::ConfigValueType::Number(Some(5000)),
                is_default: false,
            },
        );

        config_values.insert(
            "static".to_string(),
            ConfigValue {
                value: config::ConfigValueType::Bool(Some(false)),
                is_default: false,
            },
        );

        config_values.insert(
            "flat".to_string(),
            ConfigValue {
                value: config::ConfigValueType::Bool(Some(false)),
                is_default: false,
            },
        );

        let config = config::Config::new(config_values);

        config
    }

    #[test]
    fn test_create_html_from_standalone_template() {
        let config = create_hacky_config();
        config::Config::initialize(config).expect("Failed to initialize config");

        let article = ArticleWithTags {
            id: None,
            src_file_name: "documents/blog.lastlog.de/posts/test_src.md".to_string(),
            dst_file_name: "test_dst.html".to_string(),
            hash: "".to_string(),
            title: Some("Test NewArticle".to_string()),
            modification_date: None,
            summary: None,
            series: None,
            special_page: Some(true),
            draft: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
            open_graph_image: None,
            tags: None,
        };
        let html_content = "<p>This is a test body.</p>".to_string();

        let result = create_html_from_standalone_template_by_article(article, html_content.clone());

        assert!(result.is_ok());
        let rendered_html = result.unwrap();
        println!("{}", rendered_html);
        assert!(rendered_html.contains(&html_content));
        assert!(rendered_html.contains("Test NewArticle"));
        // special page: website, no image -> small card
        assert!(rendered_html.contains(r#"<meta property="og:type" content="website" />"#));
        assert!(rendered_html
            .contains(r#"<meta property="og:url" content="https://example.org/blog/test_dst.html" />"#));
        assert!(rendered_html.contains(r#"<meta name="twitter:card" content="summary" />"#));
        assert!(!rendered_html.contains("og:image"));
        assert!(!rendered_html.contains("og:description"));
        assert!(rendered_html.contains(">example.org/blog</a"));

        // regular article with image, summary is escaped in the attribute
        let date_time = chrono::NaiveDateTime::parse_from_str("2026-10-12 12:00", "%Y-%m-%d %H:%M")
            .unwrap();
        let article = ArticleWithTags {
            id: None,
            src_file_name: "documents/blog.lastlog.de/posts/test_src.md".to_string(),
            dst_file_name: "test_dst.html".to_string(),
            hash: "".to_string(),
            title: Some("Test NewArticle".to_string()),
            modification_date: Some(date_time),
            summary: Some(r#"say "hi" & <bye>"#.to_string()),
            series: None,
            special_page: None,
            draft: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
            open_graph_image: Some("posts/libnix/card.png".to_string()),
            tags: Some(vec!["nix".to_string(), "rust".to_string()]),
        };
        let rendered_html =
            create_html_from_standalone_template_by_article(article.clone(), html_content.clone())
                .unwrap();
        assert!(rendered_html.contains(r#"<meta property="og:type" content="article" />"#));
        assert!(rendered_html.contains(
            r#"<meta property="og:image" content="https://example.org/blog/posts/libnix/card.png" />"#
        ));
        assert!(rendered_html
            .contains(r#"<meta name="twitter:card" content="summary_large_image" />"#));
        assert!(rendered_html.contains(r#"content="say &quot;hi&quot; &amp; &lt;bye&gt;""#));
        assert!(rendered_html
            .contains(r#"<meta property="article:published_time" content="2026-10-12T12:00:00" />"#));
        assert!(rendered_html.contains(r#"<meta property="article:tag" content="rust" />"#));

        // drafts get no social preview
        let mut draft = article;
        draft.draft = Some(true);
        let rendered_html =
            create_html_from_standalone_template_by_article(draft, html_content).unwrap();
        assert!(!rendered_html.contains("og:"));
    }

    #[test]
    fn test_date_and_time() {
        use crate::renderer::utils::date_and_time;
        use chrono::NaiveDateTime;
        let date_time_str = "2024-04-12 20:53:00";
        let date_time = NaiveDateTime::parse_from_str(date_time_str, "%Y-%m-%d %H:%M:%S")
            .expect("Failed to parse date time");
        let formatted_date = date_and_time(&Some(date_time));
        assert_eq!(formatted_date, "12 apr 2024");

        let formatted_date_none = date_and_time(&None);
        assert_eq!(formatted_date_none, "");
    }
}
