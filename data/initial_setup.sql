INSERT INTO
    roles (name)
VALUES
    ('Admin'),
    ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
    users (name, email, password_hash, role_id)
SELECT
    'Rusty Book Manager',
    'rusty.book.manager@example.com',
    '$2b$12$m4W3nzRckFI0i9A8tXZn3ukCQYICAtlVmhoib2M8kEax.AK8hPEJ2', -- Rust7878
    role_id
FROM
    roles
WHERE
    name LIKE 'Admin';
