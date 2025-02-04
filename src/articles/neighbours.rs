struct ArticleNeighbours {
  next_article: Option<Article>,
  prev_article: Option<Article>,
  next_series: Option<Article>,
  prev_series: Option<Article>,
}

pub fn get_neighbours(article_path: PathBuf) -> ArticleNeighbours {
  ArticleNeighbours {
      next_article: None,
      prev_article: None,
      next_series: None,
      prev_series: None,
  }
}