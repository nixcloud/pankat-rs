use handlebars::Handlebars;
use serde_json::json;
use std::error::Error;
use std::fs;

pub fn create_html_from_article_template() -> Result<String, Box<dyn Error>> {
    // Step 1: Create a new Handlebars registry
    let mut handlebars = Handlebars::new();

    // Step 2: Load the template from a file
    let template_path = "documents/static/templates/article-template.html";
    let template_content = fs::read_to_string(template_path)?;

    // Step 3: Register the template with a name
    handlebars.register_template_string("welcome_html", &template_content)?;

    // Step 4: Get content
    let navtitlearticlesource = create_html_blah().unwrap_or("".to_string());
    
    // Step 5: Define data for the template
    let data = json!({
        "Title": "Sample Title",
        "SiteBrandTitle": "Sample Brand",
        "NavTitleArticleSource": navtitlearticlesource,
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

pub fn create_html_blah() -> Result<String, Box<dyn Error>> {
    // Create a new Handlebars registry
    let mut handlebars = Handlebars::new();

    // Step 2: Load the template from a file
    let template_path = "documents/static/templates/navTitleArticleSource.html";
    let template_content = fs::read_to_string(template_path)?;

    // Step 3: Register the template with a name
    handlebars.register_template_string("welcome_html", &template_content)?;

    // Register the template with a name
    handlebars.register_template_string("blah_html", &template_content)?;

    // Define data for the template
    let data = json!({
        "SpecialPage": false,
        "TitleNAV": "Navigation Title",
        "SeriesNAV": "Series Navigation",
        "Title": "Sample Title",
        "TimeString": "2023-10-05",
        "Body": "This is the body content",
    });

    // Render the template with the data
    let result = handlebars.render("blah_html", &data)?;

    // Return the rendered result
    Ok(result)
}