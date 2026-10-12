use crate::articles::timeline::tag_links_to_timeline;
use crate::articles::ArticleWithTags;
use crate::config;
use crate::db::article::ArticleNeighbours;
use crate::renderer::utils::date_and_time;
use handlebars::Handlebars;
use serde_json::json;
use std::error::Error;
use std::fs;
use std::path::{PathBuf, MAIN_SEPARATOR};

/// site_url without scheme and trailing slash, shown top left: "lastlog.de/blog"
pub fn site_label() -> String {
    let cfg = config::Config::get();
    let url = cfg.site_url.trim_end_matches('/');
    match url.split_once("://") {
        Some((_, rest)) => rest.to_string(),
        None => url.to_string(),
    }
}

/// absolute URL below site_url for a path relative to the site root, absolute URLs pass through
pub fn absolute_url(path: &str) -> String {
    if path.contains("://") {
        return path.to_string();
    }
    let cfg = config::Config::get();
    let base = cfg.site_url.trim_end_matches('/');
    let base = if base.contains("://") {
        base.to_string()
    } else {
        format!("https://{}", base)
    };
    format!("{}/{}", base, path.trim_start_matches('/'))
}

/// Open Graph / twitter card data for the social preview of a page, used by
/// templates/open-graph-template.html
pub fn open_graph_data(
    og_type: &str,
    title: &str,
    description: Option<String>,
    dst_file_name: &str,
    image: Option<String>,
    published_time: Option<chrono::NaiveDateTime>,
    tags: Option<Vec<String>>,
) -> serde_json::Value {
    let image_url: Option<String> = image.map(|i| absolute_url(&i));
    json!({
        "Type": og_type,
        "SiteName": site_label(),
        "Title": title,
        "Description": description,
        "Url": absolute_url(dst_file_name),
        "TwitterCard": if image_url.is_some() { "summary_large_image" } else { "summary" },
        "Image": image_url,
        "PublishedTime": published_time.map(|t| t.format("%Y-%m-%dT%H:%M:%S").to_string()),
        "Tags": tags.unwrap_or_default(),
    })
}

pub fn open_graph_data_by_article(article: &ArticleWithTags) -> serde_json::Value {
    if article.draft == Some(true) {
        return serde_json::Value::Null;
    }
    let special_page = article.special_page == Some(true);
    open_graph_data(
        if special_page { "website" } else { "article" },
        article.title.as_deref().unwrap_or(""),
        article.summary.clone(),
        &article.dst_file_name,
        article.open_graph_image.clone(),
        if special_page { None } else { article.modification_date },
        article.tags.clone(),
    )
}

fn register_open_graph_partial(handlebars: &mut Handlebars) -> Result<(), Box<dyn Error>> {
    let cfg = config::Config::get();
    let mut assets: PathBuf = PathBuf::from(cfg.assets.clone());
    assets.push("templates/open-graph-template.html");
    let template_content = fs::read_to_string(assets.as_path())?;
    handlebars.register_partial("open_graph", template_content)?;
    Ok(())
}

pub fn create_html_from_standalone_template_by_article(
    article: ArticleWithTags,
    html: String,
) -> Result<String, Box<dyn Error>> {
    let cfg = config::Config::get();

    let data: serde_json::Value = json!({
        "SiteBrandTitle": site_label(),
        "Subdir": cfg.subdir,
        "Title": article.title,
        "NavAndContent": html,
        "LiveUpdates": article.live_updates,
        "SpecialPage": article.special_page,
        "Tocify": article.tocify,
        "Timeline": false,
        "ArticleSrcFileName": article.src_file_name,
        "ArticleDstFileName": article.dst_file_name,
        "OpenGraph": open_graph_data_by_article(&article),
    });

    create_html_from_standalone_template(data)
}

pub fn create_html_from_standalone_template(
    data: serde_json::Value,
) -> Result<String, Box<dyn Error>> {
    let cfg = config::Config::get();

    let mut handlebars = Handlebars::new();

    let mut assets: PathBuf = PathBuf::from(cfg.assets.clone());
    assets.push("templates/standalone-template.html");
    let template = assets.as_path();
    let template_content = fs::read_to_string(template)?;

    register_open_graph_partial(&mut handlebars)?;
    handlebars.register_template_string("standalone-template", &template_content)?;

    let result = handlebars.render("standalone-template", &data)?;
    Ok(result)
}

