-- This file should undo anything in `up.sql`

DROP TABLE posts;
DROP FUNCTION IF EXISTS reset_posts_id_sequence();
