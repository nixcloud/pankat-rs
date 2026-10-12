-- Add 'open_graph_image' column to 'articles' table
ALTER TABLE articles ADD COLUMN open_graph_image TEXT;
-- force a re-parse of all articles, unchanged files would otherwise keep the old row
UPDATE articles SET hash = '';
