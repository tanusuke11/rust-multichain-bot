# シンプルなRustコンテナで確実に動作させる
FROM rust:1.85.0

# 作業ディレクトリを設定
WORKDIR /app

# ソースコードと設定をコピー
COPY Cargo.toml .
COPY src ./src
COPY config.toml .
COPY .env .

# ビルド（キャッシュを使用）
RUN cargo build --release

# 最後のレイヤーでcargoを実行（ソースが変更された場合はこちらが優先される）
CMD ["cargo", "run", "--release", "--bin", "worker"]
