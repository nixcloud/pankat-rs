CREATE TABLE articles (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
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
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name TEXT NOT NULL UNIQUE
);

CREATE TABLE article_tags (
  article_id INTEGER NOT NULL,
  tag_id INTEGER NOT NULL,
  PRIMARY KEY (article_id, tag_id),
  FOREIGN KEY (article_id) REFERENCES articles(id) ON DELETE CASCADE,
  FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

CREATE TABLE cache (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  src_file_name TEXT NOT NULL UNIQUE,
  hash TEXT,
  html TEXT
);