use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

mod plugins;
mod utils;

use crate::config;
use crate::renderer::html::{
    create_html_from_content_template, create_html_from_standalone_template,
};
use crate::renderer::pandoc::render_file;

mod tests;
use self::plugins::{draft, img, meta, series, specialpage, summary, tag, title, PluginResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Article {
    /// relative to $input
    pub src_file_name: PathBuf,
    /// relative to $input or flattened to single filename
    pub dst_file_name: Option<PathBuf>,
    pub article_mdwn_source: Option<String>,
    pub draft: Option<bool>,

    /// override for the title (derived from filename by default)
    pub title: Option<String>,
    pub modification_date: Option<std::time::SystemTime>,
    pub summary: Option<String>,

    pub tags: Option<Vec<String>>,
    pub series: Option<String>,

    pub special_page: Option<bool>,
    pub timeline: Option<bool>,

    pub anchorjs: Option<bool>,
    pub tocify: Option<bool>,
    pub show_source_link: Option<bool>,
    /// register for live updates (default true)
    pub live_updates: Option<bool>,
}

pub fn scan_articles() -> HashMap<PathBuf, Article> {
    let mut articles: HashMap<PathBuf, Article> = HashMap::new();
    let cfg = config::Config::get();
    let input_path: PathBuf = cfg.input.clone();

    let start_time = std::time::Instant::now();

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

    for (_, article) in &articles {
        println!(
            "Writing article {} to disk",
            article.clone().dst_file_name.unwrap().display()
        );
        //println!("Article: {:#?}", article);
        write_article_to_disk(article);
    }

    let duration = start_time.elapsed();
    println!("Time taken to execute: {:?}", duration);

    articles
}

fn write_article_to_disk(article: &Article) {
    let cfg = config::Config::get();
    let output_path: PathBuf = cfg.output.clone();

    let article_mdwn_source = article.article_mdwn_source.clone().unwrap();

    match render_file(article_mdwn_source.clone()) {
        Ok(mdwn_html) => {
            let content: String =
                create_html_from_content_template(article.clone(), mdwn_html).unwrap();
            let standalone_html: String =
                create_html_from_standalone_template(article.clone(), content).unwrap();

            if let Some(dst_file_name) = &article.dst_file_name {
                let mut output_filename = output_path.clone();
                output_filename.push(dst_file_name);
                std::fs::write(output_filename, standalone_html)
                    .expect("Unable to write HTML file");
            }
        }
        Err(e) => {
            println!("Error: path: {} - {}", article_mdwn_source, e);
        }
    }
}

fn parse_article(article_path: &PathBuf) -> Result<Article, Box<dyn Error>> {
    println!(
        "Parsing article {} from disk",
        article_path.clone().display()
    );

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
    match eval_plugins(&article_mdwn_raw_string, &mut article) {
        Ok(article_mdwn_refined_source) => {
            article.article_mdwn_source = Some(article_mdwn_refined_source)
        }
        Err(err) => {
            println!(
                "Error on eval_plugins in article {}: {}",
                article_path.display(),
                err
            );
        }
    }

    let dst_file_name: PathBuf = article_path
        .with_extension("html")
        .file_name()
        .unwrap()
        .into();
    article.dst_file_name = Some(dst_file_name);

    if article.title == None {
        let title = utils::article_src_file_name_to_title(&article.src_file_name);
        article.title = Some(title);
    }

    Ok(article)
}

fn eval_plugins(
    article_mdwn_raw_string: &String,
    article: &mut Article,
) -> Result<String, Box<dyn Error>> {
    let re = Regex::new(r"\[\[\!(.*?)\]\]").unwrap();

    let mut last = 0;
    let mut res: String = String::new();
    for mat in re.find_iter(&article_mdwn_raw_string) {
        let start = mat.start();
        let end = mat.end();
        match exec_plugin(&article_mdwn_raw_string[start..end], article) {
            Ok(result) => {
                res.push_str(&result.output);
            }
            Err(e) => match utils::position_to_line_and_col_number(&article_mdwn_raw_string, start)
            {
                Ok((line, col)) => {
                    return Err(format!(
                        "Error: call_plugin (at {}:{}:{}) returned error: {e}",
                        article.src_file_name.display(),
                        line,
                        col
                    )
                    .into())
                }
                Err(_) => {
                    return Err(format!(
                        "Error: call_plugin (at {}:unknown position) returned error: {e}",
                        article.src_file_name.display()
                    )
                    .into())
                }
            },
        }
        if end <= article_mdwn_raw_string.len() {
            let t = &article_mdwn_raw_string[last..start];
            res += t;
            last = end;
        } else {
            return Err(
                "Error: The specified length to extract is beyond the string's bounds.".into(),
            );
        }
    }
    if last <= article_mdwn_raw_string.len() {
        let t = &article_mdwn_raw_string[last..];
        res += t;
    }
    Ok(res)
}

pub fn exec_plugin(input: &str, article: &mut Article) -> Result<PluginResult, Box<dyn Error>> {
    let pattern = r#"\[\[!([\w]+)(?:\s+(.*))?\]\]"#;
    let re = Regex::new(pattern).unwrap();

    if let Some(captures) = re.captures(input) {
        let name: &str = captures.get(1).unwrap().as_str();
        let argument = captures.get(2).map_or("", |m| m.as_str()).trim();

        match name.to_lowercase().as_str() {
            "title" => title::title(argument, article),
            "specialpage" => specialpage::specialpage(argument, article),
            "draft" => draft::draft(argument, article),
            "meta" => meta::meta(argument, article),
            "series" => series::series(argument, article),
            "tag" => tag::tag(argument, article),
            "img" => img::img(argument, article),
            "summary" => summary::summary(argument, article),
            _ => Err(format!("Plugin '{}' is not supported", name).into()),
        }
    } else {
        Err("Plugin couldn't be decoded".into())
    }
}
