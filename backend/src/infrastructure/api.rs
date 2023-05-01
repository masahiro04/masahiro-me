use reqwest::Client;
use serde::de::DeserializeOwned;
use std::io::{Error, ErrorKind, Result};
use worker::console_log;

pub struct ApiClient {}

impl ApiClient {
    pub async fn get<T>(&self, url: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let client = Client::new();
        let req = client.get(url).send().await.unwrap();
        match req.json::<T>().await {
            Ok(data) => Ok(data),
            Err(e) => {
                console_log!("error: {:?}", e);
                Err(Error::new(ErrorKind::Other, e))
            }
        }
    }
    pub fn new() -> Self {
        Self {}
    }
}
