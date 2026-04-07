//! Database models mapped to the diesel schema.

use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// User profile and authentication model.
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    /// Primary key.
    pub id: i32,
    /// Unique username.
    pub username: String,
    /// Unique email address.
    pub email: String,
    /// Hashed password.
    pub password_hash: String,
    /// Avatar URL.
    pub avatar_url: Option<String>,
    /// User biography.
    pub bio: Option<String>,
    /// User status message.
    pub status: Option<String>,
    /// Creation timestamp.
    pub created_at: NaiveDateTime,
    /// Last update timestamp.
    pub updated_at: NaiveDateTime,
}

/// Organisation model.
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = organisations)]
pub struct Organisation {
    /// Primary key.
    pub id: i32,
    /// Organisation name.
    pub name: String,
    /// Organisation description.
    pub description: Option<String>,
    /// Verified domain.
    pub verified_domain: Option<String>,
    /// Billing plan (e.g., "free", "pro").
    pub billing_plan: String,
    /// Creation timestamp.
    pub created_at: NaiveDateTime,
    /// Last update timestamp.
    pub updated_at: NaiveDateTime,
}

/// Team model within an organisation.
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = teams)]
pub struct Team {
    /// Primary key.
    pub id: i32,
    /// Parent organisation ID.
    pub org_id: i32,
    /// Optional parent team ID.
    pub parent_id: Option<i32>,
    /// Team name.
    pub name: String,
    /// Team description.
    pub description: Option<String>,
    /// Creation timestamp.
    pub created_at: NaiveDateTime,
    /// Last update timestamp.
    pub updated_at: NaiveDateTime,
}

/// Repository model.
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = repositories)]
pub struct Repository {
    /// Primary key.
    pub id: i32,
    /// Owner ID (User or Organisation).
    pub owner_id: i32,
    /// Type of owner ("user" or "org").
    pub owner_type: String,
    /// Repository name.
    pub name: String,
    /// Repository description.
    pub description: Option<String>,
    /// Private flag.
    pub is_private: bool,
    /// Fork flag.
    pub is_fork: bool,
    /// Archived flag.
    pub archived: bool,
    /// Creation timestamp.
    pub created_at: NaiveDateTime,
    /// Last update timestamp.
    pub updated_at: NaiveDateTime,
    /// Allow merge commit flag.
    pub allow_merge_commit: bool,
    /// Allow squash merge flag.
    pub allow_squash_merge: bool,
    /// Allow rebase merge flag.
    pub allow_rebase_merge: bool,
}

/// Branch model within a repository.
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = branches)]
pub struct Branch {
    /// Primary key.
    pub id: i32,
    /// Parent repository ID.
    pub repo_id: i32,
    /// Branch name.
    pub name: String,
    /// HEAD commit SHA.
    pub head_sha: String,
    /// Protection flag.
    pub is_protected: bool,
    /// Creation timestamp.
    pub created_at: NaiveDateTime,
    /// Last update timestamp.
    pub updated_at: NaiveDateTime,
}

/// Branch protection rule model.
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = branch_protection_rules)]
pub struct BranchProtectionRule {
    /// Primary key.
    pub id: i32,
    /// Branch ID.
    pub branch_id: i32,
    /// Required PR reviews count.
    pub required_pr_reviews: i32,
    /// Require code owner reviews flag.
    pub require_code_owner_reviews: bool,
    /// Required status checks (comma-separated).
    pub required_status_checks: Option<String>,
    /// Require signed commits flag.
    pub require_signed_commits: bool,
    /// Enforce admins flag.
    pub enforce_admins: bool,
    /// Creation timestamp.
    pub created_at: NaiveDateTime,
    /// Last update timestamp.
    pub updated_at: NaiveDateTime,
}

/// Key model for SSH and GPG keys.
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = keys)]
pub struct Key {
    /// Primary key.
    pub id: i32,
    /// User ID this key belongs to.
    pub user_id: i32,
    /// Type of the key ("ssh" or "gpg").
    pub key_type: String,
    /// Title given to the key by the user.
    pub title: String,
    /// Public key data.
    pub key_data: String,
    /// Fingerprint of the key.
    pub fingerprint: String,
    /// Last used timestamp.
    pub last_used_at: Option<NaiveDateTime>,
    /// Creation timestamp.
    pub created_at: NaiveDateTime,
    /// Last update timestamp.
    pub updated_at: NaiveDateTime,
}

