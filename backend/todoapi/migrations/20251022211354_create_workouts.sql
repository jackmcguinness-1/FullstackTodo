-- this table keeps track of 
CREATE TABLE workouts (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    created_by INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    public BOOLEAN DEFAULT FALSE,
    UNIQUE (name, created_by)
);

CREATE INDEX idx_workouts_created_by ON workouts(created_by);