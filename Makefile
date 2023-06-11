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

dev_cf_pages:
	rm -rf functions && \
	cd app && \
	trunk build --release -d ../runner/dist && \
	cd ../ && \
	cd middlewares && \
	cp -r src ../functions && \
	cd ../ && \
	wrangler pages dev runner/dist

ssr_dev:
	cd app && \
	cd ../ssr_server && \
	trunk build --release -d ./dist && \
	cp robots.txt ./dist/robots.txt && \
	cargo run --features=ssr --bin simple_ssr_server -- --dir dist
