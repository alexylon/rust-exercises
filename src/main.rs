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
mod strings;
mod vector_update;

mod i18n { pub mod i18n_test; }

use crate::i18n::i18n_test;
use crate::ownership::ownership_example1;

fn main() {
    i18n_test::i18n_test("it", "Marcus");
}

// fn main() {
//     vector_update::uniques();
// }

