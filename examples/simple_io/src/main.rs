// Program to tell the user when they'll turn 100


use std::io;
extern crate chrono;

use chrono::prelude::*;

fn main() {
    let name = input("What is your name?: ").expect("Something went wrong!");
    println!("Hello, {}!", name);

    let age = input("What is you age?: ")
                .expect("Failed to get age!")
                .parse::<i32>().expect("Invalid age!");


    let current_year = Utc::now().year();
    let hundred_year = current_year - age + 100;                
    if age > 100 {
        println!("You turned 100 years old in {}", hundred_year);
    } else {
        println!("You will turn 100 years old in {}!", hundred_year);
    }
}

// mimics Python3's input method
fn input(message: &str) -> io::Result<String> {
    use std::io::Write;

    print!("{}", message);
    io::stdout().flush()?;
    
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim_end().to_owned())
    
}