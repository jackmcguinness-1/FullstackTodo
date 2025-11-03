INSERT INTO auth_providers (id, name) VALUES
    (0, 'dev'),
    (1, 'email'),
    (2, 'google')
ON CONFLICT (id) DO NOTHING;