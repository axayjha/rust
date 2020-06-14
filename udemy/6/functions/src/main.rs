#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]


fn print_value(x: i32) {
    println!("value = {}", x);
}

fn increase(x: &mut i32) {
    *x += 1;
}

fn product(x: i32, y: i32) -> i32 {
    x * y // no semicolon: return x*y
}

fn functions() {
    print_value(123);
    let mut z = 1;
    increase(&mut z);
    println!("z = {}", z);

    let a = 3;
    let b = 5;
    let p = product(a, b);

    println!("{} * {} = {}", a, b, p);
}


struct Point {
    x: f64,
    y : f64
}

struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx*dx + dy*dy).sqrt()
    }
}

fn methods() {
    let p = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let my_line = Line { start: p, end: p2};

    println!("length = {}", my_line.len());
}

fn say_hello() { println!("Hello"); }

fn closures() {
    let sh = say_hello;
    sh();

    let plus_one = |x: i32| -> i32 { x + 1};

    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a) );

    let two = 2;


    let plus_two = |x| {
        let mut z = x;
        z += 2;
        // z += two; // will borrow 2 permanently
        z
    };

    // let borrow_two = &mut two; // csnt do

    println!("{} + 2 = {}", 3, plus_two(3));

    let plus_three = |x:&mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);
}

fn is_even(x: u32) -> bool {
    x%2 == 0
}

// generators example
// function that returns functions
fn greater_than(limit: u32)
    -> impl Fn(u32) -> bool
{
    move |y| y > limit
}

fn higher_order_functions() {

    // sum of all even squares < 500

    let limit = 500;
    let mut sum = 0;

    // let above_limit = |y| y > limit;
    let above_limit = greater_than(limit);
    for i in 0.. {
        let isq = i*i;

        // if isq > limit {
        if above_limit(isq) {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }

    println!("loop sum = {}", sum);

    let sum2 = (0..)
        .map(|x| x*x)
        .take_while(|&x| x < limit)
        .filter(|x:&u32| is_even(*x))
        .fold(0, |sum, x| sum + x);

    println!("hof sum = {}", sum2);
}

fn main() {
    // println!("Hello, world!");
    // functions();
    // methods();
    // closures();
    higher_order_functions();
}
