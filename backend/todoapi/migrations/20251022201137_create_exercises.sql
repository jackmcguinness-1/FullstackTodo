CREATE TYPE muscle_group AS ENUM("chest", "bicep", "legs");

CREATE TYPE equipment AS ENU("barbell", "dumbbell", "ezbar", "pull up bar");

CREATE TYPE muscle_usage AS (
    group muscle_group,
    usage INTEGER CHECK (usage >= 1 AND usage <= 10)
)

CREATE TABLE exercises (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    image_path VARCHAR(255),
    description TEXT,
    muscle_usages muscle_usage[],
    equipments equipment[],
    -- created_by is so that users can add custom exercises if they choose
    -- TODO: we should use this to limit the amount of custom exercises a user can make
    created_by INTEGER REFERENCES users(id) ON DELETE CASCADE,
);

-- this index allows searching exercises by user
CREATE INDEX idx_exercises_created_by ON exercises(created_by);

-- this is an index to allow searching exercises by the muscle group
CREATE INDEX idx_exercises_muscle_groups ON exercises
USING GIN ((
    SELECT array_agg((m).group)
    FROM unnest(muscle_usages) AS m
));

-- this index allows to search exercises by equipments needed
CREATE INDEX idx_exercises_equipments ON exercises USING GIN (equipments);