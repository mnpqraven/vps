CREATE TABLE oauth_user (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at BIGINT NOT NULL,
    last_updated BIGINT NOT NULL
);

CREATE TABLE session (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL UNIQUE,
    session_id VARCHAR NOT NULL,
    expires_at BIGINT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES oauth_user(id)
);
