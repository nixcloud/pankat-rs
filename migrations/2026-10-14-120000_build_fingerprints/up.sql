-- fingerprint of the inputs each generated page was written from, see src/fingerprint.rs
CREATE TABLE outputs (
  dst_file_name TEXT NOT NULL PRIMARY KEY,
  fingerprint TEXT NOT NULL
);
-- the global inputs of the last run, to tell in the log what changed
CREATE TABLE build_state (
  key TEXT NOT NULL PRIMARY KEY,
  value TEXT NOT NULL
);
-- article hashes now include the generator and pandoc fingerprints
UPDATE articles SET hash = '';
