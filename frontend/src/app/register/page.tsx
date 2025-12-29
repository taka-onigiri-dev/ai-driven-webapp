'use client'

import { useState } from 'react'
import { useRouter } from 'next/navigation'
import { useForm } from 'react-hook-form'
import { authApi } from '@/lib/api/auth'
import { useAuthStore } from '@/lib/store/auth'
import { RegisterRequest } from '@/types'

export default function RegisterPage() {
  const router = useRouter()
  const { setAuth } = useAuthStore()
  const [error, setError] = useState<string>('')
  const [isLoading, setIsLoading] = useState(false)

  const {
    register,
    handleSubmit,
    formState: { errors },
    watch,
  } = useForm<RegisterRequest & { passwordConfirm: string }>()

  const password = watch('password')

  const onSubmit = async (data: RegisterRequest & { passwordConfirm: string }) => {
    setError('')
    setIsLoading(true)

    try {
      const { passwordConfirm, ...registerData } = data
      const response = await authApi.register(registerData)
      const { user, access_token, refresh_token } = response.data

      setAuth(user, access_token, refresh_token)
      router.push('/dashboard')
    } catch (err: any) {
      setError(err.message || '登録に失敗しました')
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-md w-full space-y-8">
        <div>
          <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
            新規登録
          </h2>
        </div>
        <form className="mt-8 space-y-6" onSubmit={handleSubmit(onSubmit)}>
          {error && (
            <div className="rounded-md bg-red-50 p-4">
              <div className="text-sm text-red-700">{error}</div>
            </div>
          )}

          <div className="rounded-md shadow-sm space-y-4">
            <div>
              <label htmlFor="name" className="block text-sm font-medium text-gray-700">
                名前
              </label>
              <input
                {...register('name', {
                  required: '名前は必須です',
                  minLength: { value: 1, message: '名前は1文字以上で入力してください' },
                  maxLength: { value: 100, message: '名前は100文字以内で入力してください' },
                })}
                type="text"
                className="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-primary-500 focus:border-primary-500 sm:text-sm"
                placeholder="山田 太郎"
              />
              {errors.name && (
                <p className="mt-1 text-sm text-red-600">{errors.name.message}</p>
              )}
            </div>

            <div>
              <label htmlFor="email" className="block text-sm font-medium text-gray-700">
                メールアドレス
              </label>
              <input
                {...register('email', {
                  required: 'メールアドレスは必須です',
                  pattern: {
                    value: /^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$/i,
                    message: '有効なメールアドレスを入力してください',
                  },
                  maxLength: { value: 255, message: 'メールアドレスは255文字以内で入力してください' },
                })}
                type="email"
                className="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-primary-500 focus:border-primary-500 sm:text-sm"
                placeholder="example@example.com"
              />
              {errors.email && (
                <p className="mt-1 text-sm text-red-600">{errors.email.message}</p>
              )}
            </div>

            <div>
              <label htmlFor="password" className="block text-sm font-medium text-gray-700">
                パスワード
              </label>
              <input
                {...register('password', {
                  required: 'パスワードは必須です',
                  minLength: { value: 8, message: 'パスワードは8文字以上で入力してください' },
                  maxLength: { value: 100, message: 'パスワードは100文字以内で入力してください' },
                  pattern: {
                    value: /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d).+$/,
                    message: '英大文字、英小文字、数字を含めてください',
                  },
                })}
                type="password"
                className="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-primary-500 focus:border-primary-500 sm:text-sm"
                placeholder="••••••••"
              />
              {errors.password && (
                <p className="mt-1 text-sm text-red-600">{errors.password.message}</p>
              )}
              <p className="mt-1 text-sm text-gray-500">
                8文字以上、英大文字・英小文字・数字を含む
              </p>
            </div>

            <div>
              <label htmlFor="passwordConfirm" className="block text-sm font-medium text-gray-700">
                パスワード（確認）
              </label>
              <input
                {...register('passwordConfirm', {
                  required: 'パスワード（確認）は必須です',
                  validate: (value) => value === password || 'パスワードが一致しません',
                })}
                type="password"
                className="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-primary-500 focus:border-primary-500 sm:text-sm"
                placeholder="••••••••"
              />
              {errors.passwordConfirm && (
                <p className="mt-1 text-sm text-red-600">{errors.passwordConfirm.message}</p>
              )}
            </div>
          </div>

          <div>
            <button
              type="submit"
              disabled={isLoading}
              className="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:opacity-50"
            >
              {isLoading ? '登録中...' : '登録する'}
            </button>
          </div>

          <div className="text-center">
            <a href="/login" className="text-primary-600 hover:text-primary-500">
              既にアカウントをお持ちの方はこちら
            </a>
          </div>
        </form>
      </div>
    </div>
  )
}
