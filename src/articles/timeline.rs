use crate::articles::ArticleWithTags;
use crate::config;
use serde_json::json;

use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

fn create_json_metadata(articles: &Vec<ArticleWithTags>) -> String {
    let meta_data: MetaData = MetaData::create_js_metadata(articles);
    //println!("{:#?}", meta_data);
    format!(
        r#"
        <script type="application/json" id="MetaData">{}</script>
       "#,
        serde_json::to_string(&meta_data).expect("Failed to serialize MetaData to JSON")
    )
}

#[derive(Debug, serde::Serialize)]
struct MetaData {
    #[serde(rename = "ArticleCount")]
    article_count: usize,
    #[serde(rename = "Tags")]
    tags: HashMap<String, Vec<usize>>,
    #[serde(rename = "Series")]
    series: HashMap<String, Vec<usize>>,
    #[serde(rename = "Years")]
    years: HashMap<usize, Vec<usize>>,
}

impl MetaData {
    pub fn create_js_metadata(articles: &Vec<ArticleWithTags>) -> MetaData {
        let mut tags_map: HashMap<String, Vec<usize>> = HashMap::new();
        let mut series_map: HashMap<String, Vec<usize>> = HashMap::new();
        let mut years_map: HashMap<usize, Vec<usize>> = HashMap::new();

        for article in articles {
            let year = match article.modification_date {
                Some(m_date) => m_date
                    .format("%Y")
                    .to_string()
                    .parse::<usize>()
                    .unwrap_or(0),
                None => 0,
            };
            years_map
                .entry(year)
                .or_insert_with(Vec::new)
                .push(article.id.unwrap() as usize);

            match &article.tags {
                Some(tags) => {
                    for tag in tags {
                        tags_map
                            .entry(tag.clone())
                            .or_insert_with(Vec::new)
                            .push(article.id.unwrap() as usize);
                    }
                }
                None => {}
            }

            if let Some(series) = &article.series {
                if !series.is_empty() {
                    series_map
                        .entry(series.clone())
                        .or_insert_with(Vec::new)
                        .push(article.id.unwrap() as usize);
                }
            }
        }

        MetaData {
            article_count: articles.len(),
            tags: tags_map,
            series: series_map,
            years: years_map,
        }
    }
}

fn series_to_link_list(series: Option<String>) -> String {
    match series {
        Some(series) => {
            format!(
                "<a class=\"seriesbtn btn btn-primary\" onClick=\"setFilter('series::{}', 1)\">{}</a>",
                series, series
            )
        }
        None => "".to_string(),
    }
}

pub fn tag_links_to_timeline(tags: Option<Vec<String>>) -> String {
    match tags {
        Some(tags) => {
            let mut result = String::new();
            for tag in tags {
                result.push_str(
                    &format!(
                    r#"<a href="timeline.html?filter=tag::{}" class="tagbtn btn btn-primary">{}</a>"#,
                    tag, tag
                ));
            }
            result
        }
        None => String::new(),
    }
}

fn tag_to_link_list(article: &ArticleWithTags) -> String {
    let mut output = String::new();
    if let Some(tags) = &article.tags {
        for tag in tags {
            output.push_str(&format!(
                "<a class=\"tagbtn btn btn-primary\" onClick=\"setFilter('tag::{}', 1)\">{}</a>",
                tag, tag
            ));
        }
    }
    output
}

pub fn update_timeline(articles: &Vec<ArticleWithTags>) -> Result<(), Box<dyn Error>> {
    println!("====== Updating 'timeline' ======");
    let cfg = config::Config::get();

    let html: String = format!(
        r#"
           {}
           <div class="article">
           <h1 id="SiteTitle">timeline</h1>
           <p>a list of all posts, sorted by date, with the most recent posts at the top.</p>
           {}
           {}
           </div>
           "#,
        create_json_metadata(articles),
        create_filter_control(articles),
        create_timeline_container(articles)
    );

    let data: serde_json::Value = json!({
        "SiteBrandTitle": cfg.brand,
        "Title": "timeline",
        "NavAndContent": html,
        "ArticleSrcURL": "",
        "ArticleSrcFileName": "",
        "ArticleDstFileName": "",
        "LiveUpdates": false,
        "SpecialPage": true,
        "Anchorjs": false,
        "Tocify": false,
        "Timeline": true,
    });

    match crate::renderer::html::create_html_from_standalone_template(data) {
        Ok(html) => {
            let cfg = config::Config::get();
            let mut output_path: PathBuf = cfg.output.clone();
            output_path.push("timeline.html");
            crate::articles::write_to_disk(&html, &output_path);
            Ok(())
        }
        Err(e) => {
            println!("{}", e);
            Err(e)
        }
    }
}

