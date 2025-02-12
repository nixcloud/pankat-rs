use crate::articles::ArticleWithTags;

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

#[derive(Queryable, Insertable, Identifiable, Selectable, Debug, Clone, PartialEq)]
#[diesel(table_name = schema::articles)]
pub struct Article {
    pub id: Option<i32>,
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

impl From<ArticleWithTags> for Article {
    fn from(new_article_with_tags: ArticleWithTags) -> Self {
        Article {
            id: new_article_with_tags.id,
            src_file_name: new_article_with_tags.src_file_name,
            dst_file_name: new_article_with_tags.dst_file_name,
            title: new_article_with_tags.title,
            modification_date: new_article_with_tags.modification_date,
            summary: new_article_with_tags.summary,
            series: new_article_with_tags.series,
            draft: new_article_with_tags.draft,
            special_page: new_article_with_tags.special_page,
            timeline: new_article_with_tags.timeline,
            anchorjs: new_article_with_tags.anchorjs,
            tocify: new_article_with_tags.tocify,
            live_updates: new_article_with_tags.live_updates,
        }
    }
}

impl From<Article> for ArticleWithTags {
    fn from(article: Article) -> Self {
        ArticleWithTags {
            id: article.id,
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
    pub id: Option<i32>,
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

// func (a *ArticlesDb) MostRecentArticle() (Article, error) {

// FIXME to: Result<ArticleWithTags>
pub fn get_most_recent_article(conn: &mut SqliteConnection) -> QueryResult<Option<Article>> {
    articles_table
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
            articles_objects::modification_date.desc(),
        ))
        .first(conn)
        .optional()
}

// func (a *ArticlesDb) QueryAll() ([]Article, error) {
pub fn get_all_articles(
    conn: &mut SqliteConnection,
) -> Result<Vec<ArticleWithTags>, diesel::result::Error> {
    let article_list: QueryResult<Vec<Article>> = articles_table
        .order((
            sql::<Nullable<diesel::sql_types::Timestamp>>("modification_date IS NULL"),
            articles_objects::modification_date.desc(),
        ))
        .load(conn);
    match article_list {
        Ok(list) => {
            let mut results = Vec::new();
            for article in list.iter() {
                let article_with_tags: ArticleWithTags = article.clone().into();
                results.push(article_with_tags)
            }
            Ok(results)
        }
        Err(e) => Err(e),
    }
}

// SELECT t.name AS tag_name
// FROM article_tags at
// JOIN tags t ON at.tag_id = t.id
// WHERE at.article_id = 1;

fn get_tags_for_article(article_id: i32, conn: &mut SqliteConnection) -> Option<Vec<String>> {
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
            Some(tag_names)
        }
        Err(_) => None,
    }
}

//func (a *ArticlesDb) Articles() ([]Article, error) { -> all articles, except drafts / special pages
pub fn get_visible_articles(
    conn: &mut SqliteConnection,
) -> Result<Vec<ArticleWithTags>, diesel::result::Error> {
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
            articles_objects::modification_date.desc(),
        ))
        .load::<Article>(conn);

    match articles_query {
        Ok(articles) => {
            let mut articles_out: Vec<ArticleWithTags> = Vec::new();
            for article_in in articles {
                let mut article_with_tags: ArticleWithTags = article_in.clone().into();
                match article_in.id {
                    Some(article_id) => {
                        article_with_tags.tags = get_tags_for_article(article_id, conn);
                    }
                    None => {}
                }
                articles_out.push(article_with_tags);
            }
            Ok(articles_out)
        }
        Err(e) => Err(e),
    }
}

// func (a *ArticlesDb) Drafts() ([]Article, error) {
pub fn get_drafts(conn: &mut SqliteConnection) -> QueryResult<Vec<Article>> {
    articles_table
        .filter(articles_objects::draft.eq(true))
        .order((
            sql::<Nullable<diesel::sql_types::Timestamp>>("modification_date IS NULL"),
            articles_objects::modification_date.desc(),
        ))
        .load(conn)
}

// func (a *ArticlesDb) SpecialPages() ([]Article, error) {
pub fn get_special_pages(conn: &mut SqliteConnection) -> QueryResult<Vec<Article>> {
    articles_table
        .filter(articles_objects::special_page.eq(true))
        .order((
            sql::<Nullable<diesel::sql_types::Timestamp>>("modification_date IS NULL"), // NULLs last
            articles_objects::modification_date.desc(),
        ))
        .load(conn)
}