/// Follow relationship model.
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = follows)]
pub struct Follow {
    /// Primary key.
    pub id: i32,
    /// Follower User ID.
    pub follower_id: i32,
    /// Following User ID.
    pub following_id: i32,
    /// Creation timestamp.
    pub created_at: NaiveDateTime,
}

/// Star relationship model.
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = stars)]
pub struct Star {
    /// Primary key.
    pub id: i32,
    /// User ID.
    pub user_id: i32,
    /// Repository ID.
    pub repo_id: i32,
    /// Creation timestamp.
    pub created_at: NaiveDateTime,
}

/// Organisation membership model.
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = org_memberships)]
pub struct OrgMembership {
    /// Primary key.
    pub id: i32,
    /// Organisation ID.
    pub org_id: i32,
    /// User ID.
    pub user_id: i32,
    /// Role (e.g., "owner", "member").
    pub role: String,
    /// Creation timestamp.
    pub created_at: NaiveDateTime,
    /// Last update timestamp.
    pub updated_at: NaiveDateTime,
}

/// Repository collaborator model.
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = repo_collaborators)]
pub struct RepoCollaborator {
    /// Primary key.
    pub id: i32,
    /// Repository ID.
    pub repo_id: i32,
    /// User ID.
    pub user_id: i32,
    /// Permission level (e.g., "read", "write").
    pub permission_level: String,
    /// Creation timestamp.
    pub created_at: NaiveDateTime,
    /// Last update timestamp.
    pub updated_at: NaiveDateTime,
}

/// Milestone model.
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = milestones)]
pub struct Milestone {
    /// Primary key.
    pub id: i32,
    /// Repository ID.
    pub repo_id: i32,
    /// Title.
    pub title: String,
    /// Description.
    pub description: Option<String>,
    /// State (open/closed).
    pub state: String,
    /// Due date.
    pub due_on: Option<NaiveDateTime>,
    /// Creation timestamp.
    pub created_at: NaiveDateTime,
    /// Last update timestamp.
    pub updated_at: NaiveDateTime,
}

/// Label model.
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = labels)]
pub struct Label {
    /// Primary key.
    pub id: i32,
    /// Repository ID.
    pub repo_id: i32,
    /// Name.
    pub name: String,
    /// Color hex.
    pub color: String,
    /// Description.
    pub description: Option<String>,
    /// Creation timestamp.
    pub created_at: NaiveDateTime,
    /// Last update timestamp.
    pub updated_at: NaiveDateTime,
}

/// Issue model.
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = issues)]
pub struct Issue {
    /// Primary key.
    pub id: i32,
    /// Repository ID.
    pub repo_id: i32,
    /// Issue number.
    pub number: i32,
    /// Title.
    pub title: String,
    /// Body.
    pub body: Option<String>,
    /// State.
    pub state: String,
    /// Author user ID.
    pub author_id: i32,
    /// Assignee user ID.
    pub assignee_id: Option<i32>,
    /// Milestone ID.
    pub milestone_id: Option<i32>,
    /// Creation timestamp.
    pub created_at: NaiveDateTime,
    /// Last update timestamp.
    pub updated_at: NaiveDateTime,
}

/// Issue label relationship.
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = issue_labels)]
pub struct IssueLabel {
    /// Primary key.
    pub id: i32,
    /// Issue ID.
    pub issue_id: i32,
    /// Label ID.
    pub label_id: i32,
}

/// Pull request model.
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = pull_requests)]
pub struct PullRequest {
    /// Primary key.
    pub id: i32,
    /// Repository ID.
    pub repo_id: i32,
    /// Pull request number.
    pub number: i32,
    /// Title.
    pub title: String,
    /// Body.
    pub body: Option<String>,
    /// State (open, closed).
    pub state: String,
    /// Head branch.
    pub head_branch: String,
    /// Base branch.
    pub base_branch: String,
    /// Author user ID.
    pub author_id: i32,
    /// Assignee user ID.
    pub assignee_id: Option<i32>,
    /// Milestone ID.
    pub milestone_id: Option<i32>,
    /// Is draft.
    pub is_draft: bool,
    /// Creation timestamp.
    pub created_at: NaiveDateTime,
    /// Last update timestamp.
    pub updated_at: NaiveDateTime,
}

