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


fn main() {
    // println!("Hello, world!");
    // ownership();
    borrowing();
}
