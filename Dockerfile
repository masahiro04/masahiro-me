FROM rust:1.69.0-alpine as builder

RUN apk update && apk add --no-cache pkgconfig openssl openssl-dev musl-dev bash gcc g++ make

WORKDIR /usr/src/app
COPY . .

WORKDIR /usr/src/app/crates/ssr_server
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

# WORKDIR /usr/src/app/crates/ssr_server


# for local dev
# RUN cargo install wasm-bindgen-cli --version 0.2.87
RUN trunk build --release -d ./dist
RUN cp robots.txt ./dist/robots.txt
RUN cargo build --release --target=x86_64-unknown-linux-musl --features=ssr --bin simple_ssr_server --

RUN ls -l /usr/src/app/target/
RUN ls -l /usr/local/cargo/bin/
RUN ls -l /usr/local/cargo/bin/
RUN ls -l /usr/local/cargo/bin/

FROM scratch
ENV PORT=3002
COPY --from=builder /usr/src/app/crates/ssr_server/target/x86_64-unknown-linux-musl/release/simple_ssr_server /usr/local/bin/simple_ssr_server
# COPY --from=builder /usr/local/cargo/bin/ssr_server /usr/local/bin/ssr_server

ENTRYPOINT ["simple_ssr_server"]
CMD ["--dir", "dist"]
