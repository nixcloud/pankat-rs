#[cfg(test)]
mod tests {
    use crate::articles::eval_plugins;
    use crate::articles::ArticleWithTags;

    #[test]
    fn test_title() {
        let input = "hi!\n[[!title Test Title  ]]\n".to_string();
        let expected_output = "hi!\n\n".to_string();
        let mut article = ArticleWithTags {
            id: None,
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            hash: "".to_string(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
            open_graph_image: None,
            tags: None,
        };

        let article_expected = ArticleWithTags {
            id: None,
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            hash: "".to_string(),
            title: Some("Test Title".to_string()),
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
            open_graph_image: None,
            tags: None,
        };

        let result = eval_plugins(&input, &mut article);

        assert!(result.is_ok());

        match result {
            Ok(document) => {
                println!("document: {:?}", document);
                assert_eq!(document, expected_output);
            }
            Err(_) => {}
        }

        println!("ArticleWithTags: {:#?}", article);
        assert_eq!(article, article_expected);
    }

    #[test]
    fn test_draft() {
        let input = "hi!\n[[!draft]]\n".to_string();
        let expected_output = "hi!\n\n".to_string();
        let mut article = ArticleWithTags {
            id: None,
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            hash: "".to_string(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
            open_graph_image: None,
            tags: None,
        };

        let article_expected = ArticleWithTags {
            id: None,
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            hash: "".to_string(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: Some(true),
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
            open_graph_image: None,
            tags: None,
        };

        let result = eval_plugins(&input, &mut article);

        assert!(result.is_ok());

        match result {
            Ok(document) => {
                println!("document: {:?}", document);
                assert_eq!(document, expected_output);
            }
            Err(_) => {}
        }

        println!("ArticleWithTags: {:#?}", article);
        assert_eq!(article, article_expected);
    }

    #[test]
    fn test_specialpage() {
        let input = "hi!\n[[!specialpage]]\n".to_string();
        let expected_output = "hi!\n\n".to_string();
        let mut article = ArticleWithTags {
            id: None,
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            hash: "".to_string(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
            open_graph_image: None,
            tags: None,
        };

        let article_expected = ArticleWithTags {
            id: None,
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            hash: "".to_string(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: Some(true),
            anchorjs: None,
            tocify: None,
            live_updates: None,
            open_graph_image: None,
            tags: None,
        };

        let result = eval_plugins(&input, &mut article);

        assert!(result.is_ok());

        match result {
            Ok(document) => {
                println!("document: {:?}", document);
                assert_eq!(document, expected_output);
            }
            Err(_) => {}
        }

        println!("ArticleWithTags: {:#?}", article);
        println!("article_expected: {:#?}", article_expected);
        assert_eq!(article, article_expected);
    }

    #[test]
    fn test_meta() {
        use chrono::NaiveDateTime;

        let input = "hi!\n[[!meta date=\"2024-07-19 14:33\"]]\n".to_string();
        let expected_output = "hi!\n\n".to_string();
        let mut article = ArticleWithTags {
            id: None,
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            hash: "".to_string(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
            open_graph_image: None,
            tags: None,
        };

        let parsed_time =
            NaiveDateTime::parse_from_str("2024-07-19 14:33", "%Y-%m-%d %H:%M").unwrap();

        let article_expected = ArticleWithTags {
            id: None,
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            hash: "".to_string(),
            title: None,
            modification_date: Some(parsed_time),
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
            open_graph_image: None,
            tags: None,
        };

        let result = eval_plugins(&input, &mut article);

        assert!(result.is_ok());

        match result {
            Ok(document) => {
                println!("document: {:?}", document);
                assert_eq!(document, expected_output);
            }
            Err(_) => {}
        }

        println!("ArticleWithTags: {:#?}", article);
        println!("article_expected: {:#?}", article_expected);
        assert_eq!(article, article_expected);
    }

    #[test]
    fn test_series() {
        let input = "hi!\n[[!series   asdf ]]\n".to_string();
        let expected_output = "hi!\n\n".to_string();
        let mut article = ArticleWithTags {
            id: None,
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            hash: "".to_string(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
            open_graph_image: None,
            tags: None,
        };

        let article_expected = ArticleWithTags {
            id: None,
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            hash: "".to_string(),
            title: None,
            modification_date: None,
            summary: None,
            series: Some("asdf".to_string()),
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
            open_graph_image: None,
            tags: None,
        };

        let result = eval_plugins(&input, &mut article);

        assert!(result.is_ok());

        match result {
            Ok(document) => {
                println!("document: {:?}", document);
                assert_eq!(document, expected_output);
            }
            Err(_) => {}
        }

        println!("ArticleWithTags: {:#?}", article);
        println!("article_expected: {:#?}", article_expected);
        assert_eq!(article, article_expected);
    }

    #[test]
    fn test_open_graph_image() {
        let input = "hi!\n[[!OpenGraphImage posts/libnix/card.png ]]\n".to_string();
        let expected_output = "hi!\n\n".to_string();
        let mut article = ArticleWithTags {
            id: None,
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            hash: "".to_string(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
            open_graph_image: None,
            tags: None,
        };

        let result = eval_plugins(&input, &mut article);
        assert_eq!(result.unwrap(), expected_output);
        assert_eq!(
            article.open_graph_image,
            Some("posts/libnix/card.png".to_string())
        );

        // more than one argument is an error, the plugin text stays in the document
        let mut article2 = article.clone();
        article2.open_graph_image = None;
        let input2 = "[[!OpenGraphImage a.png b.png]]".to_string();
        let result2 = eval_plugins(&input2, &mut article2);
        assert_eq!(result2.unwrap(), input2);
        assert_eq!(article2.open_graph_image, None);
    }

    #[test]
    fn test_tag() {
        let input = "hi!\n[[!tag   foo bar asdf]]\n".to_string();
        let expected_output = "hi!\n\n".to_string();
        let mut article = ArticleWithTags {
            id: None,
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            hash: "".to_string(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
            open_graph_image: None,
            tags: None,
        };

        let article_expected = ArticleWithTags {
            id: None,
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            hash: "".to_string(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
            open_graph_image: None,
            tags: vec!["foo".to_string(), "bar".to_string(), "asdf".to_string()].into(),
        };

        let result = eval_plugins(&input, &mut article);

        assert!(result.is_ok());

        match result {
            Ok(document) => {
                println!("document: {:?}", document);
                assert_eq!(document, expected_output);
            }
            Err(_) => {}
        }

        println!("ArticleWithTags: {:#?}", article);
        println!("article_expected: {:#?}", article_expected);
        assert_eq!(article, article_expected);
    }

    #[test]
    fn test_summary() {
        let input = "hi!\n[[!summary   foo bar asdf  ]]\n".to_string();
        let expected_output = "hi!\n\n".to_string();
        let mut article = ArticleWithTags {
            id: None,
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            hash: "".to_string(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
            open_graph_image: None,
            tags: None,
        };

        let article_expected = ArticleWithTags {
            id: None,
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            hash: "".to_string(),
            title: None,
            modification_date: None,
            summary: Some("foo bar asdf".to_string()),
            series: None,
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
            open_graph_image: None,
            tags: None,
        };

        let result = eval_plugins(&input, &mut article);

        assert!(result.is_ok());

        match result {
            Ok(document) => {
                println!("document: {:?}", document);
                assert_eq!(document, expected_output);
            }
            Err(_) => {}
        }

        println!("ArticleWithTags: {:#?}", article);
        println!("article_expected: {:#?}", article_expected);
        assert_eq!(article, article_expected);
    }

    #[test]
    fn test_img() {
        let input = r#"
        hi!
        [[!img media/nlnet-logo.gif class="noFancy" style="float: right"]]
        abab
        [[!img posts/libnix/Nix_snowflake_windows.svg class="noFancy" style="float: right" width="200px"]]
        "#.to_string();

        let expected_output = r#"
        hi!
        <a href="media/nlnet-logo.gif"><img src="media/nlnet-logo.gif" class="noFancy" style="float: right"></a>
        abab
        <a href="posts/libnix/Nix_snowflake_windows.svg"><img src="posts/libnix/Nix_snowflake_windows.svg" class="noFancy" style="float: right" width="200px"></a>
        "#.to_string();

        //o := `<a href="` + f[1] + `"><img src=` + b + `></a>`
        let mut article = ArticleWithTags {
            id: None,
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            hash: "".to_string(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
            open_graph_image: None,
            tags: None,
        };

        let article_expected = ArticleWithTags {
            id: None,
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            hash: "".to_string(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
            open_graph_image: None,
            tags: None,
        };

        let result = eval_plugins(&input, &mut article);

        assert!(result.is_ok());

        match result {
            Ok(document) => {
                println!("document: {:?}", document);
                assert_eq!(expected_output, document);
            }
            Err(_) => {}
        }

        println!("ArticleWithTags: {:#?}", article);
        println!("article_expected: {:#?}", article_expected);
        assert_eq!(article, article_expected);
    }

    #[test]
    fn test_write_output_skips_unchanged() {
        use crate::articles::write_output_in;
        let mut conn = crate::db::tests::establish_connection_and_initialize_schema();
        let dir = std::env::temp_dir().join("pankat-test-write-output");
        let _ = std::fs::remove_dir_all(&dir);
        let mut renders = 0;
        let mut write = |conn: &mut diesel::SqliteConnection, fingerprint: &str| {
            write_output_in(conn, &dir, "page.html", fingerprint, || {
                renders += 1;
                Some(format!("<p>{}</p>", fingerprint))
            })
        };

        assert!(write(&mut conn, "a"), "first build writes");
        assert!(!write(&mut conn, "a"), "same inputs, nothing to do");
        assert!(write(&mut conn, "b"), "changed inputs write again");
        std::fs::remove_file(dir.join("page.html")).unwrap();
        assert!(write(&mut conn, "b"), "a deleted file is written again");
        assert_eq!(renders, 3, "unchanged pages aren't even rendered");
        assert_eq!(std::fs::read_to_string(dir.join("page.html")).unwrap(), "<p>b</p>");
    }
}
