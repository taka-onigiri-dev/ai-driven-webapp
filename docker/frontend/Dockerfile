# Frontend Dockerfile (Development)
FROM node:20-alpine

# 作業ディレクトリ設定
WORKDIR /app

# package.jsonをコピー
COPY package*.json ./

# 依存関係のインストール
RUN npm install

# ソースコードをコピー
COPY . .

# デフォルトポート
EXPOSE 3000

# 開発用コマンド（docker-compose.ymlでオーバーライド）
CMD ["npm", "run", "dev"]
