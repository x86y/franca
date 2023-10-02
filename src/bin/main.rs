#![feature(custom_inner_attributes)]

use libfranca::transformer::mkterse;

fn main() {
    let i = mkterse("fn a() { let mut a = 5; let mut b = 10; }".into());
    println!("{i}");
}
