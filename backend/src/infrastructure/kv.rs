use worker::kv::KvStore;
use worker::*;

pub struct KvClient<'a> {
    store: &'a KvStore,
}

impl<'a> KvClient<'a> {
    pub fn new(store: &'a KvStore) -> Self {
        Self { store }
    }
    pub async fn put(&self, key: &str, data: String) -> Result<()> {
        let builder = match self.store.put(&key, data) {
            Ok(_builder) => _builder,
            Err(e) => {
                console_log!("{}", e.to_string());
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };
        // 24 hours
        match builder.expiration_ttl(86_400).execute().await {
            Ok(_val) => Ok(()),
            Err(e) => {
                eprintln!("{}", e);
                console_log!("{}", e.to_string());
                std::process::exit(1);
            }
        }
    }
    pub async fn get(&self, key: &str) -> Option<Response> {
        self.store
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
