use super::domain::AuditLogEntry;
use super::infra::AuditLogRepositoryImpl;
use super::repository::AuditLogRepository;
use crate::shared::AppResult;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct AuditService {
    repo: Arc<dyn AuditLogRepository>,
}

impl AuditService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        let repo = Arc::new(AuditLogRepositoryImpl::new(db));
        Self { repo }
    }

    pub async fn log(&self, entry: AuditLogEntry) -> AppResult<()> {
        // Log asynchronously without blocking the main request
        // In production, consider using a queue system
        match self.repo.create(entry).await {
            Ok(_) => Ok(()),
            Err(e) => {
                // Don't fail the request if audit logging fails
                tracing::error!("Failed to create audit log: {:?}", e);
                Ok(())
            }
        }
    }
}
