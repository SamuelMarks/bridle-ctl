use actix_web::{test, web, App};
use bridle_rest::{batch_run, sync_prs};
use bridle_sdk::models::{BatchRunRequest, SyncPrsRequest};
use bridle_rest::AppState;

#[actix_web::test]
async fn test_batch_run() {
    let data = web::Data::new(AppState { db_url: "dummy".to_string() });
    let req = BatchRunRequest {
        config_path: "nonexistent.yaml".to_string(),
        safety_mode: false,
        max_repos: None,
        max_prs_per_hour: None,
    };
    let _ = batch_run(data, web::Json(req)).await;
}

#[actix_web::test]
async fn test_sync_prs() {
    let data = web::Data::new(AppState { db_url: "dummy".to_string() });
    let req = SyncPrsRequest {
        org: "test".to_string(),
        max_prs_per_hour: None,
        fork_org: None,
    };
    let _ = sync_prs(data, web::Json(req)).await;
}
