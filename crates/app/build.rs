fn main() {
    // 環境からAPI_URLを読み取る
    // 環境変数は以下によって設定される:
    // - ローカル: Makefileが.envファイルをロード
    // - CI: GitHub Actionsが環境変数を設定
    let api_url = std::env::var("API_URL")
        .expect("API_URL environment variable must be set. Create a .env file or set it in your environment.");

    // rustcに渡してenv!()がアクセスできるようにする
    println!("cargo:rustc-env=API_URL={}", api_url);

    // API_URLが変更された場合に再実行
    println!("cargo:rerun-if-env-changed=API_URL");
}
