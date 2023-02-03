CREATE TABLE posts (
    id bigserial NOT NULL primary key,
    content text NOT NULL,
    likes integer NOT NULL DEFAULT 0,
    post_date timestamptz NOT NULL DEFAULT NOW()
);

CREATE TABLE replies (
    post_id bigint NOT NULL,
    id bigserial NOT NULL,
    likes integer NOT NULL DEFAULT 0,
    post_date timestamptz NOT NULL DEFAULT NOW(),
    PRIMARY KEY (post_id, id)
);