CREATE TABLE articles (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  src_file_name TEXT NOT NULL UNIQUE,
  dst_file_name TEXT NOT NULL UNIQUE,
  title TEXT,
  modification_date TIMESTAMP,
  summary TEXT,
  series TEXT,
  draft BOOLEAN,
  special_page BOOLEAN,
  timeline BOOLEAN,
  anchorjs BOOLEAN,
  tocify BOOLEAN,
  live_updates BOOLEAN
);

CREATE TABLE tags (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL UNIQUE,
  article_id INTEGER NOT NULL REFERENCES articles(id)
);

CREATE TABLE series (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL UNIQUE,
  article_id INTEGER NOT NULL REFERENCES articles(id)
);

CREATE TABLE articles_cache (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  hash TEXT NOT NULL UNIQUE,
  html TEXT NOT NULL UNIQUE
);

-- CREATE TABLE article_tags (
--   article_id INTEGER NOT NULL,
--   tag_id INTEGER NOT NULL,
--   FOREIGN KEY(article_id) REFERENCES articles(id) ON DELETE CASCADE,
--   FOREIGN KEY(tag_id) REFERENCES tags(id) ON DELETE CASCADE
-- );

-- CREATE INDEX idx_article_series ON articles(series);
-- CREATE INDEX idx_article_modification_date ON articles(modification_date);