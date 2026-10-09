CREATE TABLE IF NOT EXISTS account(
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE,
  password VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS account_permission(
  account INT NOT NULL PRIMARY KEY,
  permission VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS evaluated_expression(
  id SERIAL PRIMARY KEY,
  account_id INT NOT NULL REFERENCES account(id),
  expression TEXT NOT NULL,
  result TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
