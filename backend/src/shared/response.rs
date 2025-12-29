use serde::Serialize;

/// 成功レスポンス用の構造体
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

/// ページネーション情報
#[derive(Debug, Serialize)]
pub struct Pagination {
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

/// ページネーション付きレスポンス
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: Pagination,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, page: i64, per_page: i64) -> Self {
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Self {
            data,
            pagination: Pagination {
                total,
                page,
                per_page,
                total_pages,
            },
        }
    }
}
