CREATE TABLE pull_requests (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  repo_id INTEGER NOT NULL REFERENCES repositories(id),
  number INTEGER NOT NULL,
  title TEXT NOT NULL,
  body TEXT,
  state TEXT NOT NULL DEFAULT 'open',
  head_branch TEXT NOT NULL,
  base_branch TEXT NOT NULL,
  author_id INTEGER NOT NULL REFERENCES users(id),
  assignee_id INTEGER REFERENCES users(id),
  milestone_id INTEGER REFERENCES milestones(id),
  is_draft BOOLEAN NOT NULL DEFAULT 0,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
  UNIQUE(repo_id, number)
);

CREATE TABLE pull_request_reviews (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  pr_id INTEGER NOT NULL REFERENCES pull_requests(id),
  user_id INTEGER NOT NULL REFERENCES users(id),
  state TEXT NOT NULL, -- 'approved', 'changes_requested', 'commented'
  body TEXT,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE releases (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  repo_id INTEGER NOT NULL REFERENCES repositories(id),
  tag_name TEXT NOT NULL,
  target_commitish TEXT NOT NULL,
  name TEXT,
  body TEXT,
  is_draft BOOLEAN NOT NULL DEFAULT 0,
  is_prerelease BOOLEAN NOT NULL DEFAULT 0,
  author_id INTEGER NOT NULL REFERENCES users(id),
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
  published_at DATETIME
);

CREATE TABLE webhooks (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  repo_id INTEGER NOT NULL REFERENCES repositories(id),
  url TEXT NOT NULL,
  content_type TEXT NOT NULL,
  secret TEXT,
  events TEXT NOT NULL, -- comma-separated
  is_active BOOLEAN NOT NULL DEFAULT 1,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);
