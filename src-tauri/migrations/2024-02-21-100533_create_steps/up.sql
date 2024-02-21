CREATE TABLE steps (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  created_at DATETIME DEFAULT current_timestamp NOT NULL,
  question_key TEXT NOT NULL,
  session_id INTEGER NOT NULL,
  FOREIGN KEY(session_id) REFERENCES sessions(id)
);
