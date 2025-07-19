-- Add migration script here
ALTER TABLE image_descriptors ADD COLUMN weight REAL NOT NULL DEFAULT 1.0;
