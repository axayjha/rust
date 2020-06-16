#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]

fn ownership() {
    // v owns the memory that's allocated
    // variable v is on the stack
    // but the data is on heap
    let v = vec![1, 2, 3];

    // let v2 = v;

    // println!("{:?}", v);

    let u = Box::new(1); // i32 copy and not move
    // Box puts value on the heap, and returns smart pointer
    // because its easy to copy a simple int
    let u2 = u; // u2 takes the ownership of
    // the value present on heap
    // println!("u = {}", *u); // can't access u anymore


    // let foo = |v: Vec<i32>| ();
    // foo(v);

    let print_vector = |x: Vec<i32> | -> Vec<i32>
        {
            println!("{:?}", x);
            x
        };
    let vv = print_vector(v); // can do because print_vector returns x after using it
}


fn borrowing() {
    let print_vector = |x: &Vec<i32> |
        {
            println!("x[0] = {}", x[0]);
            // x.push(123); // can't do yet
        };

    let v = vec![3, 2, 1];
    print_vector(&v); // &-> borrowing the vector for a while

    println!("v[0] = {}", v[0]);

    let mut a = 40;
    {
        let b = &mut a; // mutable ref (borrow) to a
        *b += 2;
    }
    println!("a = {}", a);

    let mut z = vec![3, 2, 1];

    for i in &z {
        println!("i = {}", i);
        // z.push(5) //can't do
    }

}
/* normally - without lifetime
struct Person {
    name: String
}

struct Company {
    name: String,
    ceo: &Person
}
*/

struct Person {
    name: String
}

impl Person {
    // fn get_ref_name(&self) -> &String {
    // if you do above, you actually get below:
    // Lifetime elision
    fn get_ref_name<'a>(&'a self) -> &'a String {
        &self.name
    }
}

struct Company<'z> {
    name: String,
    ceo: &'z Person
}
// basically telling company and person have same lifetime

fn lifetime() {
    // &'static str;
    // here static is the lifetime: lives as long as program
    // lifetime is defined with a ' (apostrophe) and then a name

    let boss = Person{name: String::from("Elon Musk")};
    let tesla = Company {name: String::from("Tesla"), ceo: &boss};
    // wont compile: no lifetime parameter
    // as Company Tesla is dependent on boss, means the
    // lifetime of boss matters, as rust is against invalid references
    // rust wants a guarantee that as long as the company exists, the Person
    // it is referring to also exists

    /*
    //below wont work because p doesnt live long enough
    //lifetime is shorter than z

    let mut z: &String;
    {
        let p = Person { name: String::from("John") };
        z = p.get_ref_name();
    }


     */
}

/*

struct Person1 {
    // wrong - no lifetime elision here
    name: &str
}
*/

// correct
struct Person2<'a> {
    name: &'a str
}
/*
// Wrong
impl Person2{
    fn talk(&self) {
        println!("Hi, my name is {}.", self.name);
    }
}
*/

impl<'a> Person2<'a> {
    fn talk(&self) {
        println!("Hi, my name is {}.", self.name);
    }
}

fn lifetime_in_structure_implementation() {
    // wont work- requires lifetime specifier
    let person = Person2{ name: "Akshay"};
    person.talk();
}

use std::rc::Rc;

struct Person3 {
    name: String
}

impl Person3 {
    fn new(name: String) -> Person3 {
        Person3 { name: name }
    }

    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}


struct Person4{
    name: Rc<String>
}

impl Person4 {
    fn new(name: Rc<String>) -> Person4 {
        Person4 { name: name }
    }

    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}


fn reference_counted_variables() {
    let name = Rc::new("John".to_string());
    let person = Person4::new(name.clone());
    // whenever you want to move a var, you clone
    // clone increments reference count
    // and gives access without ordinary move semantics

    person.greet();
    // can't do below with Person3
    println!("Name = {}", name);

    let name1 = Rc::new("John".to_string());
    println!("name = {}, name has {} strong pointers", name1, Rc::strong_count(&name1));
    {
        let person1 = Person4::new(name1.clone());
        println!("name = {}, name has {} strong pointers", name1, Rc::strong_count(&name1));

        person1.greet();

    }
    println!("Name = {}", name);
    println!("name = {}, name has {} strong pointers", name1, Rc::strong_count(&name1));


}

use std::thread;
use std::sync::{Arc, Mutex};

struct Person5{
    name: Arc<String>
}

impl Person5 {
    fn new(name: Arc<String>) -> Person5 {
        Person5 { name: name }
    }

    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}


fn atomic_reference_counter_variables() {
    // Rc is not a threadsafe way of passing
    // around reference counted object
    /* // Below wont work
    let name = Rc::new("John".to_string());
    let person = Person4::new(name.clone());

    let t = thread::spawn(move || {
        person.greet();
    });

    println!("Name = {}", name);

    t.join().unwrap();

     */

    // Arc is thread safe

    let name = Arc::new("John".to_string());
    let person = Person5::new(name.clone());

    let t = thread::spawn(move || {
        person.greet();
    });

    println!("Name = {}", name);

    t.join().unwrap();
}


struct Person6 {

    name: Arc<String>,
    state: Arc<Mutex<String>>

}

impl Person6 {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person6 {
        Person6 { name: name, state: state }
    }

    fn greet(&self) {
        // Cannot borrow immutable local variable `self.state` as mutable
        let mut state = self.state.lock().unwrap();
        // won't work without mutex, need to save from concurrent access
        // self.state.clear();
        // self.state.push_str(("excited"));

        state.clear();
        state.push_str("excited");

        println!("Hi, my name is {} and I am {}", self.name, state.as_str());

    }
}
// Using a mutex for thread-safe mutability
fn mutex_demo() {
    let name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));

    let person = Person6::new(name.clone(), state.clone());
    let t = thread::spawn(move || {
        person.greet();
    });
    println!("Name = {}, state = {}", name, state.lock().unwrap().as_str());
    t.join().unwrap();
}

fn main() {
    // println!("Hello, world!");
    // ownership();
    // borrowing();
    // lifetime();
    // lifetime_in_structure_implementation();
    // reference_counted_variables();
    // atomic_reference_counter_variables();
    mutex_demo();
}
