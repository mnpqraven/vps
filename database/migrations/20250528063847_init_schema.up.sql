DROP TABLE IF EXISTS blog_tag CASCADE;

CREATE TABLE blog_tag (
    id VARCHAR(50) PRIMARY KEY,
    code VARCHAR(255) UNIQUE NOT NULL,
    label VARCHAR(255) NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

DROP TABLE IF EXISTS blog CASCADE;

CREATE TABLE blog (
    id VARCHAR(50) PRIMARY KEY,
    title VARCHAR(255),
    file_name VARCHAR(255) NOT NULL,
    file_key VARCHAR(255) NOT NULL,
    publish BOOLEAN,
    md_url VARCHAR(255) NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

DROP TABLE IF EXISTS media CASCADE;

CREATE TABLE media (
    temp_blog_id VARCHAR(50) PRIMARY KEY,
    file_name VARCHAR(255) NOT NULL,
    media_url VARCHAR(255) NOT NULL,
    blog_id VARCHAR(50),

    CONSTRAINT fk_blog FOREIGN KEY (blog_id)
        REFERENCES blog(id)
);

DROP TABLE IF EXISTS blog_tag_map CASCADE;

CREATE TABLE blog_tag_map (
    blog_id VARCHAR(50),
    CONSTRAINT fk_blog FOREIGN KEY (blog_id)
        REFERENCES blog(id),

    tag_id VARCHAR(50),
    CONSTRAINT fk_tag FOREIGN KEY (tag_id)
        REFERENCES blog_tag(id)
);
