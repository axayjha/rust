#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::mem;

fn core_data_types()
{
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

fn operators()
{
    // arithmetic

    let mut a = 2+3*4;
    println!("a= {}", a);
    a = a+1;
    // a++

    a -= 2;

    println!("remainder of {} / {} = {}", a , 3, (a%3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3); // powi, as i is int
    let b_to_pi = f64::powf(b, std::f64:: consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    let c = 1 | 2;
    println!("1|2 = {}", c);

    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0;
    println!("pi < 4 = {}", pi_less_4);
}

fn scope_and_shadowing()
{
    let a = 123;
    let a  = 1234;
    {
        let b = 456;
        println!("inside, b = {}", b);

        let a = 777;
        println!("inside, a = {}", a);
    }

    println!("outside, a = {}", a);
    // println!("outside, b = {}", b);
}


fn main() {
    println!("Hello, world!");
    operators();
}
