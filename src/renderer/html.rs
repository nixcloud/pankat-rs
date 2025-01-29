
use handlebars::Handlebars;
use serde_json::json;
use std::fs;
use std::error::Error;

pub fn create_html_from_article_template() -> Result<String, Box<dyn Error>> {
    // Step 1: Create a new Handlebars registry
    let mut handlebars = Handlebars::new();

    // Step 2: Load the template from a file
    let template_path = "documents/static/templates/article-template.html";
    let template_content = fs::read_to_string(template_path)?;

    // Step 3: Register the template with a name
    handlebars.register_template_string("welcome_html", &template_content)?;

    // Step 4: Define data for the template
    let data = json!({
        "name": "Alice",
        "place": "Rust Land",
        "include_script": true
    });

    // Step 5: Render the template with the data
    let result = handlebars.render("welcome_html", &data)?;

    // Step 6: Return the rendered result
    Ok(result)
}
