#![feature(custom_inner_attributes)]

use libfranca::loops::mkterse;

fn main() {
    let i = mkterse("fn a() { let a = &mut 5; let mut b = 10; }".into());
    println!("{i}");
}
