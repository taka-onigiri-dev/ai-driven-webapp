# Backend Dockerfile (Development)
# Note: Using nightly for edition2024 support (temporary until stable supports it)
FROM rust:nightly

# 作業ディレクトリ設定
WORKDIR /app

# 必要なパッケージのインストール
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

# 依存関係のキャッシュ用に先にCargo.tomlをコピー
COPY Cargo.toml Cargo.lock* ./

# ダミーのsrcを作成して依存関係をビルド（キャッシュ最適化）
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true
RUN rm -rf src

# ソースコードをコピー
COPY . .

# デフォルトポート
EXPOSE 8080

# 開発用コマンド（docker-compose.ymlでオーバーライド）
CMD ["cargo", "run"]
