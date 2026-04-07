#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]

// @generated automatically by Diesel CLI.

diesel::table! {
    blobs (id) {
        id -> Integer,
        repo_id -> Integer,
        sha -> Text,
        size -> Integer,
        content -> Nullable<Binary>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    branch_protection_rules (id) {
        id -> Integer,
        branch_id -> Integer,
        required_pr_reviews -> Integer,
        require_code_owner_reviews -> Bool,
        required_status_checks -> Nullable<Text>,
        require_signed_commits -> Bool,
        enforce_admins -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    branches (id) {
        id -> Integer,
        repo_id -> Integer,
        name -> Text,
        head_sha -> Text,
        is_protected -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    commits (id) {
        id -> Integer,
        repo_id -> Integer,
        sha -> Text,
        tree_sha -> Text,
        parent_shas -> Text,
        message -> Text,
        author_name -> Text,
        author_email -> Text,
        author_date -> Timestamp,
        committer_name -> Text,
        committer_email -> Text,
        committer_date -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    follows (id) {
        id -> Integer,
        follower_id -> Integer,
        following_id -> Integer,
        created_at -> Timestamp,
    }
}

diesel::table! {
    issue_labels (id) {
        id -> Integer,
        issue_id -> Integer,
        label_id -> Integer,
    }
}

diesel::table! {
    issues (id) {
        id -> Integer,
        repo_id -> Integer,
        number -> Integer,
        title -> Text,
        body -> Nullable<Text>,
        state -> Text,
        author_id -> Integer,
        assignee_id -> Nullable<Integer>,
        milestone_id -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    keys (id) {
        id -> Integer,
        user_id -> Integer,
        key_type -> Text,
        title -> Text,
        key_data -> Text,
        fingerprint -> Text,
        last_used_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    labels (id) {
        id -> Integer,
        repo_id -> Integer,
        name -> Text,
        color -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    milestones (id) {
        id -> Integer,
        repo_id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        state -> Text,
        due_on -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    org_memberships (id) {
        id -> Integer,
        org_id -> Integer,
        user_id -> Integer,
        role -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    organisations (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        verified_domain -> Nullable<Text>,
        billing_plan -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    pull_request_reviews (id) {
        id -> Integer,
        pr_id -> Integer,
        user_id -> Integer,
        state -> Text,
        body -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    pull_requests (id) {
        id -> Integer,
        repo_id -> Integer,
        number -> Integer,
        title -> Text,
        body -> Nullable<Text>,
        state -> Text,
        head_branch -> Text,
        base_branch -> Text,
        author_id -> Integer,
        assignee_id -> Nullable<Integer>,
        milestone_id -> Nullable<Integer>,
        is_draft -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    releases (id) {
        id -> Integer,
        repo_id -> Integer,
        tag_name -> Text,
        target_commitish -> Text,
        name -> Nullable<Text>,
        body -> Nullable<Text>,
        is_draft -> Bool,
        is_prerelease -> Bool,
        author_id -> Integer,
        created_at -> Timestamp,
        published_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    repo_collaborators (id) {
        id -> Integer,
        repo_id -> Integer,
        user_id -> Integer,
        permission_level -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    repositories (id) {
        id -> Integer,
        owner_id -> Integer,
        owner_type -> Text,
        name -> Text,
        description -> Nullable<Text>,
        is_private -> Bool,
        is_fork -> Bool,
        archived -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        allow_merge_commit -> Bool,
        allow_squash_merge -> Bool,
        allow_rebase_merge -> Bool,
    }
}

diesel::table! {
    stars (id) {
        id -> Integer,
        user_id -> Integer,
        repo_id -> Integer,
        created_at -> Timestamp,
    }
}

diesel::table! {
    teams (id) {
        id -> Integer,
        org_id -> Integer,
        parent_id -> Nullable<Integer>,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    trees (id) {
        id -> Integer,
        repo_id -> Integer,
        sha -> Text,
        entries -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
        password_hash -> Text,
        avatar_url -> Nullable<Text>,
        bio -> Nullable<Text>,
        status -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    webhooks (id) {
        id -> Integer,
        repo_id -> Integer,
        url -> Text,
        content_type -> Text,
        secret -> Nullable<Text>,
        events -> Text,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(blobs -> repositories (repo_id));
diesel::joinable!(branch_protection_rules -> branches (branch_id));
diesel::joinable!(branches -> repositories (repo_id));
diesel::joinable!(commits -> repositories (repo_id));
diesel::joinable!(issue_labels -> issues (issue_id));
diesel::joinable!(issue_labels -> labels (label_id));
diesel::joinable!(issues -> milestones (milestone_id));
diesel::joinable!(issues -> repositories (repo_id));
diesel::joinable!(keys -> users (user_id));
diesel::joinable!(labels -> repositories (repo_id));
diesel::joinable!(milestones -> repositories (repo_id));
diesel::joinable!(org_memberships -> organisations (org_id));
diesel::joinable!(org_memberships -> users (user_id));
diesel::joinable!(pull_request_reviews -> pull_requests (pr_id));
diesel::joinable!(pull_request_reviews -> users (user_id));
diesel::joinable!(pull_requests -> milestones (milestone_id));
diesel::joinable!(pull_requests -> repositories (repo_id));
diesel::joinable!(releases -> repositories (repo_id));
diesel::joinable!(releases -> users (author_id));
diesel::joinable!(repo_collaborators -> repositories (repo_id));
diesel::joinable!(repo_collaborators -> users (user_id));
diesel::joinable!(stars -> repositories (repo_id));
diesel::joinable!(stars -> users (user_id));
diesel::joinable!(teams -> organisations (org_id));
diesel::joinable!(trees -> repositories (repo_id));
diesel::joinable!(webhooks -> repositories (repo_id));

diesel::allow_tables_to_appear_in_same_query!(
    blobs,
    branch_protection_rules,
    branches,
    commits,
    follows,
    issue_labels,
    issues,
    keys,
    labels,
    milestones,
    org_memberships,
    organisations,
    pull_request_reviews,
    pull_requests,
    releases,
    repo_collaborators,
    repositories,
    stars,
    teams,
    trees,
    users,
    webhooks,
);
