ALTER TABLE trusted RENAME COLUMN at TO created_at;

ALTER TABLE trusted ALTER COLUMN created_at SET NOT NULL;
ALTER TABLE trusted ALTER COLUMN created_at SET DEFAULT now();
