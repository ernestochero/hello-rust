mod count_words;
mod data_science;

use std::mem;
use ferris_says::say;
use walkdir::WalkDir;
use std::io::{stdout, BufWriter};
use std::slice::Iter;
use rayon::prelude::*;


fn sum(a: i32, b: i32) {
    println!("Hey sum result {}", a + b);
}

fn dir_example() {
    for ent in WalkDir::new("./src") {
        if let Ok(ent) = ent {
            let path = ent.path();
            if path.is_dir() {
                println!("{:?} is a directory", path)
            } else {
                println!("{:?} is a file", path)
            }
        }
    }
}

fn count_walk_dir(root: &str) {
    let count = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .par_bridge()
        .filter(|e| e.path().is_file())
        .count();
    println!("Found {} files", count);
}

fn ferris_test() {
    let stdout = stdout();
    let message = String::from("Hello fellow!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

trait Animal {
    fn new(name: &'static str, naked: &'static bool) -> Self;

    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

#[derive(Debug)]
struct Person{
    name: String,
    age: u8
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32
}

struct Sheep {
    name: &'static str,
    naked: &'static bool
}
impl Animal for Sheep {
    fn new(name: &'static str, naked: &'static bool) -> Self {
        Sheep { name, naked}
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if *self.naked {
            "!!!!?"
        }
        else {
            "!!!!"
        }
    }

    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn misc() {
    sum(10,12);

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // let ys: [i32; 500] = [0;500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    println!("number of elements in array: {}", xs.len());

    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // create struct with field unit shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3 , y: 0.4};
    println!("point coordinates: ({}, {})", point.x, point.y);

    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // ferris says
    ferris_test();

    // Sheep
    let sheep = Sheep::new("Lola", &false);
    sheep.talk();
    // dir_example();
    count_walk_dir(".");

    // words count
    println!("Total lines are: {}",  count_words::count_lines());

    // reduce
/*    let l: Iter<i32> = [1, 2, 3, 4].iter();
    let s: Option<&i32> = l.reduce(|a, b | a + b );
    s.map_or((), |ss| println!("{}", ss))*/
}

fn main() {
    println!("Hello, world!");
    // data_science::test_array();
    // data_science::test_ndarray_rand();
    data_science::test_visualize();
}

