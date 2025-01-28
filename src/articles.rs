use tokio::sync::{mpsc, Mutex};
use std::collections::HashMap;
use tokio::task;

struct Article {
    id: u32,
    src_file_name: String,
    dst_file_name: String,
    article_mdwn_source: Vec<u8>,
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

struct Tag {
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