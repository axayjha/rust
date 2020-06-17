extern crate rand;
extern crate Phrases;

use rand::Rng;

fn extern_crate() {
    let mut rng = rand::thread_rng();
    let b:bool = rng.gen();

    println!("{}", b);
}
use Phrases::greetings::french;
fn custom_crate_demo() {
    println!("English: {}, {}", Phrases::greetings::english::hello(), Phrases::greetings::english::goodbye());
    println!("French: {}, {}",
             french::hello(), french::goodbye());

}

fn main() {
    // println!("Hello, world!");

    // extern_crate();
    custom_crate_demo();

}
