dev:
	cd app && \
	yarn install && \
	yarn create:tailwind && \
	trunk serve

build_cf_pages:
	cd app && \
	trunk build --release -d ../runner/dist && \
	cd ../ && \
	cd middlewares && \
	cp -r src ../runner/functions && \
	cd ../ && \
	cd runner && \
	cp _redirects ./dist/_redirects

ssr_dev:
	cd app && \
	yarn install && \
	yarn create:tailwind && \
	cd ../ssr_server && \
	trunk build --release -d ./dist && \
	cp robots.txt ./dist/robots.txt && \
	cargo run --features=ssr --bin simple_ssr_server -- --dir dist

release_app:
	cd app && \
	yarn install && \
	yarn create:tailwind && \
	cd ../ssr_server && \
	trunk build --release -d ./dist && \
	cp _redirects ./dist/_redirects && \
	cd ../middlewares && \
	cp -r src ../ssr_server