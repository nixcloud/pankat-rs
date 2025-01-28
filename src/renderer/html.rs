use handlebars::Handlebars;
use serde_json::json;
use std::fs;

fn main() {
    // Step 1: Create a new Handlebars registry
    let mut handlebars = Handlebars::new();

    // Step 2: Load the template from a file
    let template_path = "documents/static/templates/article-template.html";
    let template_content = fs::read_to_string(template_path)
        .expect("Failed to read the template file");

    // Step 3: Register the template with a name
    handlebars
        .register_template_string("welcome_html", template_content)
        .expect("Failed to register template");

    // Step 4: Define data for the template
    let data = json!({
        "name": "Alice",
        "place": "Rust Land",
        "include_script": true // Toggle this to enable/disable the script
    });

    // Step 5: Render the template with the data
    let result = handlebars
        .render("welcome_html", &data)
        .expect("Failed to render template");

    // Step 6: Output the rendered result
    println!("{}", result);
}
