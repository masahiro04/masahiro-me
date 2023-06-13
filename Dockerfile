FROM rust:1.69.0

RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
RUN apt-get update && apt-get install -y make nodejs g++ binaryen
RUN npm install -g yarn

# https://qiita.com/yagince/items/077d209ecca644398ea3 を参考に実装
ENV CARGO_BUILD_TARGET_DIR=/tmp/target

WORKDIR /usr/src/ssr_server
COPY . .

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN cargo build --release

EXPOSE 8080

CMD ["make", "ssr_dev"]
