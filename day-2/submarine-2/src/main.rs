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

    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;

    for item in &formatted_input {
        let instructions = item.split(' ').collect::<Vec<&str>>();

        if instructions[0] == "forward" {
            let x = instructions[1].parse::<i32>().unwrap();
            horizontal += x;
            depth += aim * x;
        } else if instructions[0] == "up" {
            aim -= instructions[1].parse::<i32>().unwrap();
        } else if instructions[0] == "down" {
            aim += instructions[1].parse::<i32>().unwrap();
        }
    }

    println!(
        "horizontal: {}, depth: {}, aim {}, multiplication: {}",
        horizontal,
        depth,
        aim,
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
