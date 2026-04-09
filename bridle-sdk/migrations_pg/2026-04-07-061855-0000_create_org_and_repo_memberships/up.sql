CREATE TABLE org_memberships (
  id SERIAL PRIMARY KEY,
  org_id INTEGER NOT NULL REFERENCES organisations(id),
  user_id INTEGER NOT NULL REFERENCES users(id),
  role TEXT NOT NULL, -- 'owner', 'member', 'billing_manager'
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  UNIQUE(org_id, user_id)
);

CREATE TABLE repo_collaborators (
  id SERIAL PRIMARY KEY,
  repo_id INTEGER NOT NULL REFERENCES repositories(id),
  user_id INTEGER NOT NULL REFERENCES users(id),
  permission_level TEXT NOT NULL, -- 'read', 'triage', 'write', 'maintain', 'admin'
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  UNIQUE(repo_id, user_id)
);
