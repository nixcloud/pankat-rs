use crate::articles::Article;
use crate::config;
use crate::renderer::html::create_html_from_content_template;
use std::collections::HashMap;
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_html_from_content_template() {
        let config = config::Config::new(
            PathBuf::from("/"),
            PathBuf::from("/"),
            PathBuf::from("documents/assets"),
            PathBuf::from("/"),
            23,
        );
        config::Config::initialize(config).expect("Failed to initialize config");

        let article = Article {
            special_page: Some(true),
            title: Some("Test Article".to_string()),
            src_file_name: "test_src.md".to_string().into(),
            dst_file_name: Some("test_dst.html".to_string().into()),

            article_mdwn_source: None,
            modification_date: None,
            summary: None,
            tags: None,
            series: None,

            draft: None,
            timeline: None,

            anchorjs: None,
            tocify: None,
            show_source_link: None,
            live_updates: None,
        };
        let html_content = "<p>This is a test body.</p>".to_string();

        let result = create_html_from_content_template(article, html_content.clone());

        assert!(result.is_ok());
        let rendered_html = result.unwrap();
        assert!(rendered_html.contains(&html_content));
        assert!(rendered_html.contains("Test Article"));
    }
}
