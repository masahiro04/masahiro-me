# FROM rust:1.69.0-alpine as builder
#
# RUN apk update && apk add --no-cache \
#         make \
#         g++ \
#         binaryen \
#         bash \
#         musl-dev \
#         openssl-dev \
#         pkgconfig
# # https://qiita.com/yagince/items/077d209ecca644398ea3 を参考に実装
# ENV CARGO_BUILD_TARGET_DIR=/tmp/target
#
# WORKDIR /usr
# COPY . .
#
# # TODO: trunkやwasm-bindgenなどをcacheしたい
# RUN rustup target add x86_64-unknown-linux-musl
# RUN rustup target add wasm32-unknown-unknown
# RUN cargo install --locked trunk
# # for local dev
# RUN cargo install wasm-bindgen-cli --version 0.2.87
#
# RUN cargo build --release --target=x86_64-unknown-linux-musl
# # RUN cargo build --release
# RUN make ssr_build
#
# FROM scratch
# #
# COPY --from=builder /usr/ssr_server/dist/ /dist/
# # COPY --from=builder /tmp/target/release/simple_ssr_server /usr/local/bin/simple_ssr_server
# COPY --from=builder /tmp/target/release/simple_ssr_server /simple_ssr_server
# EXPOSE 8080
#
# ENTRYPOINT ["./simple_ssr_server"]
# CMD ["--dir", "dist"]


# FROM rust:1.69.0-alpine as builder
#
# WORKDIR /usr/src/app
# RUN apk update && apk add --no-cache musl-dev bash
#
# COPY ./Cargo.lock ./Cargo.lock
# COPY ./crates/app/Cargo.toml ./crates/app/Cargo.toml
# COPY ./crates/ssr_server/Cargo.toml ./crates/ssr_server/Cargo.toml
# RUN mkdir ./crates/app/src/
# RUN touch ./crates/app/src/lib.rs
# RUN mkdir ./crates/ssr_server/src/
# RUN touch ./crates/ssr_server/src/lib.rs
#
# WORKDIR /usr/src/app/crates/ssr_server
# RUN rustup target add x86_64-unknown-linux-musl
# RUN rustup target add wasm32-unknown-unknown
# # for local dev
# # RUN cargo install wasm-bindgen-cli --version 0.2.87
# RUN cargo build --release --target=x86_64-unknown-linux-musl
# WORKDIR /usr/src/app
# COPY . .
# RUN cargo install --path ./crates/ssr_server --target=x86_64-unknown-linux-musl
#
# FROM scratch
# ENV PORT=3002
# COPY --from=builder /usr/local/cargo/bin/ssr_server /usr/local/bin/ssr_server
#
# ENTRYPOINT ["./simple_ssr_server"]
# CMD ["--dir", "dist"]

FROM rust:1.69.0-alpine as builder

WORKDIR /usr/src/app

# Alpineでのパッケージの更新とインストール
RUN apk update && apk add --no-cache musl-dev bash

# 全てのソースファイルをコピー
# ターゲットの追加
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add wasm32-unknown-unknown

# ビルドの実行
WORKDIR /usr/src/app/crates/ssr_server
RUN cargo build --release --target=x86_64-unknown-linux-musl

# ローカルのパッケージとしてインストール
WORKDIR /usr/src/app
RUN cargo install --path ./crates/ssr_server --target=x86_64-unknown-linux-musl

# 実行用の軽量なコンテナイメージ
FROM scratch
ENV PORT=3002
COPY --from=builder /usr/local/cargo/bin/ssr_server /usr/local/bin/ssr_server

ENTRYPOINT ["./simple_ssr_server"]
CMD ["--dir", "dist"]
