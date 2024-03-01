CREATE TABLE admins (
    admin_id SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    email TEXT NOT NULL,
    created_at FLOAT NOT NULL
);
