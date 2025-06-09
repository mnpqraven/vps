DROP TABLE IF EXISTS blog_tag CASCADE;

CREATE TABLE blog_tag (
    id VARCHAR(50) PRIMARY KEY,
    code VARCHAR(255) UNIQUE NOT NULL,
    label VARCHAR(255) NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

DROP TABLE IF EXISTS blog_meta CASCADE;

CREATE TABLE blog_meta (
    id VARCHAR(50) PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    file_name VARCHAR(255) NOT NULL,
    is_publish BOOLEAN NOT NULL DEFAULT false,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

DROP TABLE IF EXISTS blog_meta_tag_map CASCADE;

CREATE TABLE blog_meta_tag_map (
    blog_meta_id VARCHAR(50),
    blog_tag_id VARCHAR(50),

    CONSTRAINT fk_blog FOREIGN KEY (blog_meta_id) REFERENCES blog_meta(id),
    CONSTRAINT fk_tag FOREIGN KEY (blog_tag_id) REFERENCES blog_tag(id)
);
