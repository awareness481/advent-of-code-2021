use std::env;
mod input;
use std::isize;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let key = args[1].clone();

    let unformatted_input =
        input::fetch_input("https://adventofcode.com/2021/day/3/input", key).await;
    let input = unformatted_input.split('\n').collect::<Vec<&str>>();

    let mut sum: Vec<i16> = vec![];
    let mut inverse_sum: Vec<i16> = vec![];
    let length = input[0].len();

    for _ in 0..length {
        sum.push(0);
        inverse_sum.push(0);
    }

    for line in input.iter() {
        if line.is_empty() {
            continue;
        }

        let chars: Vec<char> = line.chars().collect();

        for i in 0..chars.len() {
            let c = chars[i].to_digit(10).unwrap() as i16;

            if c == 1 {
                sum[i] += 1;
            }
        }
    }

    for i in 0..length {
        if sum[i] < input.len() as i16 / 2 {
            sum[i] = 0;
            inverse_sum[i] = 1;
        } else {
            inverse_sum[i] = 0;
            sum[i] = 1;
        }
    }

    let gamma_binary = sum
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    let epsilon_binary = inverse_sum
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    let gamma = isize::from_str_radix(&gamma_binary, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon_binary, 2).unwrap();
    dbg!(gamma * epsilon);
}
