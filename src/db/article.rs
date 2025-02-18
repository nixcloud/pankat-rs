use crate::articles::ArticleWithTags;
use crate::articles::NewArticle;

use crate::db::schema;

use crate::db::schema::tags::dsl as tags_objects;
use crate::db::schema::tags::dsl::tags as tags_table;

use crate::db::schema::articles::dsl as articles_objects;
use crate::db::schema::articles::dsl::articles as articles_table;

use crate::db::schema::article_tags::dsl as article_tags_objects;
use crate::db::schema::article_tags::dsl::article_tags as article_tags_table;

use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types::Nullable;

use chrono::NaiveDateTime;
use std::collections::HashSet;

#[derive(Queryable, Insertable, Identifiable, Selectable, Debug, Clone, PartialEq)]
#[diesel(table_name = schema::articles)]
pub struct Article {
    pub id: i32,
    pub src_file_name: String,
    pub dst_file_name: String,
    pub title: Option<String>,
    pub modification_date: Option<NaiveDateTime>,
    pub summary: Option<String>,
    pub series: Option<String>,
    pub draft: Option<bool>,
    pub special_page: Option<bool>,
    pub timeline: Option<bool>,
    pub anchorjs: Option<bool>,
    pub tocify: Option<bool>,
    pub live_updates: Option<bool>,
}

impl From<Article> for ArticleWithTags {
    fn from(article: Article) -> Self {
        ArticleWithTags {
            id: Some(article.id),
            src_file_name: article.src_file_name,
            dst_file_name: article.dst_file_name,
            title: article.title,
            modification_date: article.modification_date,
            summary: article.summary,
            series: article.series,
            draft: article.draft,
            special_page: article.special_page,
            timeline: article.timeline,
            anchorjs: article.anchorjs,
            tocify: article.tocify,
            live_updates: article.live_updates,
            tags: None,
        }
    }
}
#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(belongs_to(Article, foreign_key = article_id))]
#[diesel(table_name = schema::tags)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Identifiable, Insertable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Article))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = schema::article_tags)]
#[diesel(primary_key(article_id, tag_id))]
pub struct ArticleTag {
    pub article_id: i32,
    pub tag_id: i32,
}

pub fn get_tags_for_article(
    conn: &mut SqliteConnection,
    article_id: i32,
) -> Result<Option<Vec<String>>, diesel::result::Error> {
    let tag_ids_result: QueryResult<Vec<i32>> = article_tags_table
        .filter(article_tags_objects::article_id.eq(article_id))
        .select(article_tags_objects::tag_id)
        .load::<i32>(conn);
    match tag_ids_result {
        Ok(tag_ids) => {
            let mut tag_names = Vec::new();
            for tag_id in tag_ids {
                let name_result: QueryResult<String> = tags_table
                    .filter(tags_objects::id.eq(tag_id))
                    .select(tags_objects::name)
                    .first(conn);
                if let Ok(t_name) = name_result {
                    tag_names.push(t_name);
                }
            }
            Ok(Some(tag_names))
        }
        Err(e) => Err(e),
    }
}

