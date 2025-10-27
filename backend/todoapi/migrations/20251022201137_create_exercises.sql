CREATE TABLE exercises (
    id SERIAL PRIMARY KEY,
    created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    image_path VARCHAR(255),
    description VARCHAR(2096),
    public BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE INDEX idx_exercises_created_by ON exercises(created_by);