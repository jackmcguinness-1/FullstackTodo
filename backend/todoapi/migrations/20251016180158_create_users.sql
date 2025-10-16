-- currently only google OAUTH2 is supported, i will add email support later if needed 
CREATE TYPE auth_provider as ENUM ('google');

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(100) NOT NULL,
    provider auth_provider NOT NULL,
    provider_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_users_email on users(email);
CREATE INDEX idx_users_provider ON users(provider);