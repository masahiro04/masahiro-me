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


FROM rust:1.69.0-alpine as builder

RUN apk update && apk add --no-cache \
        make \
        # g++ \
        binaryen \
        bash \
        musl-dev \
        openssl-dev \
        pkgconfig
# https://qiita.com/yagince/items/077d209ecca644398ea3 を参考に実装
# ENV CARGO_BUILD_TARGET_DIR=/tmp/target
WORKDIR /usr
COPY . .


# WORKDIR /usr/src/app
# RUN apk update && apk add --no-cache musl-dev
COPY ./Cargo.lock ./Cargo.lock
COPY ./ssr_server/Cargo.toml ./ssr_server/Cargo.toml

RUN mkdir ./ssr_server/src/
RUN touch ./ssr_server/src/lib.rs

WORKDIR /usr/ssr_server

RUN rustup target add wasm32-unknown-unknown
RUN rustup target add x86_64-unknown-linux-musl

RUN cargo build --release --target=x86_64-unknown-linux-musl

WORKDIR /usr/src/app
COPY . .
RUN cargo install --path ./crates/api --target=x86_64-unknown-linux-musl
