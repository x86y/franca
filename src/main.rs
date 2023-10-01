#![feature(custom_inner_attributes)]

mod lib;
use lib::{unfolder, terser_loops};

fn main() {
    unfolder();
    terser_loops("".into());
}
