struct NewArticleNeighbours {
  next_article: Option<NewArticle>,
  prev_article: Option<NewArticle>,
  next_series: Option<NewArticle>,
  prev_series: Option<NewArticle>,
}

pub fn get_neighbours(article_path: PathBuf) -> NewArticleNeighbours {
  NewArticleNeighbours {
      next_article: None,
      prev_article: None,
      next_series: None,
      prev_series: None,
  }
}