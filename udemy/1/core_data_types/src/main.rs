#![allow(dead_code)]
#![allow(unused_imports)]

use std::mem;

fn main() {
    println!("Hello, world!");

    // Integral types

    let a:u8 = 123;
    println!("a = {}", a);
    // a= 124; // error

    // mut
    let mut b:i8 = 0;
    println!("b = {}", b);
    b = 1;
    println!("b = {}", b);

    let mut c = 123456789;
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    // u8 i8 u16 i16 u32 i32 u64 i64
    // isize/usize

    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS",
        z, size_of_z, size_of_z*8);

    // char
    let d = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));
    // size = 4 bytes -> not ASCII, highest UNICODE size possible

    // fp, double
    let e = 2.5; // f64 by default -> double precision
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    // bool
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
    
}
