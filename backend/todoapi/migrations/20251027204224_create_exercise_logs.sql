CREATE TABLE exercise_logs (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    workout_log_id INT NOT NULL REFERENCES workout_logs(id) ON DELETE CASCADE,
    workout_exercise_id INT NOT NULL REFERENCES workout_exercises(id) ON DELETE CASCADE,
    num_sets INT,
    reps INT[],
    weights INT[],
    comment VARCHAR(2096)

    -- TODO: there should be some kind of constraint here to ensure that the workout_exercise_id has a matching workout_log_id
    -- but my brain is a bit too sleepy to do it now
);

CREATE INDEX idx_exercise_logs_user ON exercise_logs(user_id);
CREATE INDEX idx_exercise_logs_workout_log ON exercise_logs(workout_log_id);
CREATE INDEX idx_exercise_logs_workout_exercise ON exercise_logs(workout_exercise_id);