-- Your SQL goes here
CREATE TABLE products (
  id INTEGER AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  stock FLOAT NOT NULL,
  price INTEGER
);