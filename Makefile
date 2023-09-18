dev:
	cd crates/app && \
	yarn install && \
	yarn build:tailwind && \
	trunk serve

build:
	cd crates/server && \
	trunk build --release -d ./dist && \
	cp robots.txt ./dist/robots.txt && \
	cargo build --release --features=ssr --bin server --

run:
	cd crates/server && \
	cargo run --release --features=ssr --bin server -- --dir dist
