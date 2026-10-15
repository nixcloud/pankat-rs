use crate::db::article::{
    get_article_with_tags_by_src_file_name, get_prev_and_next_article,
    get_prev_and_next_article_for_series, ArticleNeighbours,
};
use crate::db::cache::{get_cache, set_cache};
use crate::fingerprint::{self, Fingerprint};
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::db::DbPool;
use colored::Colorize;
use notify::EventKind;
use scopeguard::defer;

use regex::Regex;
use serde_json::json;
use std::error::Error;
use std::path::PathBuf;

use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;

mod plugins;
mod tests;
pub mod timeline;
mod utils;

use crate::config;
use crate::renderer::html::{
    create_html_from_content_template, create_html_from_standalone_template_by_article,
    create_html_page, create_index_from_most_recent_article_template,
};
use crate::renderer::pandoc::pandoc_mdwn_2_html;

use self::plugins::{
    draft, hiddentext, img, meta, opengraphimage, series, specialpage, summary, tag, title,
};
use diesel::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArticleWithTags {
    pub id: Option<i32>,
    pub src_file_name: String,
    pub dst_file_name: String,
    pub hash: String,
    pub title: Option<String>,
    pub modification_date: Option<chrono::NaiveDateTime>,
    pub summary: Option<String>,
    pub tags: Option<Vec<String>>,
    pub series: Option<String>,
    pub draft: Option<bool>,
    pub special_page: Option<bool>,
    pub anchorjs: Option<bool>,
    pub tocify: Option<bool>,
    pub live_updates: Option<bool>,
    pub open_graph_image: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Insertable, AsChangeset)]
#[diesel(table_name = crate::db::schema::articles)]
// an update must clear fields whose plugin call was removed from the article, by default
// AsChangeset skips None fields and the old value (e.g. series) would stay in the database
#[diesel(treat_none_as_null = true)]
pub struct NewArticle {
    pub src_file_name: String,
    pub dst_file_name: String,
    pub hash: String,
    pub title: Option<String>,
    pub modification_date: Option<chrono::NaiveDateTime>,
    pub summary: Option<String>,
    pub series: Option<String>,
    pub draft: Option<bool>,
    pub special_page: Option<bool>,
    pub anchorjs: Option<bool>,
    pub tocify: Option<bool>,
    pub live_updates: Option<bool>,
    pub open_graph_image: Option<String>,
}

impl From<ArticleWithTags> for NewArticle {
    fn from(article: ArticleWithTags) -> Self {
        NewArticle {
            src_file_name: article.src_file_name,
            dst_file_name: article.dst_file_name,
            hash: article.hash,
            title: article.title,
            modification_date: article.modification_date,
            summary: article.summary,
            series: article.series,
            draft: article.draft,
            special_page: article.special_page,
            anchorjs: article.anchorjs,
            tocify: article.tocify,
            live_updates: article.live_updates,
            open_graph_image: article.open_graph_image,
        }
    }
}

static PANKAT_FILE: &str = ".pankat_maintained_output_folder";
const GENERATED_PAGES: [&str; 3] = ["index.html", "timeline.html", "404.html"];

fn compute_sha256(file_path: &PathBuf) -> anyhow::Result<String> {
    let mut file = File::open(file_path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 4096];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // EOF reached
        }
        hasher.update(&buffer[..bytes_read]);
    }
    let result = hasher.finalize();
    assert!(!result.is_empty(), "Hash should not be an empty string");
    Ok(format!("{:x}", result))
}

pub fn output_folder_check(output_folder: &PathBuf) -> Result<(), Box<dyn Error>> {
    let output_path_check_file = output_folder.join(PANKAT_FILE);

    if output_path_check_file.exists() {
        return Ok(());
    }

    if std::fs::read_dir(output_folder)?.next().is_none() {
        std::fs::File::create(output_path_check_file)?;
        Ok(())
    } else {
        Err(format!(
            "Output directory contains unexpected files or subdirectories but no '{}' file!",
            PANKAT_FILE
        )
        .into())
    }
}

