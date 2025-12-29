use super::domain::AuditLogEntry;
use crate::entities::audit_logs;
use crate::shared::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait AuditLogRepository: Send + Sync {
    async fn create(&self, entry: AuditLogEntry) -> AppResult<audit_logs::Model>;
}
