use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditAction {
    Login,
    Logout,
    Register,
    TokenRefresh,
    Create,
    Read,
    Update,
    Delete,
}

impl AuditAction {
    pub fn as_str(&self) -> &str {
        match self {
            AuditAction::Login => "LOGIN",
            AuditAction::Logout => "LOGOUT",
            AuditAction::Register => "REGISTER",
            AuditAction::TokenRefresh => "TOKEN_REFRESH",
            AuditAction::Create => "CREATE",
            AuditAction::Read => "READ",
            AuditAction::Update => "UPDATE",
            AuditAction::Delete => "DELETE",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuditLogEntry {
    // User information
    pub user_id: Option<i64>,
    pub user_email: Option<String>,
    pub user_role: Option<String>,

    // Action information
    pub action: AuditAction,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,

    // Request information
    pub http_method: String,
    pub endpoint: String,
    pub request_params: Option<String>,

    // Response information
    pub status_code: i32,
    pub success: bool,
    pub error_message: Option<String>,
    pub response_time_ms: Option<i32>,

    // Client information
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,

    // Location information
    pub country: Option<String>,
    pub timezone: Option<String>,
}

impl AuditLogEntry {
    pub fn new(
        http_method: String,
        endpoint: String,
        status_code: i32,
        success: bool,
    ) -> Self {
        Self {
            user_id: None,
            user_email: None,
            user_role: None,
            action: AuditAction::Read,
            resource_type: None,
            resource_id: None,
            http_method,
            endpoint,
            request_params: None,
            status_code,
            success,
            error_message: None,
            response_time_ms: None,
            ip_address: None,
            user_agent: None,
            country: None,
            timezone: None,
        }
    }

    pub fn with_user(mut self, user_id: i64, email: String, role: String) -> Self {
        self.user_id = Some(user_id);
        self.user_email = Some(email);
        self.user_role = Some(role);
        self
    }

    pub fn with_action(mut self, action: AuditAction) -> Self {
        self.action = action;
        self
    }

    pub fn with_resource(mut self, resource_type: String, resource_id: Option<String>) -> Self {
        self.resource_type = Some(resource_type);
        self.resource_id = resource_id;
        self
    }

    pub fn with_request_params(mut self, params: String) -> Self {
        self.request_params = Some(params);
        self
    }

    pub fn with_error(mut self, error: String) -> Self {
        self.error_message = Some(error);
        self
    }

    pub fn with_response_time(mut self, time_ms: i32) -> Self {
        self.response_time_ms = Some(time_ms);
        self
    }

    pub fn with_client_info(mut self, ip: Option<String>, user_agent: Option<String>) -> Self {
        self.ip_address = ip;
        self.user_agent = user_agent;
        self
    }

    pub fn with_location(mut self, country: Option<String>, timezone: Option<String>) -> Self {
        self.country = country;
        self.timezone = timezone;
        self
    }
}
