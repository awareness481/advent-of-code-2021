use std::borrow::Borrow;

pub async fn fetch_input(url: &str, key: String) -> String {
    let cookie = format!("session={}", key);
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let resp = client
        .get(url)
        .header(reqwest::header::COOKIE, cookie)
        .send()
        .await;

    match resp.borrow() {
        Ok(res) => res,
        Err(e) => panic!("Error: {}", e),
    };

    resp.unwrap().text().await.unwrap()
}