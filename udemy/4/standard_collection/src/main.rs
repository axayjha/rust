#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]

/*

Vec<T> - vector

VecDequq<T> - dequq

LinkedList<T> - list

BinaryHeap<T> where T: Ord - priority_queue

HashMap<K, V> where K:: Eq+Hash - unordered_map

BTreeMap<K, V> where K: Ord - map

HashSet<T> - unordered_set

BTreeSet<T> - set

*/

fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);

    // usize isize

    //let idx: i32 = 0;
    //println!("a[0] = {}", a[idx]); //error

    // let idx: usize = 111;
    // a[idx] = 312;
    // println!("a[0] = {}", a[idx]); // panic

    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error, no such element")
    }

    for x in &a {
        println!("{}", x);
    }

    a.push(44);
    println!("{:?}", a);
    a.pop();
    println!("{:?}", a);

    let last_elem = a.pop(); //option
    println!("last elem = {:?}", last_elem);

    // let Some(x) = a.pop(); // not always true
    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

use std::collections::HashMap;

fn hashmaps() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} sides", shapes["square".into()]);
    //println!("a square has {} sides", shapes["rectangle".into()]); // -> error

    for (key, value) in &shapes{
        println!("{}: {}", key, value);
    }

    shapes.insert("square".into(), 5); // to override
    println!("{:?}", shapes);

    shapes.entry("circle".into()).or_insert(1);
    println!("{:?}", shapes);
    let a = shapes.entry("circle".into()).or_insert(2);
    println!("{:?}", a);
    *a = 4;
    println!("{:?}", shapes);

}

use std::thread;
use std::time;
use std::collections::HashSet;

fn hashsets() {
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    greeks.insert("delta");
    println!("{:?}", greeks);

    let added_vega = greeks.insert("vega");
    println!("{}", added_vega);
    if added_vega {
        println!("we added vega");
    }

    if !greeks.contains("kappa") {
        println!("we dont have kappa");
    }

    let removed = greeks.remove("delta");

    if removed {
        println!("we removed delta");
    }
    println!("{:?}", greeks);

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();
    // subset
    println!("is {:?} a subset of {:?} ?: {}", _2_8, _1_10, _2_8.is_subset(&_1_10));

    // disjoint
    println!("are {:?} and {:?} disjoint ?: {}", _1_5, _6_10, _1_5.is_disjoint(&_6_10));

    // union
    println!("union of {:?} and {:?}  : {:?}", _2_8, _6_10, _2_8.union(&_6_10));

    // intersection
    println!("intersection of {:?} and {:?}  : {:?}", _2_8, _6_10, _2_8.intersection(&_6_10));


}

fn main() {
    // println!("Hello, world!");
    // vectors();
    // hashmaps();
    hashsets();
}
