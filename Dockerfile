FROM rust:1.69-alpine as builder

RUN apk add --no-cache make g++ binaryen bash

RUN apk update && apk add --no-cache musl-dev make binaryen bash
# https://qiita.com/yagince/items/077d209ecca644398ea3 を参考に実装
ENV CARGO_BUILD_TARGET_DIR=/tmp/target

WORKDIR /usr
COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
# for local dev
# RUN cargo install wasm-bindgen-cli --version 0.2.87

RUN cargo build --release --target=x86_64-unknown-linux-musl
RUN make ssr_build

FROM scratch
#
COPY --from=builder /usr/ssr_server/dist/ /dist/
COPY --from=builder /tmp/target/release/simple_ssr_server /usr/local/bin/simple_ssr_server
EXPOSE 8080

ENTRYPOINT ["./simple_ssr_server"]
CMD ["--dir", "dist"]