/// re-renders the articles whose navigation changed (prev/next, series, most recent)
/// and returns their (src_file_name, json command) pairs for the WS clients
fn update_affected_articles(
    conn: &mut SqliteConnection,
    changed_article_id: Option<i32>,
    affected_articles: &std::collections::HashSet<i32>,
) -> Vec<(String, String)> {
    let mut updates: Vec<(String, String)> = Vec::new();
    for id in affected_articles {
        if Some(*id) == changed_article_id {
            continue;
        }
        match crate::db::article::get_article_with_tags_by_id(conn, *id) {
            Ok(Some(affected_article)) => {
                println!(
                    "Updating affected article {} (id: {})",
                    affected_article.dst_file_name, id
                );
                write_article_to_disk(conn, &affected_article);
                if let Some(cache_entry) =
                    get_cache(conn, affected_article.src_file_name.clone())
                {
                    let html: String =
                        create_nav_content_template(conn, &affected_article, cache_entry.html);
                    updates.push((
                        affected_article.src_file_name.clone(),
                        json!({ "update": html, }).to_string(),
                    ));
                }
            }
            Ok(None) => {}
            Err(e) => {
                println!("Error loading affected article (id: {}): {:?}", id, e);
            }
        }
    }
    if let Ok(articles) = crate::db::article::get_visible_articles(conn) {
        crate::articles::timeline::update_timeline(conn, &articles);
    }
    updates
}

/// returns (src_file_name, json command) pairs which are sent via WS to the clients
/// registered for the respective article or an error
pub fn file_monitor_articles_change(
    conn: &mut SqliteConnection,
    event: &crate::file_monitor::PankatFileMonitorEvent,
) -> Result<Vec<(String, String)>, String> {
    println!("-----------> file_monitor_articles_change begin");
    defer! {
        println!("-----------< file_monitor_articles_change end");
    }

    // a rename arrives as Modify(Name(From)) for the old and Modify(Name(To)) for the new
    // path, so whether the file is still there decides between update and removal
    let exists: bool = config::Config::get().input.join(&event.path).is_file();

    match event.kind {
        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) if exists => {
            println!(
                "📝 created / ✏️ modified called on {}",
                event.path.display()
            );
            // tells a published article turning into a draft apart from editing a draft
            let was_draft: Option<bool> =
                get_article_with_tags_by_src_file_name(conn, event.path.display().to_string())
                    .ok()
                    .flatten()
                    .map(|article| article.draft == Some(true));
            let article = parse_article(conn, &event.path).map_err(|e| {
                println!("Error: {:?}", e);
                format!("Error: {:?}", e)
            })?;
            let db_reply = crate::db::article::set(conn, &article).map_err(|e| {
                let error_message = format!("Error: {:?}", e);
                println!("{}", error_message);
                error_message
            })?;
            let is_draft: bool = db_reply.article.draft == Some(true);
            if is_draft {
                remove_from_output(conn, &db_reply.article);
            } else {
                write_article_to_disk(conn, &db_reply.article);
            }
            if db_reply.most_recent_article_change.is_some() {
                update_most_recent_article(conn);
            }
            let mut updates: Vec<(String, String)> =
                update_affected_articles(conn, db_reply.article.id, &db_reply.affected_articles);

            let src_file_name: String = article.src_file_name.clone();
            let dst_file_name: &str = &db_reply.article.dst_file_name;
            match (was_draft, is_draft) {
                (Some(false), true) => {
                    updates.push((
                        src_file_name.clone(),
                        admin_only(json!({ "update": notice(
                            "article turned into a draft",
                            "this article is a draft now, only visible to you at",
                            &format!("drafts/{}", dst_file_name),
                        )})),
                    ));
                    updates.push((
                        src_file_name,
                        public_only(json!({ "update": removed_article_notice(None) })),
                    ));
                }
                (Some(true), false) => {
                    updates.push((
                        src_file_name,
                        json!({ "update": notice(
                            "draft published",
                            "this draft was published, it is now available at",
                            dst_file_name,
                        )})
                        .to_string(),
                    ));
                }
                _ => {
                    let cache_entry = get_cache(conn, src_file_name.clone())
                        .ok_or("Error loading cache for Article".to_string())?;
                    if is_draft {
                        let html = create_draft_content(&db_reply.article, cache_entry.html);
                        updates.push((src_file_name, admin_only(json!({ "update": html }))));
                    } else {
                        let html =
                            create_nav_content_template(conn, &db_reply.article, cache_entry.html);
                        updates.push((src_file_name, json!({ "update": html }).to_string()));
                    }
                }
            }
            Ok(updates)
        }
        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
            println!("🗑️ removed called on {}", event.path.display());
            if let Ok(Some(article)) =
                get_article_with_tags_by_src_file_name(conn, event.path.display().to_string())
            {
                remove_from_output(conn, &article);
            }
            let mut updates: Vec<(String, String)> = Vec::new();
            updates.push((
                event.path.display().to_string(),
                json!({ "update": removed_article_notice(event.renamed_to.as_ref()) }).to_string(),
            ));
            let res =
                crate::db::article::del_by_src_file_name(conn, event.path.display().to_string());
            match res {
                Ok(db_reply) => {
                    if db_reply.most_recent_article_change.is_some() {
                        update_most_recent_article(conn);
                    }
                    updates.extend(update_affected_articles(
                        conn,
                        None,
                        &db_reply.affected_articles,
                    ));
                    Ok(updates)
                }
                // already gone, e.g. the rename target arrived after the removal was processed
                Err(diesel::result::Error::NotFound) => Ok(updates),
                Err(e) => Err(format!("Error: {:?}", e)),
            }
        }
        _ => Err("file_monitor_articles_change: Unknown event type".to_string()),
    }
}

