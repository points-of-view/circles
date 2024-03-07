--- Delete all current sessions, since these were only test entries
DROP TABLE sessions;

CREATE TABLE IF NOT EXISTS sessions (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  created_at DATETIME DEFAULT current_timestamp NOT NULL,
  project_key TEXT NOT NULL,
  theme_key TEXT NOT NULL
);
