-- Your SQL goes here

CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE OR REPLACE FUNCTION reset_posts_id_sequence() RETURNS VOID AS $$
DECLARE max_id INT;
BEGIN
    SELECT COALESCE(MAX(id), 0) FROM posts INTO max_id;
    EXECUTE FORMAT('ALTER SEQUENCE posts_id_seq RESTART WITH %s;', max_id + 1);
END;
$$ LANGUAGE plpgsql;
