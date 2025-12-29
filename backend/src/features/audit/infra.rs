use super::domain::AuditLogEntry;
use super::repository::AuditLogRepository;
use crate::entities::audit_logs;
use crate::shared::AppResult;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use std::sync::Arc;

pub struct AuditLogRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

impl AuditLogRepositoryImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AuditLogRepository for AuditLogRepositoryImpl {
    async fn create(&self, entry: AuditLogEntry) -> AppResult<audit_logs::Model> {
        let now = chrono::Utc::now().into();

        let log = audit_logs::ActiveModel {
            user_id: Set(entry.user_id),
            user_email: Set(entry.user_email),
            user_role: Set(entry.user_role),
            action: Set(entry.action.as_str().to_string()),
            resource_type: Set(entry.resource_type),
            resource_id: Set(entry.resource_id),
            http_method: Set(entry.http_method),
            endpoint: Set(entry.endpoint),
            request_params: Set(entry.request_params),
            status_code: Set(entry.status_code),
            success: Set(entry.success),
            error_message: Set(entry.error_message),
            response_time_ms: Set(entry.response_time_ms),
            ip_address: Set(entry.ip_address),
            user_agent: Set(entry.user_agent),
            country: Set(entry.country),
            timezone: Set(entry.timezone),
            created_at: Set(now),
            ..Default::default()
        };

        let result = log.insert(self.db.as_ref()).await?;
        Ok(result)
    }
}