/// drafts and removed articles must not stay in the public output
fn remove_from_output(conn: &mut SqliteConnection, article: &ArticleWithTags) {
    let output_filename: PathBuf = config::Config::get().output.join(&article.dst_file_name);
    match std::fs::remove_file(&output_filename) {
        Err(e) if e.kind() != std::io::ErrorKind::NotFound => {
            println!("Error removing file {}: {}", output_filename.display(), e);
        }
        _ => {}
    }
    crate::db::outputs::delete(conn, &article.dst_file_name);
}

/// live update messages: {"admin": message} reaches only browsers with an admin session,
/// {"public": message} only the others, anything else all of them (see handlers::handle_socket)
pub fn admin_only(message: serde_json::Value) -> String {
    json!({ "admin": message }).to_string()
}

pub fn public_only(message: serde_json::Value) -> String {
    json!({ "public": message }).to_string()
}

/// replaces the content of an article in open browsers, the reader follows the link
fn notice(heading: &str, text: &str, path: &str) -> String {
    let cfg = config::Config::get();
    format!(
        r#"<div class="article">
             <h1 id="SiteTitle">{}</h1>
             <p>{} <a href="{}/{}">{}</a>.</p>
           </div>"#,
        heading,
        text,
        cfg.subdir.display(),
        path,
        path
    )
}

/// for a removed or renamed article, links to the new location or to the blog's index
fn removed_article_notice(renamed_to: Option<&PathBuf>) -> String {
    let cfg = config::Config::get();
    match renamed_to {
        Some(renamed_to) => notice(
            "article moved",
            "this article was renamed, it is now available at",
            &utils::create_dst_file_name(renamed_to, cfg.flat),
        ),
        None => notice(
            "article removed",
            "this article was removed, continue reading at",
            "index.html",
        ),
    }
}

/// a draft is only visible to the admin below drafts/, without prev/next navigation
/// since drafts aren't part of the timeline
fn create_draft_content(article: &ArticleWithTags, html: String) -> String {
    create_html_from_content_template(
        article.clone(),
        html,
        ArticleNeighbours::new(),
        ArticleNeighbours::new(),
    )
    .unwrap()
}

pub fn create_draft_page(conn: &mut SqliteConnection, dst_file_name: &str) -> Option<String> {
    let article = crate::db::article::get_drafts(conn)
        .ok()?
        .into_iter()
        .find(|article| article.dst_file_name == dst_file_name)?;
    let cache_entry = get_cache(conn, article.src_file_name.clone())?;
    let content = create_draft_content(&article, cache_entry.html);
    create_html_from_standalone_template_by_article(article, content).ok()
}

