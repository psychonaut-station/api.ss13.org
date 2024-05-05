use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use once_cell::sync::Lazy;
use rocket::{get, http::Status, serde::json::Json, State};
use tokio::sync::RwLock;

use crate::{
    database::{get_recent_test_merges, TestMerge},
    Database,
};

type TestMergesCache = Option<(Instant, Vec<TestMerge>)>;

static LAST_RECENT_TEST_MERGES: Lazy<Arc<RwLock<TestMergesCache>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

#[get("/recent-test-merges.json")]
pub async fn recent_test_merges(
    database: &State<Database>,
) -> Result<Json<Vec<TestMerge>>, Status> {
    {
        let recent_test_merges = LAST_RECENT_TEST_MERGES.read().await;
        if let Some((last_update, test_merges)) = &*recent_test_merges {
            if last_update.elapsed() < Duration::from_secs(600) {
                return Ok(Json(test_merges.clone()));
            }
        }
    }

    let Ok(test_merges) = get_recent_test_merges(&database.pool).await else {
        return Err(Status::InternalServerError);
    };

    let mut recent_test_merges = LAST_RECENT_TEST_MERGES.write().await;
    *recent_test_merges = Some((Instant::now(), test_merges.clone()));

    Ok(Json(test_merges))
}
