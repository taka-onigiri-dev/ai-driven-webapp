import { api } from './client'
import { ApiResponse, AuthResponse, LoginRequest, RegisterRequest } from '@/types'

export const authApi = {
  login: (data: LoginRequest) =>
    api.post<ApiResponse<AuthResponse>>('/auth/login', data),

  register: (data: RegisterRequest) =>
    api.post<ApiResponse<AuthResponse>>('/auth/register', data),

  logout: (token: string) =>
    api.post('/auth/logout', {}, { token }),

  refresh: (refreshToken: string) =>
    api.post<ApiResponse<{ access_token: string }>>('/auth/refresh', {
      refresh_token: refreshToken,
    }),
}
