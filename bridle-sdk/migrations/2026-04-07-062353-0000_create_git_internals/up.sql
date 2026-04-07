CREATE TABLE commits (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  repo_id INTEGER NOT NULL REFERENCES repositories(id),
  sha TEXT NOT NULL,
  tree_sha TEXT NOT NULL,
  parent_shas TEXT NOT NULL, -- comma-separated
  message TEXT NOT NULL,
  author_name TEXT NOT NULL,
  author_email TEXT NOT NULL,
  author_date DATETIME NOT NULL,
  committer_name TEXT NOT NULL,
  committer_email TEXT NOT NULL,
  committer_date DATETIME NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
  UNIQUE(repo_id, sha)
);

CREATE TABLE trees (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  repo_id INTEGER NOT NULL REFERENCES repositories(id),
  sha TEXT NOT NULL,
  entries TEXT NOT NULL, -- JSON string
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
  UNIQUE(repo_id, sha)
);

CREATE TABLE blobs (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  repo_id INTEGER NOT NULL REFERENCES repositories(id),
  sha TEXT NOT NULL,
  size INTEGER NOT NULL,
  content BLOB,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
  UNIQUE(repo_id, sha)
);
