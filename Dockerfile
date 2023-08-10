# FROM rust:1.69.0
#
# RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
# RUN apt-get update && apt-get install -y make nodejs g++ binaryen
# RUN npm install -g yarn
#
# # https://qiita.com/yagince/items/077d209ecca644398ea3 を参考に実装
# ENV CARGO_BUILD_TARGET_DIR=/tmp/target
#
# WORKDIR /usr/src/ssr_server
# COPY . .
#
# RUN rustup target add wasm32-unknown-unknown
# RUN cargo install --locked trunk
# RUN cargo build --release
# RUN make ssr_build
#
# EXPOSE 8080
#
# CMD ["make", "ssr_run"]

# ----------------------------------------------

FROM rust:1.69.0 as builder

RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
RUN apt-get update && apt-get install -y make nodejs g++ binaryen
RUN npm install -g yarn

# https://qiita.com/yagince/items/077d209ecca644398ea3 を参考に実装
ENV CARGO_BUILD_TARGET_DIR=/tmp/target

WORKDIR /usr
COPY . .

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN cargo build --release
RUN make ssr_build

# Runtime Stage
FROM alpine:latest
RUN apk add --no-cache libgcc libstdc++  # Rustのバイナリ実行に必要なライブラリをインストール

EXPOSE 8080
COPY --from=builder /usr/ssr_server/dist/ /dist/
COPY --from=builder /tmp/target/release/simple_ssr_server /usr/local/bin/simple_ssr_server
RUN ls -al /usr/local/bin/

RUN chmod +x /usr/local/bin/simple_ssr_server
# COPY --from=builder /usr/local/cargo/bin/simple_ssr_server /usr/local/bin/simple_ssr_server
CMD ["./simple_ssr_server", "--dir", "dist"]
