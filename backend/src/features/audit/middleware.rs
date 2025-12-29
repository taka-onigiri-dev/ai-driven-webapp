use super::domain::{AuditAction, AuditLogEntry};
use super::service::AuditService;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error, HttpMessage,
};
use std::time::Instant;

/// Middleware to log all requests
pub async fn audit_logger(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let start_time = Instant::now();

    // Extract request information
    let method = req.method().to_string();
    let path = req.path().to_string();

    // Extract IP address
    let ip_address = req
        .connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string());

    // Extract User-Agent
    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    // Extract timezone from headers (if provided)
    let timezone = req
        .headers()
        .get("x-timezone")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    // Call next service
    let res = next.call(req).await?;

    // Calculate response time
    let response_time_ms = start_time.elapsed().as_millis() as i32;

    // Extract status code
    let status_code = res.status().as_u16() as i32;
    let success = res.status().is_success();

    // Determine action based on method and path
    let action = determine_action(&method, &path);

    // Create audit log entry
    let mut entry = AuditLogEntry::new(method, path.clone(), status_code, success)
        .with_action(action)
        .with_response_time(response_time_ms)
        .with_client_info(ip_address, user_agent)
        .with_location(None, timezone);

    // Skip logging for health check endpoints
    if !path.starts_with("/health") {
        // Get database connection from request extensions
        if let Some(db) = res.request().app_data::<actix_web::web::Data<std::sync::Arc<sea_orm::DatabaseConnection>>>() {
            let audit_service = AuditService::new(db.get_ref().clone());

            // Log asynchronously (spawn task to avoid blocking)
            tokio::spawn(async move {
                if let Err(e) = audit_service.log(entry).await {
                    tracing::error!("Failed to log audit entry: {:?}", e);
                }
            });
        }
    }

    Ok(res)
}

fn determine_action(method: &str, path: &str) -> AuditAction {
    if path.contains("/login") {
        return AuditAction::Login;
    }
    if path.contains("/logout") {
        return AuditAction::Logout;
    }
    if path.contains("/register") {
        return AuditAction::Register;
    }
    if path.contains("/refresh") {
        return AuditAction::TokenRefresh;
    }

    match method {
        "GET" => AuditAction::Read,
        "POST" => AuditAction::Create,
        "PUT" | "PATCH" => AuditAction::Update,
        "DELETE" => AuditAction::Delete,
        _ => AuditAction::Read,
    }
}

/// Helper function to filter sensitive data from request body
pub fn filter_sensitive_data(body: &str) -> String {
    let sensitive_fields = vec!["password", "token", "secret", "authorization"];

    let mut filtered = body.to_string();
    for field in sensitive_fields {
        // Simple replacement for demonstration
        // In production, use proper JSON parsing
        if filtered.contains(field) {
            filtered = filtered.replace(
                &format!("\"{}\": \"", field),
                &format!("\"{}\": \"***FILTERED***", field),
            );
        }
    }

    filtered
}
