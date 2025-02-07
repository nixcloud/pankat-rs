#[cfg(test)]
use crate::articles::eval_plugins;
use crate::articles::Article;
use std::path::PathBuf;
use std::time::{Duration, UNIX_EPOCH};

mod tests {
    use super::*;

    #[test]
    fn test_title() {
        let input = "hi!\n[[!title Test Title  ]]\n".to_string();
        let expected_output = "hi!\n\n".to_string();
        let mut article = Article {
            src_file_name: PathBuf::from("example.mdwn"),
            dst_file_name: None,
            article_mdwn_source: None,
            title: None,
            modification_date: None,
            summary: None,
            tags: None,
            series: None,
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let article_expected = Article {
            src_file_name: PathBuf::from("example.mdwn"),
            dst_file_name: None,
            article_mdwn_source: None,
            title: Some("Test Title".to_string()),
            modification_date: None,
            summary: None,
            tags: None,
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

        println!("Article: {:#?}", article);
        assert_eq!(article, article_expected);
    }

    #[test]
    fn test_draft() {
        let input = "hi!\n[[!draft]]\n".to_string();
        let expected_output = "hi!\n\n".to_string();
        let mut article = Article {
            src_file_name: PathBuf::from("example.mdwn"),
            dst_file_name: None,
            article_mdwn_source: None,
            title: None,
            modification_date: None,
            summary: None,
            tags: None,
            series: None,
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let article_expected = Article {
            src_file_name: PathBuf::from("example.mdwn"),
            dst_file_name: None,
            article_mdwn_source: None,
            title: None,
            modification_date: None,
            summary: None,
            tags: None,
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

        println!("Article: {:#?}", article);
        assert_eq!(article, article_expected);
    }

    #[test]
    fn test_specialpage() {
        let input = "hi!\n[[!specialpage]]\n".to_string();
        let expected_output = "hi!\n\n".to_string();
        let mut article = Article {
            src_file_name: PathBuf::from("example.mdwn"),
            dst_file_name: None,
            article_mdwn_source: None,
            title: None,
            modification_date: None,
            summary: None,
            tags: None,
            series: None,
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let article_expected = Article {
            src_file_name: PathBuf::from("example.mdwn"),
            dst_file_name: None,
            article_mdwn_source: None,
            title: None,
            modification_date: None,
            summary: None,
            tags: None,
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

        println!("Article: {:#?}", article);
        println!("article_expected: {:#?}", article_expected);
        assert_eq!(article, article_expected);
    }

    #[test]
    fn test_meta() {
        let input = "hi!\n[[!meta date=\"2024-07-19 14:33\"]]\n".to_string();
        let expected_output = "hi!\n\n".to_string();
        let mut article = Article {
            src_file_name: PathBuf::from("example.mdwn"),
            dst_file_name: None,
            article_mdwn_source: None,
            title: None,
            modification_date: None,
            summary: None,
            tags: None,
            series: None,
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let timestamp = 1721399580;
        let time = UNIX_EPOCH + Duration::from_secs(timestamp as u64);

        let article_expected = Article {
            src_file_name: PathBuf::from("example.mdwn"),
            dst_file_name: None,
            article_mdwn_source: None,
            title: None,
            modification_date: Some(time),
            summary: None,
            tags: None,
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

        println!("Article: {:#?}", article);
        println!("article_expected: {:#?}", article_expected);
        assert_eq!(article, article_expected);
    }

    #[test]
    fn test_series() {
        let input = "hi!\n[[!series   asdf ]]\n".to_string();
        let expected_output = "hi!\n\n".to_string();
        let mut article = Article {
            src_file_name: PathBuf::from("example.mdwn"),
            dst_file_name: None,
            article_mdwn_source: None,
            title: None,
            modification_date: None,
            summary: None,
            tags: None,
            series: None,
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let article_expected = Article {
            src_file_name: PathBuf::from("example.mdwn"),
            dst_file_name: None,
            article_mdwn_source: None,
            title: None,
            modification_date: None,
            summary: None,
            tags: None,
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

        println!("Article: {:#?}", article);
        println!("article_expected: {:#?}", article_expected);
        assert_eq!(article, article_expected);
    }

    #[test]
    fn test_tag() {
        let input = "hi!\n[[!tag   foo bar asdf]]\n".to_string();
        let expected_output = "hi!\n\n".to_string();
        let mut article = Article {
            src_file_name: PathBuf::from("example.mdwn"),
            dst_file_name: None,
            article_mdwn_source: None,
            title: None,
            modification_date: None,
            summary: None,
            tags: None,
            series: None,
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let article_expected = Article {
            src_file_name: PathBuf::from("example.mdwn"),
            dst_file_name: None,
            article_mdwn_source: None,
            title: None,
            modification_date: None,
            summary: None,
            tags: vec!["foo".to_string(), "bar".to_string(), "asdf".to_string()].into(),
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

        println!("Article: {:#?}", article);
        println!("article_expected: {:#?}", article_expected);
        assert_eq!(article, article_expected);
    }

    #[test]
    fn test_summary() {
        let input = "hi!\n[[!summary   foo bar asdf  ]]\n".to_string();
        let expected_output = "hi!\n\n".to_string();
        let mut article = Article {
            src_file_name: PathBuf::from("example.mdwn"),
            dst_file_name: None,
            article_mdwn_source: None,
            title: None,
            modification_date: None,
            summary: None,
            tags: None,
            series: None,
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let article_expected = Article {
            src_file_name: PathBuf::from("example.mdwn"),
            dst_file_name: None,
            article_mdwn_source: None,
            title: None,
            modification_date: None,
            summary: Some("foo bar asdf".to_string()),
            tags: None,
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

        println!("Article: {:#?}", article);
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
        let mut article = Article {
            src_file_name: PathBuf::from("example.mdwn"),
            dst_file_name: None,
            article_mdwn_source: None,
            title: None,
            modification_date: None,
            summary: None,
            tags: None,
            series: None,
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            live_updates: None,
        };

        let article_expected = Article {
            src_file_name: PathBuf::from("example.mdwn"),
            dst_file_name: None,
            article_mdwn_source: None,
            title: None,
            modification_date: None,
            summary: None,
            tags: None,
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

        println!("Article: {:#?}", article);
        println!("article_expected: {:#?}", article_expected);
        assert_eq!(article, article_expected);
    }
}
