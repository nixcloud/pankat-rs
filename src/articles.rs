use std::path::PathBuf;
use std::collections::HashMap;

use crate::config;

pub struct Article {
    id: u32,
    src_file_name: PathBuf,
    dst_file_name: String,
    article_mdwn_source: PathBuf,
    title: String,
    modification_date: std::time::SystemTime,
    summary: String,
    tags: Vec<Tag>,
    article_cache: ArticleCache,
    series: String,
    special_page: bool,
    draft: bool,
    anchorjs: bool,
    tocify: bool,
    timeline: bool,
    show_source_link: bool,
    live_updates: bool,
}

pub struct Tag {
    id: u32,
    tag_id: u32,
    name: String,
}

struct ArticleCache {
    id: u32,
    article_cache_id: u32,
    hash: Vec<u8>,
    generated_html: String,
}

struct ArticleItem {
    file: String,
    action: FileAction,
}

enum FileAction {
    New,
    Update,
    Del,
}

pub fn scan_articles() -> HashMap<PathBuf, Article> {
    let mut articles: HashMap<PathBuf, Article> = HashMap::new();
    let cfg = config::Config::get();
    let input_path: PathBuf = cfg.input.clone();

    fn traverse_and_collect_articles(dir: &PathBuf, articles: &mut HashMap<PathBuf, Article>) {
        if dir.is_dir() {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_dir() {
                            traverse_and_collect_articles(&path, articles);
                        } else if let Some(ext) = path.extension() {
                            if ext == "mdwn" {
                                match parse_article(&path) {
                                    Ok(article) => { articles.insert(path, article); },
                                    Err(_) => { /* Handle errors if necessary */ },
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    traverse_and_collect_articles(&input_path, &mut articles);
    println!("articles: {:?}", articles.len());
    articles
}

pub fn parse_article(article_path: &PathBuf) -> Result<Article, String>
    {
        let article: Article = Article {
            id: 0,
            src_file_name: article_path.clone(),
            dst_file_name: String::from("bogus_dst.md"),
            article_mdwn_source: PathBuf::from("bogus/path"),
            title: String::from("Bogus Title"),
            modification_date: std::time::SystemTime::now(),
            summary: String::from("This is a bogus summary."),
            tags: Vec::new(),
            article_cache: ArticleCache {
                id: 0,
                article_cache_id: 0,
                hash: Vec::new(),
                generated_html: String::from("<html>"),
            },
            series: String::from("Bogus Series"),
            special_page: false,
            draft: false,
            anchorjs: false,
            tocify: false,
            timeline: false,
            show_source_link: false,
            live_updates: false,
        };
        Ok(article)
    }

// async fn handle_articles() {
//     let (tx, mut rx) = mpsc::channel::<ArticleItem>(32);
//     let article_queue = Mutex::new(HashMap::new());
//     let is_working = Mutex::new(false);

//     // Worker thread to receive and manage articles in the queue
//     task::spawn(async move {
//         while let Some(article) = rx.recv().await {
//             let mut queue = article_queue.lock().await;
//             let mut working_status = is_working.lock().await;
//             match article.action {
//                 FileAction::New | FileAction::Update => {
//                     queue.entry(article.file.clone()).or_insert(article);
//                     *working_status = true; // Set the status as active because there is work to do
//                 }
//                 FileAction::Del => {
//                     queue.remove(&article.file);
//                     *working_status = true; // Set the status as active because there is work to do
//                 }
//             }
//         }
//     });

//     // Worker thread to process articles
//     task::spawn(async move {
//         loop {
//             let mut working_status = is_working.lock().await;
//             if !*working_status {
//                 tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Sleep if there is no work
//                 continue;
//             }
//             let mut queue = article_queue.lock().await;
//             for (_, article) in queue.drain() {
//                 process_article(article).await;
//             }
//             *working_status = false; // Reset the status to idle after processing is done and the queue is empty
//         }
//     });
// }

async fn process_article(_item: ArticleItem) {
    // Empty implementation
}
