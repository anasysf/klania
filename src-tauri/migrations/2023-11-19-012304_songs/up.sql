CREATE TABLE IF NOT EXISTS songs (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  file_path TEXT NOT NULL,
  file_hash TEXT NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at DATETIME
);

CREATE UNIQUE INDEX idx_songs_file_path ON songs (file_path ASC);
CREATE UNIQUE INDEX idx_songs_file_hash ON songs (file_hash ASC);
CREATE INDEX idx_songs_deleted_at ON songs (deleted_at ASC);

CREATE TRIGGER tg_songs_updated_at
AFTER UPDATE
ON songs FOR EACH ROW
BEGIN
  UPDATE songs SET updated_at = CURRENT_TIMESTAMP 
    WHERE id = old.id;
END;
