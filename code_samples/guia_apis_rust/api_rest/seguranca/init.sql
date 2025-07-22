-- init.sql
CREATE TABLE itens (
  id SERIAL PRIMARY KEY,
  nome TEXT NOT NULL,
  quantidade INT NOT NULL
);

INSERT INTO itens (nome, quantidade) VALUES
  ('Maçã', 10),
  ('Banana', 20),
  ('Cereja', 30);
