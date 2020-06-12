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

fn main() {
    // println!("Hello, world!");
    vectors();
}
