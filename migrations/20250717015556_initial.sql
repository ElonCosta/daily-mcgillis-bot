CREATE TABLE IF NOT EXISTS image_descriptors(
  descriptor_id INTEGER PRIMARY KEY AUTOINCREMENT,
  file_name TEXT NOT NULL,
  alt_text TEXT NOT NULL,
  width INTEGER NOT NULL,
  height INTEGER NOT NULL
);
