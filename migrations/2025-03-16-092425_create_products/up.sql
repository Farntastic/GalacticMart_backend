-- Your SQL goes here
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    details TEXT NOT NULL,
    price FLOAT NOT NULL,
    stock INTEGER NOT NULL,
    image TEXT NOT NULL,
    category TEXT NOT NULL
);
