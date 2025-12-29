use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "audit_logs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    // User information
    pub user_id: Option<i64>,
    pub user_email: Option<String>,
    pub user_role: Option<String>,

    // Action information
    pub action: String,
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

    // Timestamp
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    Users,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
