'use client'

import { useEffect } from 'react'
import { useRouter } from 'next/navigation'
import { useAuthStore } from '@/lib/store/auth'
import { authApi } from '@/lib/api/auth'

export default function DashboardPage() {
  const router = useRouter()
  const { user, accessToken, clearAuth, isAuthenticated } = useAuthStore()

  useEffect(() => {
    if (!isAuthenticated()) {
      router.push('/login')
    }
  }, [isAuthenticated, router])

  const handleLogout = async () => {
    if (accessToken) {
      try {
        await authApi.logout(accessToken)
      } catch (error) {
        console.error('Logout error:', error)
      }
    }
    clearAuth()
    router.push('/')
  }

  if (!user) {
    return null
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <nav className="bg-white shadow-sm">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between h-16">
            <div className="flex items-center">
              <h1 className="text-xl font-bold text-gray-900">ダッシュボード</h1>
            </div>
            <div className="flex items-center">
              <button
                onClick={handleLogout}
                className="ml-4 px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700"
              >
                ログアウト
              </button>
            </div>
          </div>
        </div>
      </nav>

      <main className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
        <div className="px-4 py-6 sm:px-0">
          <div className="bg-white overflow-hidden shadow rounded-lg">
            <div className="px-4 py-5 sm:p-6">
              <h2 className="text-2xl font-bold text-gray-900 mb-4">
                ようこそ、{user.name}さん！
              </h2>
              <dl className="grid grid-cols-1 gap-x-4 gap-y-6 sm:grid-cols-2">
                <div>
                  <dt className="text-sm font-medium text-gray-500">メールアドレス</dt>
                  <dd className="mt-1 text-sm text-gray-900">{user.email}</dd>
                </div>
                <div>
                  <dt className="text-sm font-medium text-gray-500">ロール</dt>
                  <dd className="mt-1 text-sm text-gray-900">{user.role}</dd>
                </div>
                <div>
                  <dt className="text-sm font-medium text-gray-500">ステータス</dt>
                  <dd className="mt-1 text-sm text-gray-900">
                    {user.is_active ? 'アクティブ' : '非アクティブ'}
                  </dd>
                </div>
                <div>
                  <dt className="text-sm font-medium text-gray-500">ユーザーID</dt>
                  <dd className="mt-1 text-sm text-gray-900">{user.id}</dd>
                </div>
              </dl>
            </div>
          </div>

          <div className="mt-6 bg-white overflow-hidden shadow rounded-lg">
            <div className="px-4 py-5 sm:p-6">
              <h3 className="text-lg font-medium text-gray-900 mb-2">
                システム情報
              </h3>
              <p className="text-sm text-gray-600">
                このダッシュボードは、AI駆動開発によって構築されました。
              </p>
              <p className="mt-2 text-sm text-gray-600">
                - バックエンド: Rust + Actix Web + PostgreSQL
              </p>
              <p className="text-sm text-gray-600">
                - フロントエンド: Next.js 14 + TypeScript + Tailwind CSS
              </p>
            </div>
          </div>
        </div>
      </main>
    </div>
  )
}
