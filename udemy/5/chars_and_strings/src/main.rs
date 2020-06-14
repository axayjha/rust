#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_variables)]

fn strings() {
    // str
    let s: &'static str = "hello there!"; // &str: string slice
    // s = "abc"; // wrong
    // let h = s[0]; // -> wrong
    for c in s.chars().rev(){
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0){
        println!("first letter is {}", first_char);
    }


    // heap
    // String - dynamic

    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("{}", letters);

    // &str <> String
    let u:&str = &letters;

    // concatenation
    // String + str

    let z = letters + "abc";
    // let y = letters + &letters;
    let mut abc = "hello_world".to_string();

    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "goodbye"));

}

fn formatting() {
    let name = "Akshay";
    let greeting = format!("hi, I am {}, nice to meet you", name);
    println!("{}", greeting);

    let run = "run";
    let forest = "forest";
    let rfr = format!("{0}, {1}, {0}", run, forest);
    println!("{}", rfr);

    let info = format!("the name's {last}. {first} {last}", first="james", last="bond");
    println!("{}", info);

    let mixed = format!(
        "{1} {} {0} {} {data}",
        "alpha",
        "beta",
        data = "delta"
    );
    println!("{}", mixed);
}

use rand::Rng;
use std::io::stdin;

fn guessing() {
    let number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Enter your guess: ");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        if (guess < 1 || guess > 100) {
                            println!("Your guess is out of range");
                        } else if (guess < number) {
                            println!("Your guess is too low");
                        } else if guess > number {
                            println!("Your guess is too high");
                        } else {
                            println!("Correct!");
                            break;
                        }
                    },
                    Err(e) => {
                        println!("Could not read your input. {}. Try again!", e);
                    }
                }
            },
            Err(_) => continue
        }
    }
}

fn main() {
    // println!("Hello, world!");
    // strings();
    // formatting();
    guessing();
}
