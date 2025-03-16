dev:
	cd crates/app && \
	yarn install && \
	yarn build:tailwind && \
	trunk serve

build:
	cd crates/app && \
	yarn install && \
	yarn build:tailwind && \
	cd ../server && \
	trunk build --release -d ./dist && \
	cp robots.txt ./dist/robots.txt && \
	cp -r ../app/assets ./dist/ && \
	cp ../app/style.css ./dist/ && \
	cargo build --release --features=ssr --bin server --

run:
	cd crates/server && \
	cargo run --release --features=ssr --bin server -- --dir dist
