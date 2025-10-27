-- this table specifies which exercises are in which routines and in what order
CREATE TABLE workout_exercises (
    id SERIAL PRIMARY KEY,
    exercise_id INT NOT NULL REFERENCES exercises(id) ON DELETE CASCADE,
    workout_id INT NOT NULL REFERENCES workouts(id) ON DELETE CASCADE,
    position INT NOT NULL,
    UNIQUE (workout_id, position)
);

CREATE INDEX idx_workout_exercises_exercise ON workout_exercises(exercise_id);
CREATE INDEX idx_workout_exercises_workout ON workout_exercises(workout_id);