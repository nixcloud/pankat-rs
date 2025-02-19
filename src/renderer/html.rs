use crate::articles::ArticleWithTags;
use crate::config;
use crate::db::article::ArticleNeighbours;
use crate::renderer::utils::{date_and_time, tag_links_to_timeline};
use handlebars::Handlebars;
use serde_json::json;
use std::error::Error;
use std::fs;
use std::path::{PathBuf, MAIN_SEPARATOR};

pub fn create_html_from_standalone_template(
    article: ArticleWithTags,
    html: String,
    //relative_path: String,
) -> Result<String, Box<dyn Error>> {
    let cfg = config::Config::get();

    let mut handlebars = Handlebars::new();

    let mut assets: PathBuf = PathBuf::from(cfg.assets.clone());
    assets.push("templates/standalone-template.html");
    let article_template = assets.as_path();
    let template_content = fs::read_to_string(article_template)?;

    handlebars.register_template_string("standalone-template", &template_content)?;

    // FIXME move code below to mod.rs
    let mut input_path: PathBuf = cfg.input.clone();
    if !input_path.as_os_str().is_empty() && !input_path.to_string_lossy().ends_with(MAIN_SEPARATOR)
    {
        input_path.push(""); // Ensures trailing separator
    }
    let article_source_code_fs: PathBuf = input_path.join(article.src_file_name.clone());
    // FIXME move code above to mod.rs
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

    let data = json!({
        "SiteBrandTitle": "Sample Brand",
        "Title": article.title,
        "NavAndArticle": html,
        "ArticleSrcURL": relative_path,
        "ArticleSrcFileName": article.src_file_name,
        "ArticleDstFileName": article.dst_file_name,
        "LiveUpdates": article.live_updates,
        "SpecialPage": article.special_page,
        "Anchorjs": article.anchorjs,
        "Tocify": article.tocify,
        "Timeline": article.timeline,
    });

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
        r#"<div id="tags"><p>{}</p></div>"#,
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
    });

    let result = handlebars.render("navigation_series_template", &data)?;

    Ok(result)
}

pub fn create_index_from_most_recent_article_template(
    most_recent_article: String,
) -> Result<String, Box<dyn Error>> {
    let cfg = config::Config::get();

    let mut handlebars = Handlebars::new();

    let mut assets: PathBuf = PathBuf::from(cfg.assets.clone());
    assets.push("templates/most-recent-article.html");
    let template = assets.as_path();
    let template_content = fs::read_to_string(template)?;

    handlebars.register_template_string("most-recent-article", &template_content)?;

    let data = json!({
        "most_recent_article": most_recent_article,
    });

    let result = handlebars.render("most-recent-article", &data)?;

    Ok(result)
}
