dev:
	cd app && \
	trunk serve

build_cf_pages:
	cargo install wasm-opt && \
	cd app && \
	trunk build --release -d ../runner/dist && \
	cd ../ && \
	cd middlewares && \
	cp -r src ../runner/functions && \
	cd ../ && \
	cd runner && \
	cp _redirects ./dist/_redirects

ssr_build:
	cd app && \
	cd ../ssr_server && \
	trunk build --release -d ./dist && \
	cp robots.txt ./dist/robots.txt && \
	cd ../ssr_server && \
	cargo build --release --features=ssr --bin simple_ssr_server --

ssr_run:
	cd ssr_server && \
	cargo run --release --features=ssr --bin simple_ssr_server -- --dir dist

