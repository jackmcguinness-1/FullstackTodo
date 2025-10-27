CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(32) NOT NULL,
    auth_provider_id INT NOT NULL REFERENCES auth_providers(id), 
    auth_provider_user_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_users_email on users(email);
CREATE INDEX idx_users_auth_provider ON users(auth_provider_id);