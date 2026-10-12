use crate::articles::ArticleWithTags;
use std::error::Error;

/// [[!OpenGraphImage posts/foo/card.png]] sets the og:image used for social previews,
/// the path is relative to the site root like in [[!img]] or an absolute URL
pub fn opengraphimage(input: &str, article: &mut ArticleWithTags) -> Result<String, Box<dyn Error>> {
    let image = input.trim();
    if image.is_empty() {
        Err("Argument is missing, expected an image path".into())
    } else if image.contains(char::is_whitespace) {
        Err("Argument contains whitespace, expected a single image path".into())
    } else {
        article.open_graph_image = Some(image.to_string());
        Ok("".to_string())
    }
}
