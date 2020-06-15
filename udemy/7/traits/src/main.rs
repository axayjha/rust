
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]

use std::fmt::Debug;

trait Animal {
    fn create(name: &'static str) -> Self
    where Self: Sized;
    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }

}


struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}


impl Animal for Human {
    fn create(name: &'static str)-> Human {
        Human {name: name}
    }
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says Hello", self.name());
    }
}

impl Animal for Cat {
    fn create(name: &'static str)-> Cat {
        Cat {name: name}
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result: i32 = 0;
        for x in self { result += *x; }
        return result;
    }
}

fn traits() {
    // let h = Human::create("John");
    let h: Cat = Animal::create("John");
    h.talk();

    let c = Cat {name: "Misty"};
    c.talk();


    let a = vec![1, 2, 3];
    println!("sum = {}", a.sum()); // doesn't exist
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Square {
    side: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}


impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 3 diff syntax:

// fn print_info(shape: impl Shape + Debug) {
// fn print_info<T: Shape+Debug>(shape: T, shape2: T) {
fn print_info<T>(shape: T)
    where T: Shape+Debug {
    println!("{:?}", shape);
    println!("the area is {}", shape.area())
}


fn trait_parameters() {
    let c = Circle { radius: 2.0};
    print_info(c);
}

struct Person {
    name: String
}

impl Person {
    // fn new(name: &str) -> Person{
    //     Person { name: name.to_string()
    //     }
    // }

    // fn new<S: Into<String>>(name: S) -> Person {
    fn new<S>(name: S) -> Person
        where S: Into<String> {
        Person { name: name.into() }
    }
}

fn into_example(){
    let john = Person::new("John");
    let name: String = "Jane".to_string();

    // let jane = Person::new(name.as_ref());
}

struct Creature {
    name: String
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} enters the game", name);
        Creature { name: name.into() }
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} is dead", self.name);
    }
}

fn drop_example() {
    // let goblin = Creature::new("Jeff");
    // println!("game proceeds");

    let mut clever: Creature;
    {
        let goblin = Creature::new("Jeff");
        println!("game proceeds");
        clever = goblin;
        println!("end of scope");
    }

}

use std::ops::{Add, AddAssign, Neg};
use std::cmp::PartialEq;
use std::process::Output;

#[derive(Debug)]
struct Complex<T> {
    re: T,
    im: T
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> { re, im }
    }
}

impl<T> Add for Complex<T>
    where T: Add<Output = T>{
    type Output = Complex<T>;
    fn add(self, rhs: Self) -> Self::Output {
        // unimplemented!()
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}


// for +=
impl<T> AddAssign for Complex<T>
where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}


// for -a
impl<T> Neg for Complex<T>
    where T: Neg<Output = T> {
    type Output = Complex<T>;
    fn neg(self)-> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im
        }
    }
}

// equality -> partial, full

impl<T> PartialEq for Complex<T>
where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.re == other.re && self.im == other.im
    }
}

impl<T: Eq> Eq for Complex<T> where T: Eq{}

// can also #[derive(Eq, PartialEq)] over struct
//

fn operator_overloading() {
    let mut a = Complex::new(1.0, 2.0);
    let mut b = Complex::new(3.0, 4.0);
    // println!("{:?}", a);
    // println!("{:?}", a+b);
    // a += b;
    // println!("{:?}", a);
    println!("{}", a==a);

}

trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self)-> String {
        format!("String: {}", *self)
    }
}

// static dispatch
// resolves type at compile time
fn print_it<T: Printable>(z: T) {
    println!("{}", z.format());
} // monomorphisation

// dynamic dispatch
// resolves type at run time
fn print_it_too(z: &Printable) {
    println!("{}", z.format());
}

fn static_dispatch() {
    let a  = 123;
    let b = "hello".to_string();

    // println!("{}", a.format());
    // println!("{}", b.format());
    print_it(a);
    print_it(b);
}


fn dynamic_dispatch() {
    let shapes: [&Shape; 4] = [
        &Circle{ radius: 1.0},
        &Square{ side: 3.0},
        &Circle{ radius: 2.0},
        &Square{ side: 4.0}
    ];

    for (i, shape) in shapes.iter().enumerate() {
        println!("Shape #{} has area {}", i, shape.area());
    }
}

enum CreatureEnum {
    Human(Human),
    Cat(Cat)
}

fn vectors_of_diff_types() {
    let mut creatures = Vec::new();
    // creatures.push(Human{name: "John"});
    // creatures.push(Cat {name: "fluffy"}); // wont work

    creatures.push(CreatureEnum::Human(Human {name: "Jeff"}));
    creatures.push(CreatureEnum::Cat(Cat {name: "Fluffy"}));

    for c in creatures {
        match c {
            CreatureEnum::Human(h) => h.talk(),
            CreatureEnum::Cat(c) => c.talk()

        }
    }

    let mut animals: Vec<Box<Animal>> = Vec::new();
    // wont work because compiler doesnt know the size
    //creatures.push(CreatureEnum::Human(Human {name: "Jeff"}));

    animals.push(Box::new(Human {name: "Jeff"}));
    animals.push(Box::new(Cat {name: "Fluffy"}));

    for a in animals.iter() {
        a.talk();
    }

}

fn main() {
    // println!("Hello, world!");
    // traits();
    // trait_parameters()
    // into_example();
    // drop_example();
    // operator_overloading();
    // static_dispatch();
    // dynamic_dispatch();
    vectors_of_diff_types();
}
