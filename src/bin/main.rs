#![feature(custom_inner_attributes)]

use libfranca::loops::{terser_loops, unfolder};

fn main() {
    unfolder();
    let i = terser_loops("fn main() { while true {} }".into());
    println!("{i}");
}
