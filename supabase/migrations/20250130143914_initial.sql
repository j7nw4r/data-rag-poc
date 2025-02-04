CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE song_plays (
    id bigserial PRIMARY KEY,
    name VARCHAR NOT NULL,
    embedding VECTOR(1024) -- Adjust the dimension based on your embedding size
);

create table embeddings (
    id BIGSERIAL primary key,
    entity_id bigint not null ,
    embedding vector(1024) not null
);

create index on embeddings using ivfflat (embedding vector_ip_ops);