fn create_timeline_container(articles: &Vec<ArticleWithTags>) -> String {
    let mut years_map: HashMap<usize, Vec<ArticleWithTags>> = HashMap::new();
    for article in articles {
        let year = match article.modification_date {
            Some(m_date) => m_date
                .format("%Y")
                .to_string()
                .parse::<usize>()
                .unwrap_or(0),
            None => 0,
        };
        years_map
            .entry(year)
            .or_insert_with(Vec::new)
            .push(article.clone());
    }

    let mut keys: Vec<_> = years_map.keys().cloned().collect();
    keys.sort_by(|a, b| b.cmp(a));
    let mut page_content = String::new();
    page_content.push_str(
        r#"
          <div id="timeline" class="timeline-container">
          <br class="clear">
        "#,
    );

    for &year in &keys {
        page_content.push_str(&generate_year(year, &years_map[&year]));
        let y: usize = year - 1;
        if !years_map.contains_key(&y) {
            page_content.push_str(&generate_year(y, &Vec::new()));
        }
    }

    page_content.push_str("</div><!-- /.timeline-container -->");
    page_content.push_str(
        r#"
            </div><!-- /.timeline-container -->
        "#,
    );
    page_content
}

fn generate_year(year: usize, articles: &Vec<ArticleWithTags>) -> String {
    let template = r#"
        <div class="timeline-wrapper pankat_year pankat_year_{{Year}}">
          <dl class="timeline-series">
            <h2 class="timeline-time"><span>{{Year}}</span></h2>
              {{{generate_article_references_by_year}}}
          </dl><!-- /.timeline-series -->
        </div><!-- /.timeline-wrapper -->
        "#;

    let context = json!({
        "Year": year+1,
        "generate_article_references_by_year": generate_article_references_by_year(articles),
    });

    mustache::compile_str(template)
        .expect("Failed to compile template")
        .render_to_string(&context)
        .expect("Failed to render template")
}

fn generate_article_references_by_year(articles: &Vec<ArticleWithTags>) -> String {
    let template = r#"
        <div class="posting_div posting_{{article_id}}">
          <div class="timeline-event-content postingsEntry">
            <div>
              <div class="timeline-title">{{article_title}}</div>
              <div style="display: flex; flex-wrap: wrap; gap: 10px;">
                <div class="timeline-title-timestamp">{{article_date}}</div>
                <a href="{{dst_file_name}}" style="flex: 1;">open complete article</a>
              </div>
              <div class="summary">{{{summary}}}</div>
            </div>
            <p class="tag">{{{tagToLinkList}}}{{{seriesToLinkList}}}</p>
          </div>
        </div>
        "#;

    let mut output: String = String::new();

    for article in articles {
        let context = json!({
            "article_id": article.id.unwrap() as usize,
            "article_title": article.title,
            "article_date": crate::renderer::utils::date_and_time(&article.modification_date),
            "summary": article.summary,
            "dst_file_name": article.dst_file_name,
            "tagToLinkList": tag_to_link_list(article),
            "seriesToLinkList": series_to_link_list(article.series.clone()),
        });

        let template: String = mustache::compile_str(template)
            .expect("Failed to compile template")
            .render_to_string(&context)
            .expect("Failed to render template");

        output.push_str(&template);
    }
    output
}

fn rank_by_word_count(word_frequencies: &HashMap<String, usize>) -> Vec<(String, usize)> {
    let mut pairs: Vec<(String, usize)> = word_frequencies.clone().into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1));
    pairs
}

fn create_filter_control(articles: &Vec<ArticleWithTags>) -> String {
    let mut tags_map: HashMap<String, usize> = HashMap::new();
    let mut series_map: HashMap<String, usize> = HashMap::new();

    for article in articles {
        if let Some(series) = &article.series {
            if !series.is_empty() {
                *series_map.entry(series.clone()).or_insert(0) += 1;
            }
        }
        if let Some(tags) = &article.tags {
            for tag in tags {
                *tags_map.entry(tag.clone()).or_insert(0) += 1;
            }
        }
    }

    let tags_slice = rank_by_word_count(&tags_map);

    let mut page_content = String::new();
    page_content.push_str(
        r#"<div id="FilterControl">
            <div id="FilterControlIcon">
                <span class="glyphicon glyphicon-filter" aria-hidden="true"></span>
            </div>
            <div id="FilterControlElements">
                <div id="FilterPreSelection">
                    <a class="btn btn-primary" onClick="toSelectionView()">show tag/series filters</a>
                </div>
                <div id="FilterPostSelection">
                    <div id="FilterSelectionContent">
                        <a class="btn btn-primary" onClick="toClearSelectionView()"><span class="glyphicon glyphicon-remove-circle" aria-hidden="true"></span> clear filter</a>
                        <div id="FilterSelectionText"></div>
                    </div>
                </div>
            <div id="FilterSelection">
            <p class="lead">select a tag or series element below:</p>
            <div id="TagAndSeries">
        "#
    );

    page_content.push_str("<p id=\"tagCloud\">");
    for (tag, _) in tags_slice.iter() {
        page_content.push_str(&format!(
            "<a class=\"tagbtn btn btn-primary\" onClick=\"setFilter('tag::{}', 1)\">{}</a>",
            tag, tag
        ));
    }
    page_content.push_str("</p>");

    let series_slice = rank_by_word_count(&series_map);

    page_content.push_str("<p id=\"seriesCloud\">");
    for (series, _) in series_slice.iter() {
        page_content.push_str(&format!(
            "<a class=\"seriesbtn btn btn-primary\" onClick=\"setFilter('series::{}', 1)\">{}</a>",
            series, series
        ));
    }
    page_content.push_str("</p>");
    page_content.push_str("</div></div></div></div>");
    page_content
}
