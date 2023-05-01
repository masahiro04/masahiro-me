use worker::Env;

use worker::*;

static CACHE_KEY: &str = "BLOG_CONTENT";
static CACHE_TTL: u64 = 86_400; // 24 hours

pub struct KvClient<'a> {
    env: &'a Env,
}

impl<'a> KvClient<'a> {
    pub fn new(env: &'a Env) -> Self {
        Self { env }
    }
    pub async fn put(&self, key: &str, data: String) -> Result<()> {
        let cache = match self.env.kv(CACHE_KEY) {
            Ok(cache) => cache,
            Err(e) => {
                console_log!("{}", "cache not found".to_string());
                eprintln!("Failed to retrieve KV store: {}", e);
                std::process::exit(1);
            }
        };
        let builder = match cache.put(&key, data) {
            Ok(_builder) => _builder,
            Err(e) => {
                console_log!("{}", e.to_string());
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };
        match builder.expiration_ttl(CACHE_TTL).execute().await {
            Ok(_val) => Ok(()),
            Err(e) => {
                eprintln!("{}", e);
                console_log!("{}", e.to_string());
                std::process::exit(1);
            }
        }
    }

    pub async fn get(&self, key: &str) -> Option<Response> {
        let cache = match self.env.kv(CACHE_KEY) {
            Ok(cache) => cache,
            Err(e) => {
                console_log!("cache not found {}", e.to_string());
                // eprintln!("Failed to retrieve KV store: {}", e);
                // std::process::exit(1);
                return None;
            }
        };
        cache
            .get(&key)
            .text()
            .await
            .map(|data| -> Option<Response> {
                if let Some(ref data) = data {
                    let json_data: serde_json::Value = serde_json::from_str(data).ok()?;
                    let mut res = Response::ok(serde_json::to_string(&json_data).ok()?).unwrap();
                    res.headers_mut()
                        .set("content-type", "application/json")
                        .unwrap();
                    Some(res)
                } else {
                    // データが存在しない場合の処理
                    console_log!("{}", "Data not found".to_string());
                    None
                }
            })
            .unwrap()
    }
}
