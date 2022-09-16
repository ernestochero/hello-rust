use std::mem;

// https://dev.to/dillendev/memory-alignment-3j7e
struct Member {
    active: bool,
    age: i32,
    admin: bool,
}

pub fn test_memory() -> () {
    println!("bool: {} bytes", mem::size_of::<bool>());
    println!("i32: {} bytes", mem::size_of::<i32>());
    println!("Member: {} bytes", mem::size_of::<Member>());
}