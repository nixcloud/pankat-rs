use crate::config;
use crate::renderer::utils::{date_and_time, tag_links_to_timeline};
use handlebars::Handlebars;
use serde_json::json;
use std::error::Error;
use std::fs;
use std::path::{PathBuf, MAIN_SEPARATOR};

use crate::articles::NewArticle;

pub fn create_html_from_standalone_template(
    article: NewArticle,
    html: String,
) -> Result<String, Box<dyn Error>> {
    let cfg = config::Config::get();

    let mut handlebars = Handlebars::new();

    let mut assets: PathBuf = PathBuf::from(cfg.assets.clone());
    assets.push("templates/standalone-template.html");
    let article_template = assets.as_path();
    let template_content = fs::read_to_string(article_template)?;

    handlebars.register_template_string("standalone-template", &template_content)?;

    let mut input_path: PathBuf = cfg.input.clone();
    if !input_path.as_os_str().is_empty() && !input_path.to_string_lossy().ends_with(MAIN_SEPARATOR)
    {
        input_path.push(""); // Ensures trailing separator
    }
    let article_source_code_fs: PathBuf = PathBuf::from(article.src_file_name.clone());
    let relative_path = article_source_code_fs.strip_prefix(&input_path).unwrap();

    let data = json!({
        "SiteBrandTitle": "Sample Brand",
        "Title": article.title,
        "NavAndNewArticle": html,
        "NewArticleSrcURL": relative_path,
        "NewArticleSrcFileName": article.src_file_name,
        "NewArticleDstFileName": article.dst_file_name,
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
    article: NewArticle,
    html: String,
) -> Result<String, Box<dyn Error>> {
    let cfg = config::Config::get();

    let mut handlebars = Handlebars::new();

    let mut assets: PathBuf = PathBuf::from(cfg.assets.clone());
    assets.push("templates/content-template.html");
    let article_template = assets.as_path();
    let template_content = fs::read_to_string(article_template)?;

    handlebars.register_template_string("content_template", &template_content)?;

    let articles_nav =
        create_html_from_navigation_articles_template("previous".to_string(), "next".to_string())?;

    let series_nav = create_html_from_navigation_series_template(
        "series".to_string(),
        "previous".to_string(),
        "next".to_string(),
    )?;

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
        "NewArticlesNAV": articles_nav,
        "SeriesNAV": series_nav,
        "Title": article.title,
        "DateAndTime": date_and_time,
        "Tags": tags,
        "NewArticleContent": html,
    });

    let result = handlebars.render("content_template", &data)?;
    Ok(result)
}

pub fn create_html_from_navigation_articles_template(
    article_previous_link: String,
    article_next_link: String,
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
    series_previous_link: String,
    series_next_link: String,
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
