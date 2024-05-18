\c graphvis

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR(255),
  password VARCHAR(255)
);

CREATE TABLE graphs (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255),
  content TEXT,
  published_by INT,
  published_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_graphs_users FOREIGN KEY (published_by) REFERENCES users (id)
);

