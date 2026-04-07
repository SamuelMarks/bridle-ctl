ALTER TABLE repositories DROP COLUMN allow_merge_commit;
ALTER TABLE repositories DROP COLUMN allow_squash_merge;
ALTER TABLE repositories DROP COLUMN allow_rebase_merge;

DROP TABLE branch_protection_rules;
