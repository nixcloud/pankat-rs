use diesel::prelude::*;

use crate::db::schema::{articles, articles_cache, series, tags};

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = articles)]
pub struct Articles {
    pub id: i32,
    pub title: String,
    pub src_file_name: String,
    pub dst_file_name: String,
    pub modification_date: std::time::SystemTime,
    pub summary: String,
    // pub tags: None,
    // pub series: None,
    pub draft: bool,
    pub special_page: bool,
    pub timeline: bool,

    pub anchorjs: bool,
    pub tocify: bool,
    pub live_updates: bool,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = tags)]
pub struct Tags {
    pub id: i32,
    pub name: String,
    pub article_id: i32,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = series)]
pub struct Series {
    pub id: i32,
    pub name: String,
    pub article_id: i32,
}

#[derive(Queryable, Insertable)]
#[diesel(belongs_to(Articles))]
#[diesel(table_name = articles_cache)]
pub struct ArticlesCache {
    pub id: i32,
    pub hash: String,
    pub html: String,
}

// // func (a *ArticlesDb) GetRelatedArticles(article Article) map[string]bool {
// // func (a *ArticlesDb) Set(article *Article) (*Article, []string, error) {
// // func (a *ArticlesDb) Del(SrcFileName string) ([]string, error) {
// // func (a *ArticlesDb) QueryAll() ([]Article, error) {
// // func (a *ArticlesDb) QueryRawBySrcFileName(SrcFileName string) (*Article, error) {
// //func (a *ArticlesDb) Articles() ([]Article, error) { -> all articles, except drafts / special pages
// // func (a *ArticlesDb) MostRecentArticle() (Article, error) {
// // func (a *ArticlesDb) NextArticle(article Article) (*Article, error) {
// // func (a *ArticlesDb) PrevArticle(article Article) (*Article, error) {
// // func (a *ArticlesDb) AllTagsInDB() ([]string, error) {
// // func (a *ArticlesDb) ArticlesByTag(tagName string) ([]Article, error) {
// // func (a *ArticlesDb) AllSeriesInDB() ([]string, error) {
// // func (a *ArticlesDb) ArticlesBySeries(series string) ([]Article, error) {
// // func (a *ArticlesDb) NextArticleInSeries(article Article) (Article, error) {
// // func (a *ArticlesDb) PrevArticleInSeries(article Article) (Article, error) {
// // func (a *ArticlesDb) Drafts() ([]Article, error) {
// // func (a *ArticlesDb) SpecialPages() ([]Article, error) {
// // func (a *ArticlesDb) SetCache(article Article, generatedHTML string) error {
// // func (a *ArticlesDb) GetCache(article Article) (string, error) {
// // func (a *ArticlesDb) Contains(DstFileName string) (bool, error) {
