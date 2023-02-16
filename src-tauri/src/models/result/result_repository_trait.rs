use async_trait::async_trait;

use crate::services::payload::result_payload::ResultPayload;

#[async_trait]
pub trait ResultRepository {
    // 試合結果の検索
    async fn fetch(&self) -> Result<Vec<ResultPayload>, String>;
}