/// Pull request review model.
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = pull_request_reviews)]
pub struct PullRequestReview {
    /// Primary key.
    pub id: i32,
    /// Pull request ID.
    pub pr_id: i32,
    /// User ID.
    pub user_id: i32,
    /// Review state.
    pub state: String,
    /// Body.
    pub body: Option<String>,
    /// Creation timestamp.
    pub created_at: NaiveDateTime,
    /// Last update timestamp.
    pub updated_at: NaiveDateTime,
}

/// Release model.
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = releases)]
pub struct Release {
    /// Primary key.
    pub id: i32,
    /// Repository ID.
    pub repo_id: i32,
    /// Tag name.
    pub tag_name: String,
    /// Target commitish.
    pub target_commitish: String,
    /// Release name.
    pub name: Option<String>,
    /// Body.
    pub body: Option<String>,
    /// Is draft.
    pub is_draft: bool,
    /// Is prerelease.
    pub is_prerelease: bool,
    /// Author user ID.
    pub author_id: i32,
    /// Creation timestamp.
    pub created_at: NaiveDateTime,
    /// Publish timestamp.
    pub published_at: Option<NaiveDateTime>,
}

/// Webhook model.
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = webhooks)]
pub struct Webhook {
    /// Primary key.
    pub id: i32,
    /// Repository ID.
    pub repo_id: i32,
    /// URL.
    pub url: String,
    /// Content type.
    pub content_type: String,
    /// Secret.
    pub secret: Option<String>,
    /// Subscribed events.
    pub events: String,
    /// Is active.
    pub is_active: bool,
    /// Creation timestamp.
    pub created_at: NaiveDateTime,
    /// Last update timestamp.
    pub updated_at: NaiveDateTime,
}

/// Commit model.
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = commits)]
pub struct Commit {
    /// Primary key.
    pub id: i32,
    /// Repository ID.
    pub repo_id: i32,
    /// Commit SHA.
    pub sha: String,
    /// Tree SHA.
    pub tree_sha: String,
    /// Parent SHAs.
    pub parent_shas: String,
    /// Commit message.
    pub message: String,
    /// Author name.
    pub author_name: String,
    /// Author email.
    pub author_email: String,
    /// Author date.
    pub author_date: NaiveDateTime,
    /// Committer name.
    pub committer_name: String,
    /// Committer email.
    pub committer_email: String,
    /// Committer date.
    pub committer_date: NaiveDateTime,
    /// Creation timestamp.
    pub created_at: NaiveDateTime,
}

/// Tree model.
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = trees)]
pub struct Tree {
    /// Primary key.
    pub id: i32,
    /// Repository ID.
    pub repo_id: i32,
    /// Tree SHA.
    pub sha: String,
    /// Tree entries.
    pub entries: String,
    /// Creation timestamp.
    pub created_at: NaiveDateTime,
}

/// Blob model.
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = blobs)]
pub struct Blob {
    /// Primary key.
    pub id: i32,
    /// Repository ID.
    pub repo_id: i32,
    /// Blob SHA.
    pub sha: String,
    /// Size in bytes.
    pub size: i32,
    /// Binary content.
    pub content: Option<Vec<u8>>,
    /// Creation timestamp.
    pub created_at: NaiveDateTime,
}

use std::collections::HashMap;

/// Request payload for running tools via API or CLI.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolRunRequest {
    /// Target a specific regex file pattern.
    pub pattern: Option<String>,
    /// List of specific tools to run.
    pub tools: Option<Vec<String>>,
    /// Additional arguments mapped to specific tools (e.g., {"tool1": ["--flag1", "val"]}).
    pub tool_args: Option<HashMap<String, Vec<String>>>,
    /// Perform a dry-run without making changes (for fix actions).
    pub dry_run: Option<bool>,
    /// The action to perform ("audit" or "fix"). Defaults to "fix".
    pub action: Option<String>,
}

