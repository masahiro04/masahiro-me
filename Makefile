dev:
	cd crates/app && \
	yarn install && \
	yarn build:tailwind && \
	trunk serve

build:
	cd crates/app && \
	yarn install && \
	yarn build:tailwind && \
	trunk build --release

preview:
	cd crates/app && trunk serve --release