pub fn create_html_from_content_template(
    article: ArticleWithTags,
    html: String,
    article_neighbours: ArticleNeighbours,
    article_series_neighbours: ArticleNeighbours,
) -> Result<String, Box<dyn Error>> {
    let cfg = config::Config::get();

    let mut input_path: PathBuf = cfg.input.clone();
    if !input_path.as_os_str().is_empty() && !input_path.to_string_lossy().ends_with(MAIN_SEPARATOR)
    {
        input_path.push(""); // Ensures trailing separator
    }
    let article_source_code_fs: PathBuf = input_path.join(article.src_file_name.clone());
    let relative_path = match article_source_code_fs.strip_prefix(&input_path) {
        Ok(res) => res,
        Err(e) => {
            println!("Error: Article source file path is not within input path");
            println!("   input_path: {}", input_path.display());
            println!(
                "   article.src_file_name: {}",
                article.src_file_name.clone()
            );
            return Err(Box::new(e));
        }
    };
    // println!("article: {:#?}", article);

    // println!(
    //     "article_series_neighbours: {:#?}",
    //     article_series_neighbours
    // );

    let mut handlebars = Handlebars::new();

    let mut assets: PathBuf = PathBuf::from(cfg.assets.clone());
    assets.push("templates/content-template.html");
    let article_template = assets.as_path();
    let template_content = fs::read_to_string(article_template)?;

    handlebars.register_template_string("content_template", &template_content)?;

    let articles_nav = create_html_from_navigation_articles_template(
        match article_neighbours.prev.clone() {
            Some(p) => Some(p.dst_file_name),
            None => None,
        },
        match article_neighbours.next.clone() {
            Some(n) => Some(n.dst_file_name),
            None => None,
        },
    )?;

    let series_nav = match article.series {
        Some(series) => create_html_from_navigation_series_template(
            series,
            match article_series_neighbours.prev.clone() {
                Some(p) => Some(p.dst_file_name),
                None => None,
            },
            match article_series_neighbours.next.clone() {
                Some(n) => Some(n.dst_file_name),
                None => None,
            },
        )?,
        None => "".to_string(),
    };

    let date_and_time: String = format!(
        r#"<div id="date"><p><span id="lastupdated">{}</span></p></div>"#,
        date_and_time(&article.modification_date)
    )
    .to_string();

    let tags: String = format!(
        r#"<div class="tags">{}</div>"#,
        tag_links_to_timeline(article.tags)
    )
    .to_string();

    let data = json!({
        "SpecialPage": article.special_page,
        "ArticlesNAV": articles_nav,
        "SeriesNAV": series_nav,
        "Title": article.title,
        "DateAndTime": date_and_time,
        "Tags": tags,
        "ArticleContent": html,
        "ArticleSrcURL": relative_path,
        "Subdir": cfg.subdir,
    });

    let result = handlebars.render("content_template", &data)?;
    Ok(result)
}

pub fn create_html_from_navigation_articles_template(
    article_previous_link: Option<String>,
    article_next_link: Option<String>,
) -> Result<String, Box<dyn Error>> {
    let cfg = config::Config::get();

    let mut handlebars = Handlebars::new();

    let mut assets: PathBuf = PathBuf::from(cfg.assets.clone());
    assets.push("templates/navigation-articles-template.html");
    let article_template = assets.as_path();
    let template_content = fs::read_to_string(article_template)?;

    handlebars.register_template_string("navigation_articles_template", &template_content)?;

    let data = json!({
        "article_previous_link": article_previous_link,
        "article_next_link": article_next_link,
        "Subdir": cfg.subdir,
    });

    let result = handlebars.render("navigation_articles_template", &data)?;

    Ok(result)
}

pub fn create_html_from_navigation_series_template(
    series: String,
    series_previous_link: Option<String>,
    series_next_link: Option<String>,
) -> Result<String, Box<dyn Error>> {
    let cfg = config::Config::get();

    let mut handlebars = Handlebars::new();

    let mut assets: PathBuf = PathBuf::from(cfg.assets.clone());
    assets.push("templates/navigation-series-template.html");
    let article_template = assets.as_path();
    let template_content = fs::read_to_string(article_template)?;

    handlebars.register_template_string("navigation_series_template", &template_content)?;

    let data = json!({
        "series": series,
        "series_previous_link": series_previous_link,
        "series_next_link": series_next_link,
        "Subdir": cfg.subdir,
    });

    let result = handlebars.render("navigation_series_template", &data)?;

    Ok(result)
}

pub fn create_index_from_most_recent_article_template(
    most_recent_article: &ArticleWithTags,
) -> Result<String, Box<dyn Error>> {
    let cfg = config::Config::get();

    let mut handlebars = Handlebars::new();

    let mut assets: PathBuf = PathBuf::from(cfg.assets.clone());
    assets.push("templates/most-recent-article.html");
    let template = assets.as_path();
    let template_content = fs::read_to_string(template)?;

    register_open_graph_partial(&mut handlebars)?;
    handlebars.register_template_string("most-recent-article", &template_content)?;

    // scrapers of X/linkedin don't follow the meta refresh, so the index carries
    // the social preview of the most recent article
    let data = json!({
        "most_recent_article": most_recent_article.dst_file_name,
        "OpenGraph": open_graph_data_by_article(most_recent_article),
    });

    let result = handlebars.render("most-recent-article", &data)?;

    Ok(result)
}
