#![allow(dead_code)]
#![allow(unused_imports)]

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
fn main() {
    // structures();
    enumerations();
}

