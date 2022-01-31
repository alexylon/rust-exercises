#![allow(dead_code)]
#![allow(unused_must_use)]

mod array_vector;
mod enums;
mod match_construct;
mod tuples_structs;
mod functions;
mod generic_functions;
mod boxing_unboxing;
mod ownership;
mod io;
mod borrow_lifetimes;
mod closures;

use crate::ownership::ownership_example1;

fn main() {
    closures::closures2();
}
