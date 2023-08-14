FROM rust:1.69.0 as builder

RUN apt-get update && apt-get install -y make nodejs g++ binaryen

WORKDIR /usr/src/app
COPY . .

WORKDIR /usr/src/app/crates/ssr_server
# RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

# for local dev
# RUN cargo install wasm-bindgen-cli --version 0.2.87
RUN trunk build --release -d ./dist
RUN cp robots.txt ./dist/robots.txt
RUN cargo build --release --features=ssr --bin simple_ssr_server --

RUN ldd /usr/src/app/target/release/simple_ssr_server
RUN ldd /usr/src/app/target/release/simple_ssr_server

RUN ls -l /usr/src/app/target/release/
RUN ls -l /usr/src/app/target/release/

FROM scratch
EXPOSE 8080
COPY --from=builder /usr/src/app/crates/ssr_server/dist/ /dist/
COPY --from=builder /usr/src/app/target/release/simple_ssr_server /simple_ssr_server

# ENTRYPOINT ["/simple_ssr_server"]
# CMD ["--dir", "dist"]

CMD ["./simple_ssr_server", "--dir", "dist"]
