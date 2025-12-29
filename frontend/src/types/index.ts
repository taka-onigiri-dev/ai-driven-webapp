// API Response Types
export interface ApiResponse<T> {
  data: T
}

export interface ErrorResponse {
  error: {
    code: string
    message: string
  }
}

// User Types
export interface User {
  id: number
  email: string
  name: string
  role: string
  is_active: boolean
  created_at: string
  updated_at: string
}

// Auth Types
export interface LoginRequest {
  email: string
  password: string
}

export interface RegisterRequest {
  email: string
  password: string
  name: string
}

export interface AuthResponse {
  access_token: string
  refresh_token: string
  user: User
}

// Pagination Types
export interface Pagination {
  total: number
  page: number
  per_page: number
  total_pages: number
}

export interface PaginatedResponse<T> {
  data: T[]
  pagination: Pagination
}
