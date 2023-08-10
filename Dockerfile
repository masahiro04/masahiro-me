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
# for local dev
# RUN cargo install wasm-bindgen-cli --version 0.2.87

RUN cargo build --release
RUN make ssr_build

# Runtime Stage
FROM debian:bullseye-slim
# for local dev
# RUN apt-get update && apt-get install -y libgcc1 libstdc++6 bash ca-certificates
# for prod
RUN apt-get update && apt-get install -y libgcc1 libstdc++6 ca-certificates

EXPOSE 8080
COPY --from=builder /usr/ssr_server/dist/ /dist/
COPY --from=builder /tmp/target/release/simple_ssr_server /simple_ssr_server

ENTRYPOINT ["./simple_ssr_server"]
CMD ["--dir", "dist"]
