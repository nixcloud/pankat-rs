use crate::articles::ArticleWithTags;
use crate::db::schema::{article_tags, articles, tags};
use chrono::NaiveDateTime;
use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types::Nullable;

#[derive(Queryable, Insertable, Identifiable, Selectable, Debug, Clone, PartialEq)]
#[diesel(table_name = articles)]
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
            id: None,
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
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Article))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = article_tags)]
#[diesel(primary_key(article_id, tag_id))]
pub struct ArticleTags {
    pub article_id: i32,
    pub tag_id: i32,
}

// func (a *ArticlesDb) MostRecentArticle() (Article, error) {

// FIXME to: Result<ArticleWithTags>
pub fn get_most_recent_article(conn: &mut SqliteConnection) -> QueryResult<Option<Article>> {
    use crate::db::schema::articles::dsl::*;
    articles
        .filter(draft.eq(false).or(draft.is_null()))
        .filter(special_page.eq(false).or(special_page.is_null()))
        .order((
            sql::<Nullable<diesel::sql_types::Timestamp>>("modification_date IS NULL"),
            modification_date.desc(),
        ))
        .first(conn)
        .optional()
}

// func (a *ArticlesDb) QueryAll() ([]Article, error) {
pub fn get_all_articles(
    conn: &mut SqliteConnection,
) -> Result<Vec<ArticleWithTags>, diesel::result::Error> {
    use crate::db::schema::articles::dsl::*;
    let article_list: QueryResult<Vec<Article>> = articles
        .order((
            sql::<Nullable<diesel::sql_types::Timestamp>>("modification_date IS NULL"),
            modification_date.desc(),
        ))
        .load(conn);
    match article_list {
        Ok(list) => {
            let mut results = Vec::new();
            for article in list.iter() {
                let a_w_t: ArticleWithTags = article.clone().into();
                // FIXME add the tags query
                results.push(a_w_t)
            }
            Ok(results)
        }
        Err(e) => Err(e),
    }
}

//func (a *ArticlesDb) Articles() ([]Article, error) { -> all articles, except drafts / special pages
pub fn get_visible_articles(conn: &mut SqliteConnection) -> QueryResult<Vec<Article>> {
    use crate::db::schema::articles::dsl::*;
    articles
        .filter(draft.eq(false).or(draft.is_null()))
        .filter(special_page.eq(false).or(special_page.is_null()))
        .order((
            sql::<Nullable<diesel::sql_types::Timestamp>>("modification_date IS NULL"),
            modification_date.desc(),
        ))
        .load(conn)
}

// func (a *ArticlesDb) Drafts() ([]Article, error) {
pub fn get_drafts(conn: &mut SqliteConnection) -> QueryResult<Vec<Article>> {
    use crate::db::schema::articles::dsl::*;
    articles
        .filter(draft.eq(true))
        .order((
            sql::<Nullable<diesel::sql_types::Timestamp>>("modification_date IS NULL"),
            modification_date.desc(),
        ))
        .load(conn)
}

// func (a *ArticlesDb) SpecialPages() ([]Article, error) {
pub fn get_special_pages(conn: &mut SqliteConnection) -> QueryResult<Vec<Article>> {
    use crate::db::schema::articles::dsl::*;
    articles
        .filter(special_page.eq(true))
        .order((
            sql::<Nullable<diesel::sql_types::Timestamp>>("modification_date IS NULL"), // NULLs last
            modification_date.desc(),
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
pub fn set(conn: &mut SqliteConnection, new_article: &ArticleWithTags) -> QueryResult<usize> {
    use crate::db::schema::{article_tags, articles, tags};
    let article: Article = new_article.clone().into();

    diesel::insert_into(articles::table)
        .values(article)
        .execute(conn)
    // match a {
    //     Ok(s) => {},
    //     Err(e) => match e {

    //     }
    // }

    //let tags_list: Vec<String> = new_article.tags.clone().unwrap_or_default();
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
    use crate::db::schema::articles::dsl::*;
    articles
        .filter(draft.eq(false).or(draft.is_null()))
        .filter(special_page.eq(false).or(special_page.is_null()))
        .filter(series.eq(series))
        .order((
            sql::<Nullable<diesel::sql_types::Timestamp>>("modification_date IS NULL"),
            modification_date.desc(),
        ))
        .load(conn)
}

// func (a *ArticlesDb) AllTagsInDB() ([]string, error) {
pub fn get_all_tags(conn: &mut SqliteConnection) -> QueryResult<Vec<String>> {
    use crate::db::schema::tags::dsl::*;

    tags.select(name).load(conn)
}
// func (a *ArticlesDb) ArticlesByTag(tagName string) ([]Article, error) {
pub fn get_all_artigles_by_tag(
    conn: &mut SqliteConnection,
    tag: String,
) -> QueryResult<Vec<String>> {
    use crate::db::schema::tags::dsl::*;

    tags.select(name).load(conn)
}

// func (a *ArticlesDb) QueryRawBySrcFileName(SrcFileName string) (*Article, error) {
// func (a *ArticlesDb) NextArticle(article Article) (*Article, error) {
// func (a *ArticlesDb) PrevArticle(article Article) (*Article, error) {

// func (a *ArticlesDb) SetCache(article Article, generatedHTML string) error {
// func (a *ArticlesDb) GetCache(article Article) (string, error) {

// func (a *ArticlesDb) GetRelatedArticles(article Article) map[string]bool {

// func (a *ArticlesDb) AllSeriesInDB() ([]string, error) {

// func (a *ArticlesDb) NextArticleInSeries(article Article) (Article, error) {
// func (a *ArticlesDb) PrevArticleInSeries(article Article) (Article, error) {
