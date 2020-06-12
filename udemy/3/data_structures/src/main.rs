#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::mem;
mod pm;
struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

fn structures() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("point p is at ({}, {})", p.x, p.y);

    let p2 = Point {x : 5.0, y: 10.0};
    let _my_line = Line {start:p , end: p2};
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tuple
    Cmyk { cyan: u8, magenta: u8, yellow: u8, black: u8}, // struct
}

fn enumerations() {
    let c: Color = Color::Cmyk {cyan: 0, magenta:90, yellow: 0,  black:255};

    match c {
        Color::Red => println!("R"),
        Color::Green => println!("G"),
        Color::Blue => println!("B"),
        Color::RgbColor(0,0,0)
        | Color::Cmyk {cyan: _, magenta: _, yellow: _, black: 255} => println!("black"), // or case
        Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        _ => () // do nothing
    };
}

union IntOrFloat {
    i: i32,
    f: f32
}

fn union_ex(){
    let mut iof = IntOrFloat {i : 123};
    iof.i = 42;

    let value = unsafe { iof.i };
    println!("iof.i = {}", value);

    process_value(iof);

}

fn process_value(iof: IntOrFloat) {
    unsafe{
        match iof {
            IntOrFloat{i: 42} => {
                println!("meaning of life value");
            }

            IntOrFloat { f } => {
                println!("value = {}", f);
            }
        }
    }
}

fn option_test() {
    let x = 3.0;
    let y = 3.0;

    // Option -> Some(v) | None
    let result =
        if y != 0.0 { Some(x/y) } else { None };

    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("Can't divide by 0")
    }

    if let Some(z) = result {
        println!("result = {}", z);
    }

    // while let
}

fn array_test() {
    let mut a:[i32; 5] = [1, 2, 3, 4, 5];
    println!("a has {} elements, first is {}",
        a.len(), a[0]);
    a[0] = 321;
    println!("a[0] = {}", a[0]);

    println!("{:?}", a);

    if a != [1, 2, 3, 4, 5] {
        println!("Doesn't match");
    }
    let b = [1u16; 10];
    println!("{:?}", b);
    for i in 0..b.len(){
        println!("{}", b[i]);
    }

    println!("b took up {} bytes", std::mem::size_of_val(&b));

    let mtx: [[f32; 3]; 2] =
    [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]

    ];

    println!("{:?}", mtx);

    for i in 0..mtx.len()
    {
        for j in 0..mtx[i].len()
        {
            if i==j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}

fn use_slice(slice: &mut [i32]) {
    println!("first element = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

fn slices() {
    let mut data = [1, 2, 3, 4, 5];

    use_slice(&mut data[1..4]);
    // use_slice(&mut data);
    println!("{:?}", data);

}

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x+y, x*y)
}

fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // destructuring
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    let sp2 = sum_and_product(4, 7);
    let  combined = (sp, sp2);

    println!("{:?}", combined);
    println!("last element = {}", (combined.1).1);

    let ((c, d), (e, f)) = combined;

    let foo = (true, 42.0, -1i8);

    let meaning = (42,);

    println!("{:?}", meaning);
}




fn main() {
    // structures();
    // enumerations();
    // union_ex();
    // option_test();
    // array_test();
    // slices();
    // tuples();

    //pm::pattern_matching();
    pm::generics();

}


