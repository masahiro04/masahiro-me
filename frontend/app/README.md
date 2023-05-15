### how to run
```
trunk serve
```


trunk build --release -d ./dist
cargo run --features=ssr --bin simple_ssr_server -- --dir dist