#[cfg(test)]
mod tool_tests {
    use super::*;

    #[test]
    fn test_tool_run_request() {
        let req = ToolRunRequest {
            pattern: Some(r".*\.rs$".into()),
            tools: Some(vec!["clippy".into()]),
            tool_args: None,
            dry_run: Some(true),
            action: None,
        };
        assert_eq!(req.pattern.as_deref(), Some(r".*\.rs$"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let now = chrono::Utc::now().naive_utc();
        let user = User {
            id: 1,
            username: "testuser".into(),
            email: "test@example.com".into(),
            password_hash: "hash".into(),
            avatar_url: None,
            bio: None,
            status: None,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(user.username, "testuser");
    }

    #[test]
    fn test_org_creation() {
        let now = chrono::Utc::now().naive_utc();
        let org = Organisation {
            id: 1,
            name: "testorg".into(),
            description: None,
            verified_domain: None,
            billing_plan: "free".into(),
            created_at: now,
            updated_at: now,
        };
        assert_eq!(org.name, "testorg");
    }

    #[test]
    fn test_team_creation() {
        let now = chrono::Utc::now().naive_utc();
        let team = Team {
            id: 1,
            org_id: 1,
            parent_id: None,
            name: "devs".into(),
            description: None,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(team.name, "devs");
    }

    #[test]
    fn test_repo_creation() {
        let now = chrono::Utc::now().naive_utc();
        let repo = Repository {
            id: 1,
            owner_id: 1,
            owner_type: "user".into(),
            name: "repo".into(),
            description: None,
            is_private: false,
            is_fork: false,
            archived: false,
            allow_merge_commit: true,
            allow_squash_merge: true,
            allow_rebase_merge: true,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(repo.name, "repo");
    }

    #[test]
    fn test_branch_creation() {
        let now = chrono::Utc::now().naive_utc();
        let branch = Branch {
            id: 1,
            repo_id: 1,
            name: "main".into(),
            head_sha: "abcdef".into(),
            is_protected: true,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(branch.name, "main");
    }

    #[test]
    fn test_branch_protection_rule_creation() {
        let now = chrono::Utc::now().naive_utc();
        let rule = BranchProtectionRule {
            id: 1,
            branch_id: 1,
            required_pr_reviews: 1,
            require_code_owner_reviews: true,
            required_status_checks: None,
            require_signed_commits: true,
            enforce_admins: true,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(rule.required_pr_reviews, 1);
    }

    #[test]
    fn test_key_creation() {
        let now = chrono::Utc::now().naive_utc();
        let key = Key {
            id: 1,
            user_id: 1,
            key_type: "ssh".into(),
            title: "my key".into(),
            key_data: "ssh-rsa AAAAB3Nza...".into(),
            fingerprint: "SHA256:abcd...".into(),
            last_used_at: None,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(key.title, "my key");
        assert_eq!(key.key_type, "ssh");
    }

    #[test]
    fn test_follow_creation() {
        let now = chrono::Utc::now().naive_utc();
        let follow = Follow {
            id: 1,
            follower_id: 1,
            following_id: 2,
            created_at: now,
        };
        assert_eq!(follow.follower_id, 1);
        assert_eq!(follow.following_id, 2);
    }

    #[test]
    fn test_star_creation() {
        let now = chrono::Utc::now().naive_utc();
        let star = Star {
            id: 1,
            user_id: 1,
            repo_id: 2,
            created_at: now,
        };
        assert_eq!(star.user_id, 1);
        assert_eq!(star.repo_id, 2);
    }

    #[test]
    fn test_org_membership_creation() {
        let now = chrono::Utc::now().naive_utc();
        let membership = OrgMembership {
            id: 1,
            org_id: 1,
            user_id: 2,
            role: "owner".into(),
            created_at: now,
            updated_at: now,
        };
        assert_eq!(membership.role, "owner");
    }

    #[test]
    fn test_repo_collaborator_creation() {
        let now = chrono::Utc::now().naive_utc();
        let collab = RepoCollaborator {
            id: 1,
            repo_id: 1,
            user_id: 2,
            permission_level: "write".into(),
            created_at: now,
            updated_at: now,
        };
        assert_eq!(collab.permission_level, "write");
    }

    #[test]
    fn test_issue_creation() {
        let now = chrono::Utc::now().naive_utc();
        let issue = Issue {
            id: 1,
            repo_id: 1,
            number: 1,
            title: "bug".into(),
            body: None,
            state: "open".into(),
            author_id: 1,
            assignee_id: None,
            milestone_id: None,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(issue.title, "bug");
    }

    #[test]
    fn test_milestone_creation() {
        let now = chrono::Utc::now().naive_utc();
        let milestone = Milestone {
            id: 1,
            repo_id: 1,
            title: "v1".into(),
            description: None,
            state: "open".into(),
            due_on: None,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(milestone.title, "v1");
    }

    #[test]
    fn test_label_creation() {
        let now = chrono::Utc::now().naive_utc();
        let label = Label {
            id: 1,
            repo_id: 1,
            name: "bug".into(),
            color: "ff0000".into(),
            description: None,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(label.name, "bug");
    }

    #[test]
    fn test_issue_label_creation() {
        let issue_label = IssueLabel {
            id: 1,
            issue_id: 1,
            label_id: 2,
        };
        assert_eq!(issue_label.issue_id, 1);
    }

    #[test]
    fn test_pull_request_creation() {
        let now = chrono::Utc::now().naive_utc();
        let pr = PullRequest {
            id: 1,
            repo_id: 1,
            number: 1,
            title: "fix".into(),
            body: None,
            state: "open".into(),
            head_branch: "feat".into(),
            base_branch: "main".into(),
            author_id: 1,
            assignee_id: None,
            milestone_id: None,
            is_draft: false,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(pr.title, "fix");
    }

    #[test]
    fn test_pr_review_creation() {
        let now = chrono::Utc::now().naive_utc();
        let review = PullRequestReview {
            id: 1,
            pr_id: 1,
            user_id: 2,
            state: "approved".into(),
            body: None,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(review.state, "approved");
    }

    #[test]
    fn test_release_creation() {
        let now = chrono::Utc::now().naive_utc();
        let release = Release {
            id: 1,
            repo_id: 1,
            tag_name: "v1.0".into(),
            target_commitish: "main".into(),
            name: None,
            body: None,
            is_draft: false,
            is_prerelease: false,
            author_id: 1,
            created_at: now,
            published_at: None,
        };
        assert_eq!(release.tag_name, "v1.0");
    }

    #[test]
    fn test_webhook_creation() {
        let now = chrono::Utc::now().naive_utc();
        let webhook = Webhook {
            id: 1,
            repo_id: 1,
            url: "http://test".into(),
            content_type: "json".into(),
            secret: None,
            events: "*".into(),
            is_active: true,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(webhook.url, "http://test");
    }

    #[test]
    fn test_commit_creation() {
        let now = chrono::Utc::now().naive_utc();
        let commit = Commit {
            id: 1,
            repo_id: 1,
            sha: "abcdef".into(),
            tree_sha: "123456".into(),
            parent_shas: "".into(),
            message: "initial".into(),
            author_name: "test".into(),
            author_email: "test@example.com".into(),
            author_date: now,
            committer_name: "test".into(),
            committer_email: "test@example.com".into(),
            committer_date: now,
            created_at: now,
        };
        assert_eq!(commit.sha, "abcdef");
    }

    #[test]
    fn test_tree_creation() {
        let now = chrono::Utc::now().naive_utc();
        let tree = Tree {
            id: 1,
            repo_id: 1,
            sha: "123456".into(),
            entries: "[]".into(),
            created_at: now,
        };
        assert_eq!(tree.sha, "123456");
    }

    #[test]
    fn test_blob_creation() {
        let now = chrono::Utc::now().naive_utc();
        let blob = Blob {
            id: 1,
            repo_id: 1,
            sha: "789012".into(),
            size: 4,
            content: Some(vec![1, 2, 3, 4]),
            created_at: now,
        };
        assert_eq!(blob.sha, "789012");
    }
}
