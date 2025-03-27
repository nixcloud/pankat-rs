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
            "brand".to_string(),
            ConfigValue {
                value: config::ConfigValueType::String(Some("".to_string())),
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
            title: Some("Test NewArticle".to_string()),
            modification_date: None,
            summary: None,
            series: None,
            special_page: Some(true),
            draft: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
            tags: None,
        };
        let html_content = "<p>This is a test body.</p>".to_string();

        let result = create_html_from_standalone_template_by_article(article, html_content.clone());

        assert!(result.is_ok());
        let rendered_html = result.unwrap();
        println!("{}", rendered_html);
        assert!(rendered_html.contains(&html_content));
        assert!(rendered_html.contains("Test NewArticle"));
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
