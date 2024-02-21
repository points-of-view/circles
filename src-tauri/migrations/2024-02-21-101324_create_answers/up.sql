-- Your SQL goes here
CREATE TABLE answers (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  token_key TEXT NOT NULL,
  option_key TEXT NOT NULL,
  step_id INTEGER NOT NULL,
  FOREIGN KEY(step_id) REFERENCES steps(id)
);