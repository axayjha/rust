#![allow(dead_code)]
#![allow(unused_imports)]

fn main() {
    println!("Hello, world!");

    let a:u8 = 123;
    println!("a = {}", a);
    // a= 124; // error

    // mut
    let mut b:i8 = 0;
    println!("b = {}", b);
    b = 1;
    println!("b = {}", b);


}
