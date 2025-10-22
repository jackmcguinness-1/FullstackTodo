CREATE TABLE routines (
    name VARCHAR(255),
    created_by INTEGER REFERENCES users(id),
    description TEXT,
    PRIMARY KEY (name, created_by)
)

CREATE INDEX idx_routines_created_by ON routines(created_by);

--TODO: maybe create an index to allow searching routines by equipments