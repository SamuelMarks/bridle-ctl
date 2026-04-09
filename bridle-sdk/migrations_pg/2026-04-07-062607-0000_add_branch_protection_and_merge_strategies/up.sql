CREATE TABLE branch_protection_rules (
  id SERIAL PRIMARY KEY,
  branch_id INTEGER NOT NULL REFERENCES branches(id),
  required_pr_reviews INTEGER NOT NULL DEFAULT FALSE,
  require_code_owner_reviews BOOLEAN NOT NULL DEFAULT FALSE,
  required_status_checks TEXT, -- comma separated
  require_signed_commits BOOLEAN NOT NULL DEFAULT FALSE,
  enforce_admins BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  UNIQUE(branch_id)
);

ALTER TABLE repositories ADD COLUMN allow_merge_commit BOOLEAN NOT NULL DEFAULT TRUE;
ALTER TABLE repositories ADD COLUMN allow_squash_merge BOOLEAN NOT NULL DEFAULT TRUE;
ALTER TABLE repositories ADD COLUMN allow_rebase_merge BOOLEAN NOT NULL DEFAULT TRUE;
