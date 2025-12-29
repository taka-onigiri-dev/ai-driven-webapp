FROM rust:1.83-slim

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

# Copy backend directory
COPY backend /app

# Build dependencies (this layer will be cached)
RUN cargo build --release || true

EXPOSE 8080

CMD ["cargo", "run", "--release"]
