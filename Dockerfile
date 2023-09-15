FROM rust:1.69.0-alpine as builder

RUN apk add --no-cache build-base npm binaryen

WORKDIR /usr
COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

RUN cargo build --release --target x86_64-unknown-linux-musl

WORKDIR /usr/crates/ssr_server

RUN trunk build --release -d ./dist
RUN cp robots.txt ./dist/robots.txt
RUN cargo build --release --target x86_64-unknown-linux-musl --features=ssr --bin simple_ssr_server --

# RUN ls -la
# RUN ldd /usr/target/x86_64-unknown-linux-musl/release/simple_ssr_server

# ----
# # Runtime Stage
# FROM alpine:latest
# #
# RUN apk add --no-cache libgcc libstdc++ ca-certificates
#
# EXPOSE 8080
# COPY --from=builder /usr/crates/ssr_server/dist/ /dist/
# COPY --from=builder /usr/target/x86_64-unknown-linux-musl/release/simple_ssr_server /simple_ssr_server
#
# ENTRYPOINT ["./simple_ssr_server"]
# CMD ["--dir", "dist"]
# ----

FROM scratch

EXPOSE 8080
COPY --from=builder /usr/crates/ssr_server/dist/ /dist/
COPY --from=builder /usr/target/x86_64-unknown-linux-musl/release/simple_ssr_server /simple_ssr_server

ENTRYPOINT ["/simple_ssr_server"]
CMD ["--dir", "dist"]
