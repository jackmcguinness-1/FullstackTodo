CREATE TABLE exercise_muscles (
    id SERIAL PRIMARY KEY,
    exercise_id INT NOT NULL REFERENCES exercises(id),
    muscle_id INT NOT NULL REFERENCES muscles(id),
    usage INT NOT NULL CHECK (usage >= 1 AND usage <= 10)
);

CREATE INDEX idx_exercise_muscles_exercise on exercise_muscles(exercise_id);
CREATE INDEX idx_exercise_muscles_muscle on exercise_muscles(muscle_id);