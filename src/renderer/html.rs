use crate::config;
use handlebars::Handlebars;
use serde_json::json;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::articles::Article;

// https://docs.rs/handlebars/latest/handlebars/

pub fn create_html_from_standalone_template(
    article: Article,
    html: String,
) -> Result<String, Box<dyn Error>> {
    let cfg = config::Config::get();

    // Step 1: Create a new Handlebars registry
    let mut handlebars = Handlebars::new();

    // Step 2: Load the template from a file
    let mut assets: PathBuf = PathBuf::from(cfg.assets.clone());
    assets.push("templates/standalone-template.html");
    let article_template = assets.as_path();
    let template_content = fs::read_to_string(article_template)?;

    // Step 3: Register the template with a name
    handlebars.register_template_string("standalone-template", &template_content)?;

    // Step 5: Define data for the template
    let data = json!({
        "Title": article.title,
        "SiteBrandTitle": "Sample Brand",
        "NavTitleArticleSource": html,
        "ArticleSourceCodeURL": "http://example.com/source",
        "ArticleSourceCodeFS": article.src_file_name,
        "ArticleDstFileName": article.dst_file_name,
        "ShowSourceLink": article.show_source_link,
        "LiveUpdates": article.live_updates,
        "SpecialPage": article.special_page,
        "Anchorjs": article.anchorjs,
        "Tocify": article.tocify,
        "Timeline": article.timeline,
    });

    // Step 6: Render the template with the data
    let result = handlebars.render("standalone-template", &data)?;

    // Step 7: Return the rendered result
    Ok(result)
}

pub fn create_html_from_content_template(
    article: Article,
    html: String,
) -> Result<String, Box<dyn Error>> {
    let cfg = config::Config::get();

    // Step 1: Create a new Handlebars registry
    let mut handlebars = Handlebars::new();

    // Step 2: Load the template from a file
    let mut assets: PathBuf = PathBuf::from(cfg.assets.clone());
    assets.push("templates/content-template.html");
    let article_template = assets.as_path();
    let template_content = fs::read_to_string(article_template)?;

    // Step 3: Register the template with a name
    handlebars.register_template_string("content_template", &template_content)?;

    // Step 5: Define data for the template
    let data = json!({
        "SpecialPage": article.special_page,
        "TitleNAV": "TitleNAV",
        "SeriesNAV": "SeriesNAV",
        "Title": article.title,
        "TimeString": "Sample Brand",
        "Body": html,
    });

    // Step 6: Render the template with the data
    let result = handlebars.render("content_template", &data)?;

    // Step 7: Return the rendered result
    Ok(result)
}


