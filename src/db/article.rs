use crate::articles::NewArticle;
use crate::db::schema::{articles, tags};
use chrono::NaiveDateTime;
use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types::Nullable;

#[derive(Queryable, Insertable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = articles)]
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

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = tags)]
pub struct Tags {
    pub id: i32,
    pub name: String,
    pub article_id: i32,
}

// func (a *ArticlesDb) MostRecentArticle() (Article, error) {
pub fn get_most_recent_article(conn: &mut SqliteConnection) -> QueryResult<Option<Article>> {
    use crate::db::schema::articles::dsl::*;
    articles
        .filter(draft.eq(false))
        .filter(special_page.eq(false))
        .order((
            sql::<Nullable<diesel::sql_types::Timestamp>>("modification_date IS NULL"),
            modification_date.desc(),
        ))
        .first(conn)
        .optional()
}

// func (a *ArticlesDb) QueryAll() ([]Article, error) {
pub fn get_all_articles(conn: &mut SqliteConnection) -> QueryResult<Vec<Article>> {
    use crate::db::schema::articles::dsl::*;
    articles
        .order((
            sql::<Nullable<diesel::sql_types::Timestamp>>("modification_date IS NULL"),
            modification_date.desc(),
        ))
        .load(conn)
}

//func (a *ArticlesDb) Articles() ([]Article, error) { -> all articles, except drafts / special pages
pub fn get_visible_articles(conn: &mut SqliteConnection) -> QueryResult<Vec<Article>> {
    use crate::db::schema::articles::dsl::*;
    articles
        .filter(draft.eq(false))
        .filter(special_page.eq(false))
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
pub fn set(conn: &mut SqliteConnection, new_article: &NewArticle) -> QueryResult<i32> {
    diesel::insert_into(articles::table)
        .values(new_article)
        .execute(conn)?;

    articles::table
        .select(articles::id)
        .order(articles::id.desc())
        .first(conn)
}

// func (a *ArticlesDb) Del(SrcFileName string) ([]string, error) {
// func (a *ArticlesDb) QueryRawBySrcFileName(SrcFileName string) (*Article, error) {
// func (a *ArticlesDb) NextArticle(article Article) (*Article, error) {
// func (a *ArticlesDb) PrevArticle(article Article) (*Article, error) {

// func (a *ArticlesDb) SetCache(article Article, generatedHTML string) error {
// func (a *ArticlesDb) GetCache(article Article) (string, error) {

// func (a *ArticlesDb) GetRelatedArticles(article Article) map[string]bool {
// func (a *ArticlesDb) AllTagsInDB() ([]string, error) {
// func (a *ArticlesDb) ArticlesByTag(tagName string) ([]Article, error) {
// func (a *ArticlesDb) AllSeriesInDB() ([]string, error) {
// func (a *ArticlesDb) ArticlesBySeries(series string) ([]Article, error) {
// func (a *ArticlesDb) NextArticleInSeries(article Article) (Article, error) {
// func (a *ArticlesDb) PrevArticleInSeries(article Article) (Article, error) {

// won't implement
// func (a *ArticlesDb) Contains(DstFileName string) (bool, error) {
