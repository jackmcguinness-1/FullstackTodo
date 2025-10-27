-- this table records workouts done by the user
CREATE TABLE workout_logs (
    id SERIAL PRIMARY KEY,
    workout_id INT NOT NULL REFERENCES workouts(id),
    start_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    end_time TIMESTAMPTZ,
    public BOOLEAN DEFAULT FALSE
);

CREATE INDEX idx_workout_logs_workout ON workout_logs(workout_id);