/// one entry of the drafts or special pages list, `path` is relative to subdir
fn article_list_entry(path: &str, article: &ArticleWithTags, note: &str) -> String {
    let cfg = config::Config::get();
    format!(
        r#"<div class="articleListEntry">
             <h3><a href="{}/{}">{}</a> {}</h3>
             <p><span class="articleListDate">{}</span> {}</p>
           </div>"#,
        cfg.subdir.display(),
        path,
        handlebars::html_escape(article.title.as_deref().unwrap_or(&article.dst_file_name)),
        note,
        crate::renderer::utils::date_and_time(&article.modification_date),
        article.summary.as_deref().unwrap_or_default(),
    )
}

pub fn create_drafts_index(conn: &mut SqliteConnection) -> Result<String, Box<dyn Error>> {
    let drafts = crate::db::article::get_drafts(conn)?;
    let mut content = String::from(r#"<h1 id="SiteTitle">drafts</h1>"#);
    if drafts.is_empty() {
        content.push_str("<p>there are no drafts, add <code>[[!draft]]</code> to an article.</p>");
    }
    for draft in drafts {
        content.push_str(&article_list_entry(
            &format!("drafts/{}", draft.dst_file_name),
            &draft,
            "",
        ));
    }
    create_html_page("drafts", &content)
}

/// special pages (about, imprint, ...) aren't in the timeline, so this is the overview
pub fn create_special_pages_index(conn: &mut SqliteConnection) -> Result<String, Box<dyn Error>> {
    let mut special_pages = crate::db::article::get_special_pages(conn)?;
    special_pages.sort_by_key(|article| {
        article
            .title
            .clone()
            .unwrap_or_else(|| article.dst_file_name.clone())
            .to_lowercase()
    });
    let mut content = String::from(r#"<h1 id="SiteTitle">special pages</h1>"#);
    if special_pages.is_empty() {
        content.push_str(
            "<p>there are no special pages, add <code>[[!SpecialPage]]</code> to an article.</p>",
        );
    }
    for page in special_pages {
        content.push_str(&if page.draft == Some(true) {
            article_list_entry(
                &format!("drafts/{}", page.dst_file_name),
                &page,
                r#"<span class="label label-default">draft</span>"#,
            )
        } else {
            article_list_entry(&page.dst_file_name, &page, "")
        });
    }
    create_html_page("special pages", &content)
}

pub fn collect_garbage(pool: &DbPool) {
    let cfg = config::Config::get();
    let input_path: PathBuf = cfg.input.clone();
    let output_path: PathBuf = cfg.output.clone();

    let mut conn = pool
        .get()
        .expect("Failed to get a connection from the pool");

    match crate::db::article::get_all_articles(&mut conn) {
        Ok(articles) => {
            println!("====== Running GC on 'articles table' ======");
            for article in articles.clone() {
                let path = input_path.join(article.src_file_name);
                if !path.exists() {
                    println!("Removing garbage 'article table' entry: {:?}", path);
                    let _ = crate::db::article::del_by_id(&mut conn, article.id.unwrap());
                }
            }
            println!("====== Running GC on 'output' directory ======");
            match output_folder_check(&output_path) {
                Ok(_) => {
                    let lookup_articles_set: std::collections::HashSet<String> = articles
                        .iter()
                        // a stale html of an article turned into a draft must go
                        .filter(|article| article.draft != Some(true))
                        .map(|article| article.dst_file_name.clone())
                        // generated, but no articles; only rewritten when they changed
                        .chain(GENERATED_PAGES.iter().map(|page| page.to_string()))
                        .collect();
                    for entry in std::fs::read_dir(output_path.clone()).unwrap() {
                        let entry = entry.unwrap();
                        let relative_entry = entry
                            .path()
                            .strip_prefix(&output_path)
                            .unwrap()
                            .to_path_buf();
                        let relative_entry_string: String = relative_entry.display().to_string();
                        if relative_entry_string == PANKAT_FILE {
                            continue;
                        }
                        if entry.file_type().unwrap().is_dir() {
                            let w = "WARNING: ignoring path, needs to be implemented!".red();
                            println!("{w}");
                            continue;
                        }
                        if !lookup_articles_set.contains(relative_entry_string.as_str()) {
                            println!("Removing garbage 'output' entry: {:?}", relative_entry);
                            std::fs::remove_file(entry.path()).unwrap();
                            crate::db::outputs::delete(&mut conn, &relative_entry_string);
                        }
                    }
                }
                Err(e) => {
                    println!(
                        "Warning! Not doing any GC of output dir '{}': {}",
                        output_path.display(),
                        e
                    );
                }
            }
        }
        Err(_) => {}
    };

    match crate::db::cache::get_cache_src_file_names(&mut conn) {
        Ok(entries) => {
            println!("====== Running GC on 'cache table' ======");
            for (id, path) in entries {
                let path = input_path.join(path);
                if !path.exists() {
                    println!("Removing garbage 'cache table' entry: {:?}", path);
                    let _ = crate::db::cache::del_cache_by_id(&mut conn, id.unwrap());
                }
            }
        }
        Err(_) => {}
    };
}

pub fn scan_articles(pool: &DbPool) {
    let cfg = config::Config::get();
    let input_path: PathBuf = cfg.input.clone();

    let mut conn = pool
        .get()
        .expect("Failed to get a connection from the pool");

    let start_time = std::time::Instant::now();

    println!("====== Parsing input for mdwn documents ======");

    fn traverse_and_collect_articles(
        conn: &mut SqliteConnection,
        dir: &PathBuf,
        input_path: &PathBuf,
    ) {
        if dir.is_dir() {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        //println!("{}", path.clone().display());
                        if path.is_dir() {
                            traverse_and_collect_articles(conn, &path, &input_path);
                        } else if let Some(ext) = path.extension() {
                            if ext == "mdwn" {
                                let article_path: PathBuf =
                                    path.strip_prefix(input_path).unwrap().to_path_buf();
                                STATS.scanned.fetch_add(1, Ordering::Relaxed);
                                match parse_article(conn, &article_path) {
                                    Ok(article) => {
                                        //println!("Parsed article: {:#?}", article);
                                        let _ = crate::db::article::set(conn, &article);
                                    }
                                    Err(_) => { /* Handle errors if necessary */ }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    traverse_and_collect_articles(&mut conn, &input_path, &input_path);

    let duration = start_time.elapsed();
    println!("Time to scan input for articles: {:?}", duration);
}

pub fn build_articles(pool: &DbPool) {
    let mut conn = pool
        .get()
        .expect("Failed to get a connection from the pool");

    let start_time = std::time::Instant::now();

    match crate::db::article::get_visible_articles(&mut conn) {
        Ok(articles) => {
            crate::articles::timeline::update_timeline(&mut conn, &articles);

            for article in articles {
                write_article_to_disk(&mut conn, &article);
            }
        }
        Err(_) => { /* Handle errors if necessary */ }
    }

    update_special_pages(&mut conn);
    update_most_recent_article(&mut conn);
    // for nginx (error_page 404), pankat renders it on request
    let fingerprint: String = Fingerprint::new("404")
        .add(fingerprint::inputs().render())
        .finish();
    write_output(&mut conn, "404.html", &fingerprint, || {
        Some(crate::renderer::html::create_not_found_page())
    });

    let duration = start_time.elapsed();
    println!("Time to build articles: {:?}", duration);
    let scanned = STATS.scanned.load(Ordering::Relaxed);
    let parsed = STATS.parsed.load(Ordering::Relaxed);
    println!(
        "articles: {} parsed ({} pandoc runs), {} unchanged | output: {} written, {} unchanged",
        parsed,
        STATS.pandoc.load(Ordering::Relaxed),
        scanned.saturating_sub(parsed),
        STATS.written.load(Ordering::Relaxed),
        STATS.unchanged.load(Ordering::Relaxed),
    );
}

pub fn update_special_pages(conn: &mut SqliteConnection) {
    match crate::db::article::get_special_pages(conn) {
        Ok(special_pages) => {
            // a special page can be a draft as well, those stay out of the public output
            for article in special_pages.into_iter().filter(|a| a.draft != Some(true)) {
                write_article_to_disk(conn, &article);
            }
        }
        Err(_) => { /* Handle errors if necessary */ }
    }
}

pub fn update_most_recent_article(conn: &mut SqliteConnection) {
    match crate::db::article::get_most_recent_article(conn) {
        Ok(article_option) => match article_option {
            Some(article) => {
                // the index redirects to the most recent article and carries its social preview
                let fingerprint: String = Fingerprint::new("index")
                    .add(fingerprint::inputs().render())
                    .add(page_data(&article))
                    .finish();
                write_output(conn, "index.html", &fingerprint, || {
                    create_index_from_most_recent_article_template(&article)
                        .map_err(|e| println!("Error: {}", e))
                        .ok()
                });
            }
            None => {}
        },
        Err(_) => {}
    };
}

fn neighbours(conn: &mut SqliteConnection, article_id: i32) -> (ArticleNeighbours, ArticleNeighbours) {
    (
        get_prev_and_next_article(conn, article_id).unwrap_or_else(|_| ArticleNeighbours::new()),
        get_prev_and_next_article_for_series(conn, article_id)
            .unwrap_or_else(|_| ArticleNeighbours::new()),
    )
}

fn create_nav_content_template(
    conn: &mut SqliteConnection,
    article: &ArticleWithTags,
    html: String,
) -> String {
    let (article_neighbours, article_series_neighbours) = neighbours(conn, article.id.unwrap());
    create_html_from_content_template(
        article.clone(),
        html,
        article_neighbours,
        article_series_neighbours,
    )
    .unwrap()
}

/// the data of an article which ends up on its pages, for the output fingerprints; without
/// `hash`, which only tells the parse stage whether to run again
pub(crate) fn page_data(article: &ArticleWithTags) -> String {
    format!(
        "{:?}",
        ArticleWithTags {
            hash: String::new(),
            ..article.clone()
        }
    )
}

/// the navigation only links the neighbours, so only their names matter
fn neighbour_names(neighbours: &ArticleNeighbours) -> [Option<&str>; 2] {
    [&neighbours.prev, &neighbours.next].map(|n| n.as_ref().map(|a| a.dst_file_name.as_str()))
}

fn write_article_to_disk(conn: &mut SqliteConnection, article: &ArticleWithTags) {
    let Some(cache_entry) = get_cache(conn, article.src_file_name.clone()) else {
        println!("Error retrieving cache for path: {}", &article.src_file_name);
        return;
    };
    let (article_neighbours, article_series_neighbours) = neighbours(conn, article.id.unwrap());
    let fingerprint: String = Fingerprint::new("article-page")
        .add(fingerprint::inputs().render())
        .add(page_data(article))
        .add(&cache_entry.html)
        .add(format!(
            "{:?} {:?}",
            neighbour_names(&article_neighbours),
            neighbour_names(&article_series_neighbours)
        ))
        .finish();
    write_output(conn, &article.dst_file_name, &fingerprint, || {
        let content = create_html_from_content_template(
            article.clone(),
            cache_entry.html,
            article_neighbours,
            article_series_neighbours,
        )
        .ok()?;
        create_html_from_standalone_template_by_article(article.clone(), content).ok()
    });
}

/// what a scan and build did, for the summary in the log
struct Stats {
    scanned: AtomicUsize,
    parsed: AtomicUsize,
    pandoc: AtomicUsize,
    written: AtomicUsize,
    unchanged: AtomicUsize,
}

static STATS: Stats = Stats {
    scanned: AtomicUsize::new(0),
    parsed: AtomicUsize::new(0),
    pandoc: AtomicUsize::new(0),
    written: AtomicUsize::new(0),
    unchanged: AtomicUsize::new(0),
};

/// the only way generated pages get into the output: `fingerprint` covers every input of
/// `render`, so with an unchanged fingerprint and an existing file there's nothing to do
pub(crate) fn write_output(
    conn: &mut SqliteConnection,
    dst_file_name: &str,
    fingerprint: &str,
    render: impl FnOnce() -> Option<String>,
) {
    let output: PathBuf = config::Config::get().output.clone();
    write_output_in(conn, &output, dst_file_name, fingerprint, render);
}

fn write_output_in(
    conn: &mut SqliteConnection,
    output: &std::path::Path,
    dst_file_name: &str,
    fingerprint: &str,
    render: impl FnOnce() -> Option<String>,
) -> bool {
    let path: PathBuf = output.join(dst_file_name);
    if path.is_file() && crate::db::outputs::get(conn, dst_file_name).as_deref() == Some(fingerprint)
    {
        STATS.unchanged.fetch_add(1, Ordering::Relaxed);
        return false;
    }
    let Some(html) = render() else {
        println!("Error rendering {}", dst_file_name);
        return false;
    };
    println!("Writing {}", dst_file_name);
    write_to_disk(&html, &path);
    crate::db::outputs::set(conn, dst_file_name, fingerprint);
    STATS.written.fetch_add(1, Ordering::Relaxed);
    true
}

pub fn write_to_disk(content: &String, filepath: &PathBuf) {
    if let Some(parent) = filepath.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            eprintln!("Failed to create directory {:?}: {}", parent, e);
        }
    }
    std::fs::write(filepath, content.as_str()).expect("Unable to write HTML file");
}

fn parse_article(
    conn: &mut SqliteConnection,
    article_path: &PathBuf,
) -> Result<ArticleWithTags, Box<dyn Error>> {
    let src_file_name_string = article_path.display().to_string();
    let cfg = config::Config::get();
    let input_path: PathBuf = cfg.input.clone();

    let article_input_path: PathBuf = cfg.input.join(article_path);
    // the plugins run again when the file, the generator code or pandoc changed
    let hash: String = Fingerprint::new("article")
        .add(compute_sha256(&article_input_path).unwrap())
        .add(fingerprint::inputs().parse())
        .finish();
    let existing_article: Result<Option<ArticleWithTags>, diesel::result::Error> =
        get_article_with_tags_by_src_file_name(conn, article_path.display().to_string());

    if let Ok(existing_article) = existing_article {
        if let Some(article_with_tags) = existing_article {
            if article_with_tags.hash == hash {
                return Ok(article_with_tags);
            }
        }
    }
    println!("Parsing article {}", article_path.display());
    STATS.parsed.fetch_add(1, Ordering::Relaxed);

    let mut new_article: ArticleWithTags = ArticleWithTags {
        id: None,
        src_file_name: src_file_name_string.clone(),
        dst_file_name: utils::create_dst_file_name(&article_path, cfg.flat),
        hash,
        title: None,
        modification_date: None,
        summary: None,
        tags: None,
        series: None,
        draft: None,
        special_page: None,
        anchorjs: Some(true),
        tocify: Some(true),
        live_updates: Some(true),
        open_graph_image: None,
    };

    let file_path: PathBuf = input_path.join(article_path);
    let article_mdwn_raw_string = std::fs::read_to_string(file_path).unwrap();
    match eval_plugins(&article_mdwn_raw_string, &mut new_article) {
        Ok(article_mdwn_refined_source) => {
            if new_article.special_page == Some(true) {
                new_article.tocify = None;
            }
            if new_article.title == None {
                let title = utils::article_src_file_name_to_title(&article_path);
                new_article.title = Some(title);
            }
            // pandoc runs again when its input, pandoc itself or the lua filters changed
            let anchorjs: bool = new_article.anchorjs.unwrap_or(false);
            // the title is the root of the table of contents
            let toc_title: Option<&str> = match new_article.tocify {
                Some(true) => new_article.title.as_deref(),
                _ => None,
            };
            let hash: String = Fingerprint::new("pandoc-input")
                .add(&article_mdwn_refined_source)
                .add(&fingerprint::inputs().pandoc)
                .add([anchorjs as u8, toc_title.is_some() as u8])
                .add(toc_title.unwrap_or(""))
                .finish();
            // println!(
            //     "src_file_name_string.clone(): {}",
            //     src_file_name_string.clone()
            // );
            let renew_cache: bool = match get_cache(conn, src_file_name_string.clone()) {
                Some(cache_entry) => {
                    //println!("Cache_entry.hash: {}, hash: {}", cache_entry.hash, hash);
                    if cache_entry.hash == hash {
                        false
                    } else {
                        true
                    }
                }
                None => true,
            };
            if renew_cache {
                STATS.pandoc.fetch_add(1, Ordering::Relaxed);
                match pandoc_mdwn_2_html(article_mdwn_refined_source.clone(), anchorjs, toc_title) {
                    Ok(html) => {
                        match set_cache(conn, src_file_name_string.clone(), html.clone(), hash) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Error udpating cache: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!(
                            "Error: No entry in cache for path: {}: {}",
                            src_file_name_string, e,
                        );
                        return Err(e);
                    }
                };
            } else {
                println!(" ... skipping call to pandoc, already in cache");
            };
            Ok(new_article)
        }
        Err(e) => {
            println!(
                "Error: Evaluating plugins on: {}: {}",
                src_file_name_string, e,
            );
            Err(e)
        }
    }
}

fn eval_plugins(
    article_mdwn_raw_string: &String,
    article: &mut ArticleWithTags,
) -> Result<String, Box<dyn Error>> {
    let re = Regex::new(r"\[\[\!(.*?)\]\]").unwrap();

    let mut last = 0;
    let mut res: String = String::new();
    for mat in re.find_iter(&article_mdwn_raw_string) {
        let start = mat.start();
        let end = mat.end();

        if start > last {
            res += &article_mdwn_raw_string[last..start];
            last = start;
        }

        match exec_plugin(&article_mdwn_raw_string[start..end], article) {
            Ok(result) => {
                res.push_str(&result);
            }
            Err(e) => {
                res += &article_mdwn_raw_string[start..end];
                match utils::position_to_line_and_col_number(&article_mdwn_raw_string, start) {
                    Ok((line, col)) => {
                        println!(
                            "Error: call_plugin (at {}:{}:{}) returned error: {e}",
                            article.src_file_name, line, col
                        );
                    }
                    Err(_) => {
                        println!(
                            "Error: call_plugin (at {}:unknown position) returned error: {e}",
                            article.src_file_name
                        )
                    }
                }
            }
        }
        if end <= article_mdwn_raw_string.len() {
            res += &article_mdwn_raw_string[last..start];
            last = end;
        } else {
            return Err(
                "Error: The specified length to extract is beyond the string's bounds.".into(),
            );
        }
    }
    if last <= article_mdwn_raw_string.len() {
        let t = &article_mdwn_raw_string[last..];
        res += t;
    }
    Ok(res)
}

pub fn exec_plugin(input: &str, article: &mut ArticleWithTags) -> Result<String, Box<dyn Error>> {
    let pattern = r#"\[\[!([\w]+)(?:\s+(.*))?\]\]"#;
    let re = Regex::new(pattern).unwrap();

    if let Some(captures) = re.captures(input) {
        let name: &str = captures.get(1).unwrap().as_str();
        let argument = captures.get(2).map_or("", |m| m.as_str()).trim();

        match name.to_lowercase().as_str() {
            "title" => title::title(argument, article),
            "specialpage" => specialpage::specialpage(argument, article),
            "hiddentext" => hiddentext::hiddentext(argument),
            "draft" => draft::draft(argument, article),
            "meta" => meta::meta(argument, article),
            "series" => series::series(argument, article),
            "tag" => tag::tag(argument, article),
            "img" => img::img(argument, article),
            "summary" => summary::summary(argument, article),
            "opengraphimage" => opengraphimage::opengraphimage(argument, article),
            _ => Err(format!("Plugin '{}' is not supported", name).into()),
        }
    } else {
        Err("Plugin couldn't be decoded".into())
    }
}
