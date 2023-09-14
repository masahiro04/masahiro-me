FROM rust:1.69.0-alpine as builder

# 必要なツールやライブラリをインストール
RUN apk add --no-cache build-base npm binaryen
# RUN npm install -g yarn

# https://qiita.com/yagince/items/077d209ecca644398ea3 を参考に実装
ENV CARGO_BUILD_TARGET_DIR=/tmp/target

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
RUN cargo run --release --target x86_64-unknown-linux-musl --features=ssr --bin simple_ssr_server -- --dir dist

# # lddコマンドで動的リンクを検証
RUN ldd /tmp/target/release/simple_ssr_server
RUN ldd /tmp/target/release/simple_ssr_server
RUN ldd /tmp/target/release/simple_ssr_server

# # Runtime Stage
FROM alpine:latest
#
# # 必要なライブラリをインストール
RUN apk add --no-cache libgcc libstdc++ ca-certificates

EXPOSE 8080
COPY --from=builder /usr/crates/ssr_server/dist/ /dist/
COPY --from=builder /tmp/target/release/simple_ssr_server /simple_ssr_server

ENTRYPOINT ["./simple_ssr_server"]
CMD ["--dir", "dist"]
