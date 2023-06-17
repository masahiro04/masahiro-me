FROM rust:1.69.0

RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
RUN apt-get update && apt-get install -y make nodejs g++ binaryen
RUN npm install -g yarn

# WORKDIR /dummy
# COPY Cargo.toml Cargo.toml
# RUN mkdir src
# RUN echo "fn main(){}" > src/main.rs
# RUN cargo build --release


# https://qiita.com/yagince/items/077d209ecca644398ea3 を参考に実装
ENV CARGO_BUILD_TARGET_DIR=/tmp/target

WORKDIR /usr/src/ssr_server
COPY . .

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN cargo build --release

# FROM debian:10.4
# FROM debian:buster-slim
# COPY --from=builder /usr/local/cargo/bin/trunk /usr/local/bin/trunk

EXPOSE 8080

# COPY --from=builder /usr/src/ssr_server/target/release/ssr_server /usr/local/bin/ssr_server

CMD ["make", "ssr_dev"]

# Stage 1: Build the trunk
# FROM rust:1.69.0 as builder
# RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
# RUN apt-get update && apt-get install -y make nodejs g++ binaryen
# RUN npm install -g yarn
# # ENV CARGO_BUILD_TARGET_DIR=/tmp/target
#
# RUN apt-get clean && apt-get update && apt-get install -y make nodejs g++ binaryen
#
# WORKDIR /usr/src/ssr_server
# COPY . .
#
# RUN ls -al # ここでディレクトリの内容を確認します。
#
# RUN rustup target add wasm32-unknown-unknown
# RUN cargo install --locked trunk
# RUN cargo build --release
#
# RUN ls /usr/src
# RUN ls /usr/src/ssr_server
# RUN ls /usr/src/ssr_server/target/release
#
# CMD cd app && \
#     cd ../ssr_server && \
#     trunk build --release -d ./dist && \
#     cp robots.txt ./dist/robots.txt
#
# # Stage 2: Copy the trunk binary
# # FROM debian:buster-slim
# FROM debian:bullseye-slim
# # COPY --from=builder /usr/local/cargo/bin/trunk /usr/local/bin/trunk
# COPY --from=builder /usr/src/ssr_server/target/release/ssr_server /usr/local/bin/ssr_server
