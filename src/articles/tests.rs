use crate::articles::eval_plugins;
use crate::articles::Article;
use std::path::PathBuf;
use std::time::SystemTime;

#[cfg(test)]
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
            show_source_link: None,
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
            show_source_link: None,
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
            show_source_link: None,
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
            show_source_link: None,
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
            show_source_link: None,
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
            show_source_link: None,
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
            show_source_link: None,
            live_updates: None,
        };

        let article_expected = Article {
            src_file_name: PathBuf::from("example.mdwn"),
            dst_file_name: None,
            article_mdwn_source: None,
            title: None,
            modification_date: Some(SystemTime {
                tv_sec: 1721399580,
                tv_nsec: 0,
            }),
            summary: None,
            tags: None,
            series: None,
            draft: None,
            special_page: None,
            timeline: None,
            anchorjs: None,
            tocify: None,
            show_source_link: None,
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
}
