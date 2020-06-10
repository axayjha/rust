#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

// use rand::Rng;
use std::io::stdin;
use crate::State::Locked;

fn if_statement() {
    let temp = 15;
    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("day is {}", day);
}

fn while_and_loop() {
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        println!("x = {}", x);
    }
    let mut y = 1;
    loop // while true
    {
        y *= 2;
        println!("y = {}", y);
        if y == 1<<10 {
            break;
        }
    }
}

fn for_loop() {
    for x in 1..11 {
        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}

fn match_ex() {
    let country_code = 144;
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "Unknown", //[1, 1000]
        _ => "invalid" // default
    };

    println!("the country with code {} is {}",
    country_code, country);
}

enum State {
    Locked,
    Failed,
    Unlocked
}

fn combination_lock() {
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end())
                    }
                    Err(_) => continue
                }
                if entry == code {
                    state = State::Unlocked;
                    continue;
                }

                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("FAILED");
                entry.clear(); // makes it an empty string
                state = State::Locked;
                continue
            }
            State::Unlocked => {
                println!("UNLOCKED");
                return;
            }
        };
    }
}

fn main() {
    // println!("Hello, world!");
    // if_statement();
    // while_and_loop();
    // for_loop();
    // match_ex();
    combination_lock();
}


