use std::error::Error;
use std::path::PathBuf;

pub fn position_to_line_and_col_number(
    article_mdwn_raw_string: &String,
    position: usize,
) -> Result<(usize, usize), Box<dyn Error>> {
    if position >= article_mdwn_raw_string.len() {
        return Err("Position is out of bounds".into());
    }

    let mut line = 0;
    let mut col = 0;
    let mut current_pos = 0;

    for (_i, ch) in article_mdwn_raw_string.chars().enumerate() {
        if current_pos == position {
            return Ok((line, col));
        }

        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }

        current_pos += ch.len_utf8();
    }

    Err("Failed to backtrack position".into())
}

pub fn article_src_file_name_to_title(article_src_file_name: &PathBuf) -> String {
    // Convert PathBuf to string and handle error
    let file_name_str = article_src_file_name
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();
    // Replace underscores with spaces and return the result
    file_name_str.replace("_", " ").to_string()
}

pub fn create_dst_file_name(article_path: &PathBuf, flat: bool) -> String {
    if flat {
        article_path
            .with_extension("html")
            .file_name()
            .unwrap()
            .to_string_lossy()
            .replace(" ", "_")
            .to_string()
    } else {
        let path_without_prefix = article_path
            .strip_prefix("posts")
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|_| article_path.clone());
        path_without_prefix
            .with_extension("html")
            .to_string_lossy()
            .replace(" ", "_")
            .to_string()
    }
}

#[test]
fn test_create_dst_file_name_flat() {
    let article_path = PathBuf::from("src/articles/test.mdwn");
    let flat = true;
    let dst_file_name = create_dst_file_name(&article_path, flat);
    assert_eq!(dst_file_name, "test.html");
}

#[test]
fn test_create_dst_file_name() {
    let article_path = PathBuf::from("src/articles/test.mdwn");
    let flat = false;
    let dst_file_name = create_dst_file_name(&article_path, flat);
    assert_eq!(dst_file_name, "src/articles/test.html");
}
