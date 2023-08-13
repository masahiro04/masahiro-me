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
WORKDIR /usr/src/app
RUN apk update && apk add --no-cache pkgconfig openssl openssl-dev musl-dev libssl-dev bash gcc g++ make
COPY . .

WORKDIR /usr/src/app/crates/ssr_server
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/app/crates/ssr_server

RUN cargo install trunk

# for local dev
# RUN cargo install wasm-bindgen-cli --version 0.2.87
RUN trunk build --release -d ./dist
RUN cp robots.txt ./dist/robots.txt
RUN cargo build --release --target=x86_64-unknown-linux-musl --features=ssr --bin simple_ssr_server --

FROM scratch
ENV PORT=3002
COPY --from=builder /usr/local/cargo/bin/ssr_server /usr/local/bin/ssr_server

ENTRYPOINT ["./simple_ssr_server"]
CMD ["--dir", "dist"]
