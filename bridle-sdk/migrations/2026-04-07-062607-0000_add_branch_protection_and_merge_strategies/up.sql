CREATE TABLE branch_protection_rules (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  branch_id INTEGER NOT NULL REFERENCES branches(id),
  required_pr_reviews INTEGER NOT NULL DEFAULT 0,
  require_code_owner_reviews BOOLEAN NOT NULL DEFAULT 0,
  required_status_checks TEXT, -- comma separated
  require_signed_commits BOOLEAN NOT NULL DEFAULT 0,
  enforce_admins BOOLEAN NOT NULL DEFAULT 0,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
  UNIQUE(branch_id)
);

ALTER TABLE repositories ADD COLUMN allow_merge_commit BOOLEAN NOT NULL DEFAULT 1;
ALTER TABLE repositories ADD COLUMN allow_squash_merge BOOLEAN NOT NULL DEFAULT 1;
ALTER TABLE repositories ADD COLUMN allow_rebase_merge BOOLEAN NOT NULL DEFAULT 1;
