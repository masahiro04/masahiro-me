FROM rust:1.69.0-alpine as builder

# Install system dependencies
# RUN apk update && apk add curl make g++ nodejs npm binaryen bash
RUN apk update && apk add curl make g++ nodejs npm binaryen bash openssl openssl-dev pkgconfig
RUN npm install -g yarn

# Set cargo build target directory
ENV CARGO_BUILD_TARGET_DIR=/tmp/target

WORKDIR /usr
COPY . .

# Install Rust tools, targets and build
RUN rustup target add wasm32-unknown-unknown
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install --locked trunk
# for local dev
# RUN cargo install wasm-bindgen-cli --version 0.2.87

RUN cargo build --release --target x86_64-unknown-linux-musl
RUN make ssr_build

# Runtime Stage
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache libgcc libstdc++ ca-certificates

EXPOSE 8080
COPY --from=builder /usr/ssr_server/dist/ /dist/
COPY --from=builder /tmp/target/release/simple_ssr_server /simple_ssr_server

ENTRYPOINT ["./simple_ssr_server"]
CMD ["--dir", "dist"]
