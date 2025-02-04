use crate::config;
use handlebars::Handlebars;
use serde_json::json;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

// https://docs.rs/handlebars/latest/handlebars/

pub fn create_html_from_standalone_template() -> Result<String, Box<dyn Error>> {
    let cfg = config::Config::get();
    
    // Step 1: Create a new Handlebars registry
    let mut handlebars = Handlebars::new();

    // Step 2: Load the template from a file
    let mut assets: PathBuf = PathBuf::from(cfg.assets.clone());
    assets.push("templates/standalone-template.html");
    let article_template = assets.as_path();
    let template_content = fs::read_to_string(article_template)?;

    // Step 3: Register the template with a name
    handlebars.register_template_string("welcome_html", &template_content)?;

    // Step 4: Get content
    let content_template = create_html_from_content_template().unwrap_or("".to_string());

    // Step 5: Define data for the template
    let data = json!({
        "Title": "Sample Title",
        "SiteBrandTitle": "Sample Brand",
        "NavTitleArticleSource": content_template,
        "ArticleSourceCodeURL": "http://example.com/source",
        "ArticleSourceCodeFS": "/local/path/to/source",
        "ArticleDstFileName": "roadmap.html",
        "ShowSourceLink": true,
        "LiveUpdates": true,
        "SpecialPage": true,
        "Anchorjs": true,
        "Tocify": true,
        "Timeline": true,
    });

    // Step 6: Render the template with the data
    let result = handlebars.render("welcome_html", &data)?;

    // Step 7: Return the rendered result
    Ok(result)
}

pub fn create_html_from_content_template() -> Result<String, Box<dyn Error>> {
    let cfg = config::Config::get();

    // Step 1: Create a new Handlebars registry
    let mut handlebars = Handlebars::new();

    // Step 2: Load the template from a file
    let mut assets: PathBuf = PathBuf::from(cfg.assets.clone());
    assets.push("templates/content-template.html");
    let article_template = assets.as_path();
    let template_content = fs::read_to_string(article_template)?;

    // Step 3: Register the template with a name
    handlebars.register_template_string("welcome_html", &template_content)?;

    // Step 5: Define data for the template
    let data = json!({
        "SpecialPage": true,
        "TitleNAV": "Sample Title",
        "SeriesNAV": "Sample Title",
        "Title": "Sample Title",
        "TimeString": "Sample Brand",
        "Body": "Sample Brand",
    });

    // Step 6: Render the template with the data
    let result = handlebars.render("welcome_html", &data)?;

    // Step 7: Return the rendered result
    Ok(result)
}
