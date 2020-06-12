
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tuple
    Cmyk { cyan: u8, magenta: u8, yellow: u8, black: u8}, // struct
}


fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "dozen",
        z @ 9...11 => "lots of", // can operate on z now
        _ if (x % 2 == 0) => "some",
        _ => "a few "
    }
}

pub fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }


    let point = (3, 4);
    match (point) {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y),
        (x, 0) => print!("y axis, x = {}", x),
        // (x, y)=> println!("({}, {})", x, y)
        (_, y)=> println!("(?, {})", y)
    }

    let c: Color = Color::Cmyk {cyan: 0, magenta:90, yellow: 0,  black:255};

    match c {
        Color::Red => println!("R"),
        Color::Green => println!("G"),
        Color::Blue => println!("B"),
        Color::RgbColor(0,0,0)
        | Color::Cmyk {black: 255,..} => println!("black"), // or case
        Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        _ => () // do nothing
    };
}

struct _Point{
    x: f64,
    y: f64
}

struct  Point<T> {
    x: T,
    y: T
}

// can also use diff types T and V for x and y.

struct Line<T> {
    start: Point<T>,
    end: Point<T>
}

pub fn generics() {
    let a: Point<i32>  = Point{ x: 0, y: 0};
    let b = Point {x : 1.2, y : 3.4 };
}