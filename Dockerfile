FROM rust:1.69.0 as builder

RUN apt-get update && apt-get install -y make nodejs g++ binaryen
# RUN apk update && apk add --no-cache pkgconfig openssl openssl-dev musl-dev bash gcc g++ make

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

# RUN ls -l /usr/src/app/target/x86_64-unknown-linux-musl/
RUN ls -l /usr/local/cargo/bin/
RUN ls -l /usr/local/cargo/bin/
RUN ls -l /usr/local/cargo/bin/

FROM scratch
ENV PORT=3002
COPY --from=builder /usr/src/app/crates/ssr_server/dist/ /dist/
# COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/simple_ssr_server /simple_ssr_server
# COPY --from=builder /usr/src/app/target/release/simple_ssr_server /usr/local/bin/simple_ssr_server
COPY --from=builder /usr/src/app/target/release/simple_ssr_server /simple_ssr_server
# RUN echo "Before listing root directory"
RUN ls -l /
RUN ls -l /
RUN ls -l /
# RUN echo "After listing root directory"

ENTRYPOINT ["./simple_ssr_server"]
CMD ["--dir", "dist"]