// func (a *ArticlesDb) Set(article *Article) (*Article, []string, error) {
// returns affected neighbours like:
// * next/prev neighbours
// * next/prev tags neighbours
// * next/prev series neighbours
// * most recent article (if changed)
// * add/del on draft (so leptos can display an adapted list using /ws)
// * add/del on special_pages (so leptos can display an adapted list using /ws)

// 1. check if article is exists
// 1. a) it exists, update it but track old neigbours
//  2. update tags
//  3. update article_tags bindings
//  4. finish transaction
// 1. b) it doesn't exist, create it
// follow 2./3./4.
pub fn set(conn: &mut SqliteConnection, new_article_with_tags: &ArticleWithTags) {
    let article: Article = new_article_with_tags.clone().into();

    let t = conn.transaction(|mut conn| {
        let articles_result = diesel::insert_into(articles_table)
            .values(article)
            .get_results::<Article>(conn);

        match articles_result {
            Ok(ref articles_result) => {
                let article_id: i32 = articles_result[0].id.unwrap(); // FIXME error handling

                if let Some(tags) = new_article_with_tags.tags.clone() {
                    // add to tags table and reference it in article_tags table
                    for tag in tags.iter() {
                        let tag_result = diesel::insert_into(tags_table)
                            .values(tags_objects::name.eq(tag))
                            .on_conflict(tags_objects::name)
                            .do_nothing()
                            .get_result::<Tag>(conn);

                        let tag_id: i32 = match tag_result {
                            Ok(tag_result) => {
                                // If the insert was successful, query the inserted tag to get its ID
                                let inserted_tag = tags_table
                                    .filter(tags_objects::name.eq(tag))
                                    .select(tags_objects::id)
                                    .first::<Option<i32>>(conn)
                                    .unwrap();
                                inserted_tag.unwrap()
                            }
                            Err(_) => {
                                // If the insert failed due to a conflict, query the existing tag by name and get its ID
                                let existing_tag = tags_table
                                    .filter(tags_objects::name.eq(tag))
                                    .select(tags_objects::id)
                                    .first::<Option<i32>>(conn)
                                    .unwrap();
                                existing_tag.unwrap()
                            }
                        };

                        let article_tag: ArticleTag = ArticleTag { article_id, tag_id };
                        println!(" -> {} - {:?}", tag, article_tag);

                        let _ = diesel::insert_into(article_tags_table)
                            .values(article_tag)
                            .execute(conn);
                    }
                }
            }
            Err(ref e) => {
                println!("freaking article already exists, please implement this xxx");
            }
        };

        articles_result
    });
}

// // func (a *ArticlesDb) Del(SrcFileName string) ([]string, error) {
// pub fn del(conn: &mut SqliteConnection, src_file_name: &str) -> QueryResult<usize> {
//     let s: String = src_file_name.to_string()

//     // let num_deleted = diesel::delete(
//     //     articles::table.filter(src_file_name.eq(s))
//     // )
//         //
//         //.filter(special_page.eq(false)))
//         .execute(conn);

//     num_deleted
// }

// func (a *ArticlesDb) ArticlesBySeries(series string) ([]Article, error) {
pub fn get_visible_articles_by_series(
    conn: &mut SqliteConnection,
    series: &str,
) -> QueryResult<Vec<Article>> {
    articles_table
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
        .filter(articles_objects::series.eq(series))
        .order((
            sql::<Nullable<diesel::sql_types::Timestamp>>("modification_date IS NULL"),
            articles_objects::modification_date.desc(),
        ))
        .load(conn)
}

// func (a *ArticlesDb) AllTagsInDB() ([]string, error) {
pub fn get_all_tags(conn: &mut SqliteConnection) -> QueryResult<Vec<String>> {
    tags_table.select(tags_objects::name).load(conn)
}
// func (a *ArticlesDb) ArticlesByTag(tagName string) ([]Article, error) {
pub fn get_visible_articles_by_tag(
    conn: &mut SqliteConnection,
    tag: String,
) -> QueryResult<Vec<String>> {
    tags_table.select(tags_objects::name).load(conn)
}

// func (a *ArticlesDb) QueryRawBySrcFileName(SrcFileName string) (*Article, error) {
// func (a *ArticlesDb) NextArticle(article Article) (*Article, error) {
// func (a *ArticlesDb) PrevArticle(article Article) (*Article, error) {


// func (a *ArticlesDb) GetRelatedArticles(article Article) map[string]bool {

// func (a *ArticlesDb) AllSeriesInDB() ([]string, error) {

// func (a *ArticlesDb) NextArticleInSeries(article Article) (Article, error) {
// func (a *ArticlesDb) PrevArticleInSeries(article Article) (Article, error) {
