use std::borrow::Borrow;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fetch_input(
        "https://adventofcode.com/2021/day/2/input",
        String::from(&args[1]),
    )
    .await;
    let formatted_input = input.split('\n').collect::<Vec<&str>>();

    let mut horizontal = 0;
    let mut depth = 0;

    for i in 0..formatted_input.len() - 1 {
        let instructions = formatted_input[i].split(' ').collect::<Vec<&str>>();
        dbg!(&instructions);
        if instructions[0] == "forward" {
            horizontal += instructions[1].parse::<i32>().unwrap();
        } else if instructions[0] == "up" {
            depth -= instructions[1].parse::<i32>().unwrap();
        } else if instructions[0] == "down" {
            depth += instructions[1].parse::<i32>().unwrap();
        }
    }

    println!(
        "horizontal: {}, depth: {}, multiplication: {}",
        horizontal,
        depth,
        horizontal * depth
    );
}

async fn fetch_input(url: &str, key: String) -> String {
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
    dbg!(&resp);

    match resp.borrow() {
        Ok(res) => res,
        Err(e) => panic!("Error: {}", e),
    };

    resp.unwrap().text().await.unwrap()
}