pub fn get_article_with_tags_by_id(
    conn: &mut SqliteConnection,
    article_id: i32,
) -> Result<Option<ArticleWithTags>, diesel::result::Error> {
    let res = articles_table
        .filter(articles_objects::id.eq(article_id))
        .first::<Article>(conn);
    match res {
        Ok(article) => {
            let mut article_with_tags: ArticleWithTags = article.clone().into();
            match get_tags_for_article(conn, article.id) {
                Ok(tags) => {
                    article_with_tags.tags = tags;
                    Ok(Some(article_with_tags))
                }
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

pub fn get_article_with_tags_by_src_file_name(
    conn: &mut SqliteConnection,
    src_file_name: String,
) -> Result<Option<ArticleWithTags>, diesel::result::Error> {
    let res = articles_table
        .filter(articles_objects::src_file_name.eq(src_file_name))
        .first::<Article>(conn);
    match res {
        Ok(article) => {
            let mut article_with_tags: ArticleWithTags = article.clone().into();
            match get_tags_for_article(conn, article.id) {
                Ok(tags) => {
                    article_with_tags.tags = tags;
                    Ok(Some(article_with_tags))
                }
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

// func (a *ArticlesDb) MostRecentArticle() (Article, error) {
pub fn get_most_recent_article(
    conn: &mut SqliteConnection,
) -> Result<Option<ArticleWithTags>, diesel::result::Error> {
    let res = articles_table
        .filter(
            articles_objects::draft
                .eq(false)
                .or(articles_objects::draft.is_null()),
        )
        .filter(
            articles_objects::special_page
                .eq(false)
                .or(articles_objects::special_page.is_null()),
        )
        .order((
            sql::<Nullable<diesel::sql_types::Timestamp>>("modification_date IS NULL"),
            articles_objects::modification_date.asc(),
        ))
        .first::<Article>(conn);
    match res {
        Ok(article) => {
            let mut article_with_tags: ArticleWithTags = article.clone().into();
            match get_tags_for_article(conn, article.id) {
                Ok(tags) => {
                    article_with_tags.tags = tags;
                    Ok(Some(article_with_tags))
                }
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

// func (a *ArticlesDb) QueryAll() ([]Article, error) {
pub fn get_all_articles(
    conn: &mut SqliteConnection,
) -> Result<Vec<ArticleWithTags>, diesel::result::Error> {
    let res: QueryResult<Vec<Article>> = articles_table
        .order((
            sql::<Nullable<diesel::sql_types::Timestamp>>("modification_date IS NULL"),
            articles_objects::modification_date.asc(),
        ))
        .load(conn);
    match res {
        Ok(articles) => {
            let mut results = Vec::new();
            for article in articles.iter() {
                let mut article_with_tags: ArticleWithTags = article.clone().into();
                match get_tags_for_article(conn, article.id) {
                    Ok(tags) => {
                        article_with_tags.tags = tags;
                        results.push(article_with_tags)
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(results)
        }
        Err(e) => Err(e),
    }
}

//func (a *ArticlesDb) Articles() ([]Article, error) { -> all articles, except drafts / special pages
pub fn get_visible_articles(
    conn: &mut SqliteConnection,
) -> Result<Vec<ArticleWithTags>, diesel::result::Error> {
    // FIXME rewrite most functions to this return type
    let articles_query = articles_table
        .filter(
            articles_objects::draft
                .eq(false)
                .or(articles_objects::draft.is_null()),
        )
        .filter(
            articles_objects::special_page
                .eq(false)
                .or(articles_objects::special_page.is_null()),
        )
        .order((
            sql::<Nullable<diesel::sql_types::Timestamp>>("modification_date IS NULL"),
            articles_objects::modification_date.asc(),
        ))
        .load::<Article>(conn);
    match articles_query {
        Ok(articles) => {
            let mut articles_out: Vec<ArticleWithTags> = Vec::new();
            for article in articles {
                let mut article_with_tags: ArticleWithTags = article.clone().into();
                match get_tags_for_article(conn, article.id) {
                    Ok(tags) => {
                        article_with_tags.tags = tags;
                        articles_out.push(article_with_tags);
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(articles_out)
        }
        Err(e) => Err(e),
    }
}

// func (a *ArticlesDb) ArticlesBySeries(series string) ([]Article, error) {
pub fn get_visible_articles_by_series(
    conn: &mut SqliteConnection,
    series: &str,
) -> Result<Vec<ArticleWithTags>, diesel::result::Error> {
    let res = articles_table
        .filter(articles_objects::series.eq(series))
        .filter(
            articles_objects::draft
                .eq(false)
                .or(articles_objects::draft.is_null()),
        )
        .filter(
            articles_objects::special_page
                .eq(false)
                .or(articles_objects::special_page.is_null()),
        )
        .order((
            sql::<Nullable<diesel::sql_types::Timestamp>>("modification_date IS NULL"),
            articles_objects::modification_date.asc(),
        ))
        .load::<Article>(conn);
    match res {
        Ok(articles) => {
            let mut articles_out: Vec<ArticleWithTags> = Vec::new();
            for article in articles {
                let mut article_with_tags: ArticleWithTags = article.clone().into();
                match get_tags_for_article(conn, article.id) {
                    Ok(tags) => {
                        article_with_tags.tags = tags;
                        articles_out.push(article_with_tags);
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(articles_out)
        }
        Err(e) => Err(e),
    }
}

// func (a *ArticlesDb) ArticlesByTag(tagName string) ([]Article, error) {
pub fn get_visible_articles_by_tag(
    conn: &mut SqliteConnection,
    tag: String,
) -> Result<Vec<ArticleWithTags>, diesel::result::Error> {
    let res = articles_table
        .inner_join(
            article_tags_table.on(articles_objects::id.eq(article_tags_objects::article_id)),
        )
        .inner_join(tags_table.on(article_tags_objects::tag_id.eq(tags_objects::id)))
        .filter(tags_objects::name.eq(tag))
        .filter(
            articles_objects::draft
                .eq(false)
                .or(articles_objects::draft.is_null()),
        )
        .select(articles_table::all_columns())
        .load::<Article>(conn);
    match res {
        Ok(articles) => {
            let mut articles_out: Vec<ArticleWithTags> = Vec::new();
            for article in articles {
                let mut article_with_tags: ArticleWithTags = article.clone().into();
                match get_tags_for_article(conn, article.id) {
                    Ok(tags) => {
                        article_with_tags.tags = tags;
                        articles_out.push(article_with_tags);
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(articles_out)
        }
        Err(e) => Err(e),
    }
}

// func (a *ArticlesDb) Drafts() ([]Article, error) {
pub fn get_drafts(
    conn: &mut SqliteConnection,
) -> Result<Vec<ArticleWithTags>, diesel::result::Error> {
    let res = articles_table
        .filter(articles_objects::draft.eq(true))
        .order((
            sql::<Nullable<diesel::sql_types::Timestamp>>("modification_date IS NULL"),
            articles_objects::modification_date.asc(),
        ))
        .load::<Article>(conn);
    match res {
        Ok(articles) => {
            let mut articles_out: Vec<ArticleWithTags> = Vec::new();
            for article in articles {
                let mut article_with_tags: ArticleWithTags = article.clone().into();
                match get_tags_for_article(conn, article.id) {
                    Ok(tags) => {
                        article_with_tags.tags = tags;
                        articles_out.push(article_with_tags);
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(articles_out)
        }
        Err(e) => Err(e),
    }
}

// func (a *ArticlesDb) SpecialPages() ([]Article, error) {
pub fn get_special_pages(
    conn: &mut SqliteConnection,
) -> Result<Vec<ArticleWithTags>, diesel::result::Error> {
    let res = articles_table
        .filter(articles_objects::special_page.eq(true))
        .order((
            sql::<Nullable<diesel::sql_types::Timestamp>>("modification_date IS NULL"),
            articles_objects::modification_date.asc(),
        ))
        .load::<Article>(conn);
    match res {
        Ok(articles) => {
            let mut articles_out: Vec<ArticleWithTags> = Vec::new();
            for article in articles {
                let mut article_with_tags: ArticleWithTags = article.clone().into();
                match get_tags_for_article(conn, article.id) {
                    Ok(tags) => {
                        article_with_tags.tags = tags;
                        articles_out.push(article_with_tags);
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(articles_out)
        }
        Err(e) => Err(e),
    }
}

fn tag_difference<'a>(
    tags_a: &'a Option<Vec<String>>,
    tags_b: &'a Option<Vec<String>>,
) -> Vec<String> {
    use std::collections::HashMap;

    let mut map: HashMap<&str, i32> = HashMap::new();

    if let Some(tags) = tags_a {
        for tag in tags {
            *map.entry(tag.as_str()).or_insert(0) += 1;
        }
    }

    if let Some(tags) = tags_b {
        for tag in tags {
            *map.entry(tag.as_str()).or_insert(0) -= 1;
        }
    }

    map.into_iter()
        .filter(|&(_, count)| count > 0)
        .map(|(tag, _)| tag.to_string())
        .collect()
}

// func (a *ArticlesDb) Set(article *Article) (*Article, []string, error) {
pub fn set(
    conn: &mut SqliteConnection,
    new_article_with_tags: &ArticleWithTags,
) -> Result<HashSet<i32>, diesel::result::Error> {
    let new_article: NewArticle = new_article_with_tags.clone().into();
    let new_tags: Option<Vec<String>> = new_article_with_tags.tags.clone();
    if new_article_with_tags.src_file_name.is_empty() {
        return Err(diesel::result::Error::NotFound);
    }
    if new_article_with_tags.dst_file_name.is_empty() {
        return Err(diesel::result::Error::NotFound);
    }
    conn.transaction(|conn| {
        let existing_article_reply = articles_table
            .filter(articles_objects::src_file_name.eq(new_article_with_tags.src_file_name.clone()))
            .get_result::<Article>(conn);

        if let Ok(existing_article) = existing_article_reply {
            let mut existing_article_with_tags: ArticleWithTags = existing_article.clone().into();
            let existing_article_id = existing_article.id;

            let affected_articles_before: AllArticleNeighbours =
                get_neighbours_helper(conn, existing_article_id).unwrap();

            //println!("{:#?}", affected_articles_after);

            match get_tags_for_article(conn, existing_article_id) {
                Ok(tags) => {
                    existing_article_with_tags.tags = tags;
                }
                Err(e) => return Err(e),
            }
            // update existing article
            //println!("update existing article");

            let _ =
                diesel::update(articles_table.filter(articles_objects::id.eq(existing_article_id)))
                    .set(&new_article)
                    .execute(conn)
                    .unwrap();

            let affected_articles_after: AllArticleNeighbours =
                get_neighbours_helper(conn, existing_article_id).unwrap();

            // update tags
            //println!("update tags");

            let tags_to_remove: Vec<String> =
                tag_difference(&existing_article_with_tags.tags, &new_tags);
            let tags_to_add: Vec<String> =
                tag_difference(&new_tags, &existing_article_with_tags.tags);

            for tag_name in tags_to_remove {
                let tag_id_res: QueryResult<i32> = tags_table
                    .filter(tags_objects::name.eq(&tag_name))
                    .select(tags_objects::id)
                    .first(conn);
                if let Ok(tag_id) = tag_id_res {
                    let _ = diesel::delete(
                        article_tags_table.filter(
                            article_tags_objects::article_id
                                .eq(existing_article_id)
                                .and(article_tags_objects::tag_id.eq(tag_id)),
                        ),
                    )
                    .execute(conn);
                }
            }

            for tag_name in tags_to_add {
                let tag_result = diesel::insert_into(tags_table)
                    .values(tags_objects::name.eq(&tag_name))
                    .on_conflict(tags_objects::name)
                    .do_nothing()
                    .get_result::<Tag>(conn);
                let tag_id: i32 = match tag_result {
                    Ok(tag) => tag.id,
                    Err(_) => tags_table
                        .filter(tags_objects::name.eq(&tag_name))
                        .select(tags_objects::id)
                        .first::<i32>(conn)
                        .unwrap(),
                };
                let article_tag = ArticleTag {
                    article_id: existing_article_id,
                    tag_id,
                };
                let _ = diesel::insert_into(article_tags_table)
                    .values(article_tag)
                    .execute(conn);
            }

            let affected_articles = affected_articles_before.diff(&affected_articles_after);
            //affected_articles.remove(&existing_article_id);
            Ok(affected_articles)
        } else {
            let most_recent_article = match get_most_recent_article(conn) {
                Ok(article_option) => article_option,
                Err(_) => None,
            };
            let mut affected_articles_before: AllArticleNeighbours = AllArticleNeighbours::new();
            affected_articles_before.most_recent_article = most_recent_article;

            //println!("creating new article");
            // insert as new article
            let articles_result: Result<Vec<Article>, diesel::result::Error> =
                diesel::insert_into(articles_table)
                    .values(new_article)
                    .get_results::<Article>(conn);
            match articles_result {
                Ok(articles_result) => {
                    let article_id: i32 = articles_result[0].id; // FIXME error handling
                    if let Some(tags) = new_article_with_tags.tags.clone() {
                        // add to tags table and reference it in article_tags table
                        for tag in tags.iter() {
                            let tag_result = diesel::insert_into(tags_table)
                                .values(tags_objects::name.eq(tag))
                                .on_conflict(tags_objects::name)
                                .do_nothing()
                                .get_result::<Tag>(conn);
                            let tag_id: i32 = match tag_result {
                                Ok(_) => {
                                    // If the insert was successful, query the inserted tag to get its ID
                                    let inserted_tag = tags_table
                                        .filter(tags_objects::name.eq(tag))
                                        .select(tags_objects::id)
                                        .first::<i32>(conn);
                                    inserted_tag.unwrap()
                                }
                                Err(_) => {
                                    // If the insert failed due to a conflict, query the existing tag by name and get its ID
                                    let existing_tag = tags_table
                                        .filter(tags_objects::name.eq(tag))
                                        .select(tags_objects::id)
                                        .first::<i32>(conn);
                                    existing_tag.unwrap()
                                }
                            };
                            let article_tag: ArticleTag = ArticleTag { article_id, tag_id };
                            //println!(" -> {} - {:?}", tag, article_tag);
                            let _ = diesel::insert_into(article_tags_table)
                                .values(article_tag)
                                .execute(conn);
                        }
                    }
                    let affected_articles_after: AllArticleNeighbours =
                        get_neighbours_helper(conn, article_id).unwrap();
                    let affected_articles = affected_articles_before.diff(&affected_articles_after);
                    return Ok(affected_articles);
                }
                Err(e) => {
                    println!("Error on creating article: {:?}", e);
                    return Err(e);
                }
            };
        }
    })
}

// func (a *ArticlesDb) Del(SrcFileName string) ([]string, error) {
pub fn del_by_src_file_name(
    conn: &mut SqliteConnection,
    src_file_name: String,
) -> Result<HashSet<i32>, diesel::result::Error> {
    let res: QueryResult<i32> = articles_table
        .filter(articles_objects::src_file_name.eq(src_file_name.clone()))
        .select(articles_objects::id)
        .first(conn);

    match res {
        Ok(id) => del_by_id(conn, id),
        Err(e) => Err(e),
    }
}
fn get_neighbours_helper(
    conn: &mut SqliteConnection,
    id: i32,
) -> Result<AllArticleNeighbours, diesel::result::Error> {
    let mut all_article_neighbours = AllArticleNeighbours::new();
    match get_most_recent_article(conn) {
        Ok(article_option) => {
            all_article_neighbours.most_recent_article = article_option;
        }
        Err(_) => {}
    }
    match get_prev_and_next_article(conn, id) {
        Ok(neighbours) => {
            all_article_neighbours.prev_next_article = neighbours;
        }
        Err(e) => return Err(e),
    }
    match get_prev_and_next_article_for_series(conn, id) {
        Ok(neighbours) => {
            all_article_neighbours.prev_next_article_series = neighbours;
        }
        Err(e) => return Err(e),
    }
    Ok(all_article_neighbours)
}

pub fn del_by_id(
    conn: &mut SqliteConnection,
    id: i32,
) -> Result<HashSet<i32>, diesel::result::Error> {
    let affected_articles_before: AllArticleNeighbours = get_neighbours_helper(conn, id).unwrap();
    //println!("{:#?}", affected_articles_before);
    let num_deleted =
        diesel::delete(articles_table.filter(articles_objects::id.eq(id))).execute(conn);
    match num_deleted {
        Ok(0) => {
            return Err(diesel::result::Error::NotFound);
        }
        Ok(_) => {
            let most_recent_article = match get_most_recent_article(conn) {
                Ok(article_option) => article_option,
                Err(_) => None,
            };
            let mut affected_articles_after: AllArticleNeighbours = AllArticleNeighbours::new();
            affected_articles_after.most_recent_article = most_recent_article;
            //println!("{:#?}", affected_articles_after);
            let mut affected_articles = affected_articles_before.diff(&affected_articles_after);
            affected_articles.remove(&id);
            Ok(affected_articles)
        }
        Err(e) => Err(e),
    }
}

// func (a *ArticlesDb) AllTagsInDB() ([]string, error) {
pub fn get_all_tags(conn: &mut SqliteConnection) -> Result<Vec<String>, diesel::result::Error> {
    let res = tags_table.select(tags_objects::name).load(conn);
    match res {
        Ok(tags) => Ok(tags),
        Err(e) => Err(e),
    }
}

// func (a *ArticlesDb) AllSeriesInDB() ([]string, error) {
pub fn get_all_series_from_visible_articles(
    conn: &mut SqliteConnection,
) -> Result<Vec<String>, diesel::result::Error> {
    let res: QueryResult<Vec<Article>> = articles_table
        .filter(articles_objects::series.is_not_null())
        .filter(
            articles_objects::draft
                .eq(false)
                .or(articles_objects::draft.is_null()),
        )
        .order((
            sql::<Nullable<diesel::sql_types::Timestamp>>("modification_date IS NULL"),
            articles_objects::modification_date.asc(),
        ))
        .load(conn);
    match res {
        Ok(articles) => {
            let mut results: Vec<String> = Vec::new();
            for article in articles.iter() {
                match &article.series.clone() {
                    Some(series) => {
                        results.push(series.clone());
                    }
                    None => {}
                }
            }
            Ok(results)
        }
        Err(e) => Err(e),
    }
}

#[derive(Debug)]
pub struct AllArticleNeighbours {
    most_recent_article: Option<ArticleWithTags>,
    prev_next_article: ArticleNeighbours,
    prev_next_article_series: ArticleNeighbours,
}

impl AllArticleNeighbours {
    pub fn new() -> Self {
        AllArticleNeighbours {
            most_recent_article: None,
            prev_next_article: ArticleNeighbours::new(),
            prev_next_article_series: ArticleNeighbours::new(),
        }
    }

    pub fn diff(&self, other: &AllArticleNeighbours) -> HashSet<i32> {
        let mut differences = HashSet::new();

        if self.most_recent_article != other.most_recent_article {
            if let Some(article) = &self.most_recent_article {
                differences.insert(article.id.unwrap());
            }
            if let Some(article) = &other.most_recent_article {
                differences.insert(article.id.unwrap());
            }
        }
        for id in self.prev_next_article.diff(&other.prev_next_article) {
            differences.insert(id);
        }
        for id in self
            .prev_next_article_series
            .diff(&other.prev_next_article_series)
        {
            differences.insert(id);
        }
        differences
    }
}

#[derive(Debug)]
pub struct ArticleNeighbours {
    pub prev: Option<ArticleWithTags>,
    pub next: Option<ArticleWithTags>,
}

impl ArticleNeighbours {
    pub fn new() -> Self {
        ArticleNeighbours {
            prev: None,
            next: None,
        }
    }
    pub fn diff(&self, other: &ArticleNeighbours) -> HashSet<i32> {
        let mut differences = HashSet::new();
        if &self.prev != &other.prev {
            match &self.prev {
                Some(article) => {
                    differences.insert(article.id.unwrap());
                }
                None => {}
            }
        }
        if &self.next != &other.next {
            match &self.next {
                Some(article) => {
                    differences.insert(article.id.unwrap());
                }
                None => {}
            }
        }
        differences
    }
}

fn find_prev_and_next_articles(
    conn: &mut SqliteConnection,
    articles: &Vec<Article>,
    id: i32,
) -> Result<ArticleNeighbours, diesel::result::Error> {
    let mut prev_article: Option<ArticleWithTags> = None;
    let mut next_article: Option<ArticleWithTags> = None;
    if let Some(pos) = articles.iter().position(|article| article.id == id) {
        if pos > 0 {
            let p: Article = articles[pos - 1].clone();
            let mut prev: ArticleWithTags = p.clone().into();
            match get_tags_for_article(conn, p.id) {
                Ok(tags) => {
                    prev.tags = tags;
                    prev_article = Some(prev);
                }
                Err(e) => return Err(e),
            }
        }
        if pos < articles.len() - 1 {
            let n: Article = articles[pos + 1].clone();
            let mut next: ArticleWithTags = n.clone().into();
            match get_tags_for_article(conn, n.id) {
                Ok(tags) => {
                    next.tags = tags;
                    next_article = Some(next);
                }
                Err(e) => return Err(e),
            }
        }
    }

    let n = ArticleNeighbours {
        prev: prev_article,
        next: next_article,
    };
    Ok(n)
}

// func (a *ArticlesDb) NextArticle(article Article) (*Article, error) {
// func (a *ArticlesDb) PrevArticle(article Article) (*Article, error) {
pub fn get_prev_and_next_article(
    conn: &mut SqliteConnection,
    id: i32,
) -> Result<ArticleNeighbours, diesel::result::Error> {
    let articles_query: QueryResult<Vec<Article>> = articles_table
        .filter(
            articles_objects::draft
                .eq(false)
                .or(articles_objects::draft.is_null()),
        )
        .filter(
            articles_objects::special_page
                .eq(false)
                .or(articles_objects::special_page.is_null()),
        )
        .order((
            sql::<Nullable<diesel::sql_types::Timestamp>>("modification_date IS NULL"),
            articles_objects::modification_date.asc(),
        ))
        .load::<Article>(conn);
    match articles_query {
        Ok(articles) => find_prev_and_next_articles(conn, &articles, id),
        Err(e) => {
            println!("Error: {}", e);
            Err(e)
        }
    }
}

// func (a *ArticlesDb) NextArticleInSeries(article Article) (Article, error) {
// func (a *ArticlesDb) PrevArticleInSeries(article Article) (Article, error) {
pub fn get_prev_and_next_article_for_series(
    conn: &mut SqliteConnection,
    id: i32,
) -> Result<ArticleNeighbours, diesel::result::Error> {
    let res = articles_table
        .select(articles_objects::series)
        .first::<Option<String>>(conn);

    match res {
        Ok(series_option) => match series_option {
            Some(series) => {
                let articles_query: QueryResult<Vec<Article>> = articles_table
                    .filter(articles_objects::series.eq(series))
                    .filter(
                        articles_objects::draft
                            .eq(false)
                            .or(articles_objects::draft.is_null()),
                    )
                    .filter(
                        articles_objects::special_page
                            .eq(false)
                            .or(articles_objects::special_page.is_null()),
                    )
                    .order((
                        sql::<Nullable<diesel::sql_types::Timestamp>>("modification_date IS NULL"),
                        articles_objects::modification_date.asc(),
                    ))
                    .load::<Article>(conn);

                match articles_query {
                    Ok(articles) => find_prev_and_next_articles(conn, &articles, id),
                    Err(e) => {
                        println!("Error: {}", e);
                        Err(e)
                    }
                }
            }
            None => {
                println!("No series found for article with id {}", id);
                Ok(ArticleNeighbours::new())
            }
        },
        Err(e) => {
            println!("Error loading article from db: {}", e);
            Err(e)
        }
    }
}

// func (a *ArticlesDb) GetRelatedArticles(article Article) map[string]bool {
// func (a *ArticlesDb) QueryRawBySrcFileName(SrcFileName string) (*Article, error) {
