#[cfg(test)]
mod tests {
    use crate::articles::eval_plugins;
    use crate::articles::NewArticle;
    use std::time::{Duration, UNIX_EPOCH};

    #[test]
    fn test_title() {
        let input = "hi!\n[[!title Test Title  ]]\n".to_string();
        let expected_output = "hi!\n\n".to_string();
        let mut article = NewArticle {
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let article_expected = NewArticle {
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            title: Some("Test Title".to_string()),
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
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

        println!("NewArticle: {:#?}", article);
        assert_eq!(article, article_expected);
    }

    #[test]
    fn test_draft() {
        let input = "hi!\n[[!draft]]\n".to_string();
        let expected_output = "hi!\n\n".to_string();
        let mut article = NewArticle {
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let article_expected = NewArticle {
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: Some(true),
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
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

        println!("NewArticle: {:#?}", article);
        assert_eq!(article, article_expected);
    }

    #[test]
    fn test_specialpage() {
        let input = "hi!\n[[!specialpage]]\n".to_string();
        let expected_output = "hi!\n\n".to_string();
        let mut article = NewArticle {
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let article_expected = NewArticle {
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: Some(true),
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
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

        println!("NewArticle: {:#?}", article);
        println!("article_expected: {:#?}", article_expected);
        assert_eq!(article, article_expected);
    }

    #[test]
    fn test_meta() {
         use chrono::NaiveDateTime;

        let input = "hi!\n[[!meta date=\"2024-07-19 14:33\"]]\n".to_string();
        let expected_output = "hi!\n\n".to_string();
        let mut article = NewArticle {
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let parsed_time = NaiveDateTime::parse_from_str("2024-07-19 14:33", "%Y-%m-%d %H:%M").unwrap();

        let article_expected = NewArticle {
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            title: None,
            modification_date: Some(parsed_time),
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
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

        println!("NewArticle: {:#?}", article);
        println!("article_expected: {:#?}", article_expected);
        assert_eq!(article, article_expected);
    }

    #[test]
    fn test_series() {
        let input = "hi!\n[[!series   asdf ]]\n".to_string();
        let expected_output = "hi!\n\n".to_string();
        let mut article = NewArticle {
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let article_expected = NewArticle {
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            title: None,
            modification_date: None,
            summary: None,
            series: Some("asdf".to_string()),
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
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

        println!("NewArticle: {:#?}", article);
        println!("article_expected: {:#?}", article_expected);
        assert_eq!(article, article_expected);
    }

    #[test]
    fn test_tag() {
        //     let input = "hi!\n[[!tag   foo bar asdf]]\n".to_string();
        //     let expected_output = "hi!\n\n".to_string();
        //     let mut article = NewArticle {
        //         src_file_name: "example.mdwn".to_string(),
        //         dst_file_name: String::new(),
        //         title: None,
        //         modification_date: None,
        //         summary: None,
        //         series: None,
        //         draft: None,
        //         special_page: None,
        //         timeline: None,
        //         anchorjs: None,
        //         tocify: None,
        //         live_updates: None,
        //     };

        //     let article_expected = NewArticle {
        //         src_file_name: "example.mdwn".to_string(),
        //         dst_file_name: String::new(),
        //         title: None,
        //         modification_date: None,
        //         summary: None,
        //         //tags: vec!["foo".to_string(), "bar".to_string(), "asdf".to_string()].into(),
        //         series: None,
        //         draft: None,
        //         special_page: None,
        //         timeline: None,
        //         anchorjs: None,
        //         tocify: None,
        //         live_updates: None,
        //     };

        //     let result = eval_plugins(&input, &mut article);

        //     assert!(result.is_ok());

        //     match result {
        //         Ok(document) => {
        //             println!("document: {:?}", document);
        //             assert_eq!(document, expected_output);
        //         }
        //         Err(_) => {}
        //     }

        //     println!("NewArticle: {:#?}", article);
        //     println!("article_expected: {:#?}", article_expected);
        //     assert_eq!(article, article_expected);
        assert_eq!(1, 2);
    }

    #[test]
    fn test_summary() {
        let input = "hi!\n[[!summary   foo bar asdf  ]]\n".to_string();
        let expected_output = "hi!\n\n".to_string();
        let mut article = NewArticle {
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let article_expected = NewArticle {
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            title: None,
            modification_date: None,
            summary: Some("foo bar asdf".to_string()),
            series: None,
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
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

        println!("NewArticle: {:#?}", article);
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
        let mut article = NewArticle {
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let article_expected = NewArticle {
            src_file_name: "example.mdwn".to_string(),
            dst_file_name: String::new(),
            title: None,
            modification_date: None,
            summary: None,
            series: None,
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
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

        println!("NewArticle: {:#?}", article);
        println!("article_expected: {:#?}", article_expected);
        assert_eq!(article, article_expected);
    }
}
