CREATE TABLE exercise_equipment (
    id SERIAL PRIMARY KEY,
    exercise_id INT NOT NULL REFERENCES exercises(id),
    equipment_id INT NOT NULL REFERENCES equipments(id)
);

CREATE INDEX idx_exercise_equipment_exercise on exercise_equipment(exercise_id);
CREATE INDEX idx_exercise_equipment_equipment on exercise_equipment(equipment_id);