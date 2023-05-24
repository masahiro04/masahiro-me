### how to run
```
trunk serve
```



cd ../
cd app
trunk build --release -d ./dist

cargo run --features=ssr --bin simple_ssr_server -- --dir dist


for docker
d build
d up -d
d exec ssr_server bash


cargo run --features=ssr --bin simple_ssr_server -- --dir dist
