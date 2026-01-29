# Load environment variables from .env file
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

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
