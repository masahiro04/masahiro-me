dev:
	cd crates/app && \
	yarn install && \
	yarn build:tailwind && \
	trunk serve

ssr_build:
	cd ssr_server && \
	trunk build --release -d ./dist && \
	cp robots.txt ./dist/robots.txt && \
	cargo build --release --features=ssr --bin simple_ssr_server --

ssr_run:
	cd ssr_server && \
	cargo run --release --features=ssr --bin simple_ssr_server -- --dir dist
