CREATE TABLE orders (
    order_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    products INT[] NOT NULL,
    total FLOAT NOT NULL,
    status TEXT NOT NULL,
    created_at FLOAT NOT NULL
);

