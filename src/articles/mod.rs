use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use regex::Regex;
use std::time::SystemTime;

use crate::renderer::*;
use crate::config;

/// asdf
pub struct Article {
    /// relative to $input
    src_file_name: PathBuf,
    /// relative to $input or flattened to single filename
    dst_file_name: Option<PathBuf>,
    article_mdwn_source: Option<String>,
    draft: Option<bool>,

    /// override for the title (derived from filename by default)
    title: Option<String>,
    modification_date: Option<std::time::SystemTime>,
    summary: Option<String>,

    tags: Option<Vec<String>>,
    series: Option<String>,

    special_page: Option<bool>,
    timeline: Option<bool>,

    anchorjs: Option<bool>,
    tocify: Option<bool>,
    show_source_link: Option<bool>,
    /// register for live updates (default true)
    live_updates: Option<bool>,
}

pub fn scan_articles() -> HashMap<PathBuf, Article> {
    let mut articles: HashMap<PathBuf, Article> = HashMap::new();
    let cfg = config::Config::get();
    let input_path: PathBuf = cfg.input.clone();

    fn traverse_and_collect_articles(dir: &PathBuf, articles: &mut HashMap<PathBuf, Article>) {
        if dir.is_dir() {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_dir() {
                            traverse_and_collect_articles(&path, articles);
                        } else if let Some(ext) = path.extension() {
                            if ext == "mdwn" {
                                match parse_article(&path) {
                                    Ok(article) => {
                                        articles.insert(path, article);
                                    }
                                    Err(_) => { /* Handle errors if necessary */ }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    traverse_and_collect_articles(&input_path, &mut articles);

    //println!("articles: {:?}", articles.len());

    for (_, article) in &articles {
        write_article_to_disk(article);
    }

    articles
}

fn write_article_to_disk(article: &Article) {
    let cfg = config::Config::get();
    let output_path: PathBuf = cfg.output.clone();

    // let content = renderer::create_html_from_content_template(article);
    // let html: String = renderer::create_html_from_standalone_template(article, content);

    // if let Some(dst_file_name) = &article.dst_file_name {
    //     let mut output_file = output_path.clone();
    //     output_file.push(dst_file_name);
    //     std::fs::write(output_file, html).expect("Unable to write HTML file");
    // }
}

fn parse_article(article_path: &PathBuf) -> Result<Article, Box<dyn Error>> {
    // flattens directory structure into a string only

    let mut article: Article = Article {
        src_file_name: article_path.clone(),
        dst_file_name: None,
        article_mdwn_source: None,
        title: None,
        modification_date: None,
        summary: None,
        tags: None,
        series: None,

        draft: None,
        special_page: None,
        timeline: None,

        anchorjs: None,
        tocify: None,
        show_source_link: None,
        live_updates: None,
    };

    let article_mdwn_raw_string = std::fs::read_to_string(article_path).unwrap();
    match process_plugins(&article_mdwn_raw_string, &mut article) {
        Ok(article_mdwn_source) => article.article_mdwn_source = Some(article_mdwn_source),
        Err(err) => {
            todo!()
        }
    }

    let dst_file_name: PathBuf = article_path
        .with_extension("html")
        .file_name()
        .unwrap()
        .into();
    article.dst_file_name = Some(dst_file_name);

    Ok(article)
}

fn process_plugins(
    article_mdwn_raw_string: &String,
    article: &mut Article,
) -> Result<String, Box<dyn Error>> {
    //     let mut processed_article = Vec::new();
    //     let re = Regex::new(r"\[\[!(.*?)\]\]").unwrap();
    //     let mut prev_pos = 0;
    //     let mut found_plugins = Vec::new();

    //     for mat in re.find_iter(article_bytes) {
    //         if prev_pos != mat.start() {
    //             processed_article.extend_from_slice(&article_bytes[prev_pos..mat.start()]);
    //         }
    //         let (plugin_output, name) = call_plugin(&article_bytes[mat.start()..mat.end()], article);
    //         found_plugins.push(name);
    //         processed_article.extend_from_slice(&plugin_output);
    //         prev_pos = mat.end();
    //     }

    //     processed_article.extend_from_slice(&article_bytes[prev_pos..]);
    //     println!("{} plugins: {:?}", article.dst_file_name, found_plugins);
    //     processed_article
    Ok("".to_string())
}

// fn call_plugin(input: &String, article: &mut Article) -> (Vec<u8>, String) {
//     // if input.len() < 5 {
//     //     return (Vec::new(), String::new());
//     // }
//     // let content = &input[3..input.len() - 2];
//     // let parts: Vec<&str> = String::from_utf8_lossy(content)
//     //     .split_whitespace()
//     //     .collect();
//     // let name = parts.get(0).map(|s| s.to_lowercase()).unwrap_or_default();
//     // let mut output = Vec::new();

//     match name.as_str() {
//         //     "specialpage" => article.special_page = true,
//         //     "draft" => article.draft = true,
//         // "meta" => {
//         //     let re = Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}").unwrap();
//         //     if let Some(mat) = re.find(&String::from_utf8_lossy(input)) {
//         //         if let Ok(parsed_time) =
//         //             chrono::NaiveDateTime::parse_from_str(mat.as_str(), "%Y-%m-%d %H:%M")
//         //         {
//         //             article.modification_date = SystemTime::from(parsed_time);
//         //         }
//         //     }
//         // }
//         // "series" => {
//         //     if parts.len() > 1 {
//         //         article.series = Some(parts[1..].join(" "));
//         //     }
//         // }
//         // "tag" => {
//         //     for tag in &parts[1..] {
//         //         article.tags.push(Tag {
//         //             name: tag.to_string(),
//         //         });
//         //     }
//         // }
//         // "img" => {
//         //     if parts.len() > 1 {
//         //         let img_tag = format!(
//         //             "<a href=\"{}\"><img src=\"{}\"></a>",
//         //             parts[1],
//         //             parts[1..].join(" ")
//         //         );
//         //         output = img_tag.into_bytes();
//         //     }
//         // }
//         // "summary" => {
//         //     if parts.len() > 1 {
//         //         article.summary = Some(parts[1..].join(" "));
//         //     }
//         // }
//         // "title" => {
//         //     if parts.len() > 1 {
//         //         article.title = Some(parts[1..].join(" "));
//         //     }
//         // }
//         _ => println!(
//             "{}: plugin '{}' NOT supported",
//             article.src_file_name.display(),
//             name
//         ),
//     }
//     (output, name)
// }
