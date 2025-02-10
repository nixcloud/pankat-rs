#[cfg(test)]
mod tests {
    use crate::articles::NewArticle;
    use crate::config;
    use crate::renderer::html::{
        create_html_from_content_template, create_html_from_standalone_template,
    };
    use std::path::PathBuf;

    // #[test]
    // fn test_create_html_from_content_template() {
    //     let config = config::Config::new(
    //         PathBuf::from("documents/blog.lastlog.de"),
    //         PathBuf::from("documents/output"),
    //         PathBuf::from("documents/assets"),
    //         PathBuf::from("documents"),
    //         23,
    //     );
    //     config::Config::initialize(config).expect("Failed to initialize config");

    //     let article = NewArticle {
    //         special_page: Some(true),
    //         title: Some("Test NewArticle".to_string()),
    //         src_file_name: "documents/blog.lastlog.de/posts/test_src.md".to_string(),
    //         dst_file_name: "test_dst.html".to_string(),

    //         modification_date: None,
    //         summary: None,
    //         series: None,

    //         draft: None,
    //         timeline: None,

    //         anchorjs: None,
    //         tocify: None,
    //         live_updates: None,
    //     };
    //     let html_content = "<p>This is a test body.</p>".to_string();

    //     let result = create_html_from_content_template(article, html_content.clone());

    //     assert!(result.is_ok());
    //     let rendered_html = result.unwrap();
    //     println!("{}", rendered_html);
    //     assert!(rendered_html.contains(&html_content));
    //     assert!(rendered_html.contains("Test NewArticle"));
    // }

    #[test]
    fn test_create_html_from_standalone_template() {
        let config = config::Config::new(
            PathBuf::from("documents/blog.lastlog.de"),
            PathBuf::from("documents/output"),
            PathBuf::from("documents/assets"),
            PathBuf::from("documents"),
            23,
        );
        config::Config::initialize(config).expect("Failed to initialize config");

        let article = NewArticle {
            src_file_name: "documents/blog.lastlog.de/posts/test_src.md".to_string(),
            dst_file_name: "test_dst.html".to_string(),
            title: Some("Test NewArticle".to_string()),
            modification_date: None,
            summary: None,
            series: None,
            special_page: Some(true),
            draft: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };
        let html_content = "<p>This is a test body.</p>".to_string();

        let result = create_html_from_standalone_template(article, html_content.clone());

        assert!(result.is_ok());
        let rendered_html = result.unwrap();
        println!("{}", rendered_html);
        assert!(rendered_html.contains(&html_content));
        assert!(rendered_html.contains("Test NewArticle"));
    }
}
