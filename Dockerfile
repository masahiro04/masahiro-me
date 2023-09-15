FROM rust:1.69.0-alpine as builder

# 必要なツールやライブラリをインストール
RUN apk add --no-cache build-base npm binaryen pkgconfig openssl-dev

WORKDIR /usr
COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
# for local dev
# RUN cargo install wasm-bindgen-cli --version 0.2.87

RUN cargo build --release --target x86_64-unknown-linux-musl
# RUN make ssr_build

WORKDIR /usr/crates/ssr_server
RUN cargo build --release --target x86_64-unknown-linux-musl --features=ssr --bin simple_ssr_server --

# # lddコマンドで動的リンクを検証
# RUN ldd /usr/src/app/target/x86_64-unknown-linux-musl/release/simple_ssr_server
# RUN ldd /usr/src/app/target/x86_64-unknown-linux-musl/release/simple_ssr_server
RUN ls -la
RUN ls -la
RUN ls -la

RUN ldd /usr/target/x86_64-unknown-linux-musl/release/simple_ssr_server

# # Runtime Stage
FROM alpine:latest
#
# # 必要なライブラリをインストール
RUN apk add --no-cache libgcc libstdc++ ca-certificates

EXPOSE 8080
COPY --from=builder /usr/crates/ssr_server/dist/ /dist/
# COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/simple_ssr_server /simple_ssr_server
COPY --from=builder /usr/target/x86_64-unknown-linux-musl/release/simple_ssr_server /simple_ssr_server

ENTRYPOINT ["./simple_ssr_server"]
CMD ["--dir", "dist"]
