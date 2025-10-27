CREATE TABLE owned_equipments (
    id SERIAL PRIMARY KEY,
    owner_id INT NOT NULL REFERENCES users(id),
    equipment_id INT NOT NULL REFERENCES equipments(id)
);

CREATE INDEX idx_owned_equipments_owner ON owned_equipments(owner_id);