CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    email TEXT NOT NULL,
    created_at FLOAT NOT NULL,
    products INT[] NOT NULL,
    orders INT[] NOT NULL
);

