pub fn ownership_example1() {
    // Move semantics is used, by default, by dynamic strings, boxes,
    // any collection (including vectors), enums, structs, and tuple-structs.
    let string1: String = "Hello".to_owned();
    let string2 = string1.clone(); // copy semantics
    let string3 = &string1;
    let string4 = string1; // move semantics (default)
    // Not allowed as string1 is already moved and as such - inaccessible:
    // let string5 = string1;

    // Copy semantics is used by numbers, Booleans,
    // static strings, arrays, tuples, and references.
    let str1: &str = "World";
    let str2 = str1;
    let str3 = str1;

    println!("{}", format!("{} {}", string4, str3));
}

// For primitive numbers, static strings, and references, Rust does not
// use move semantics. For these data types, Rust uses copy semantics.
pub fn ownership_example2() {
    let i1 = 123;
    let _i2 = i1;
    let s1 = "abc";
    let _s2 = s1;
    let r1 = &i1;
    let _r2 = r1;

    print!("{} {} {}", i1, s1, r1);
}

pub fn ownership_example3() {
    let a1 = 123;
    let b1 = a1.clone();
    let c1 = b1;
    print!("{} {} {}", a1, b1, c1);
    let a2 = Vec::<bool>::new();
    let b2 = a2.clone();
    let c2 = b2;
    print!(" {:?}", a2);
// ILLEGAL: print!("{:?}", b2);
    print!(" {:?}", c2);
    let a3 = std::fs::File::open(".").unwrap();
// ILLEGAL: let b3 = a3.clone();
    let c3 = a3;
// ILLEGAL: print!("{:?}", a3);
    print!(" {:?}", c3);
}

// Regarding the ability to be copied, there are three kinds of objects,
// above is an example of all these cases:

// 1. The objects, like primitive numbers, that implement both Copy and
// Clone, are copyable (and also cloneable). They implement copy
// semantics, and they can also be cloned explicitly.

// 2. The objects, like collections, that implement Clone, but don’t
// implement Copy, are cloneable but noncopyable. They implement
// move semantics, but they can be cloned explicitly.

// 3. The objects, like file handles, that implement neither Copy nor Clone,
// are noncloneable (and also noncopyable). They implement move
// semantics, and they cannot be cloned.

// • No object can implement Copy but not Clone. This means that no
// object is copyable but not cloneable. This is because such object
// would be copied implicitly but not explicitly, and this is pointless.


// As said before, enums, structs, and tuple structs, by default, do not implement either the
// Copy trait or the Clone trait, so they are non-cloneable. Though, you may implement the
// single Clone trait for each of them, or both the Clone trait and the Copy trait:

struct S1 {}

impl Clone for S1 {
    fn clone(&self) -> Self { Self {} }
}

// OR just
#[derive(Clone)]
struct S2 {}

fn ownership_example4() {
    let s1 = S1 {};
    let a1 = s1.clone();
    let b1 = s1;
    // Not allowed, because Copy not implemented:
    // let c = s;

    let s2 = S2 {};
    let a2 = s2.clone();
    let b2 = s2;
}

struct S3 {}

// In order to implement Copy trait, first Clone should be implemented
impl Clone for S3 {
    fn clone(&self) -> Self { Self {} }
}

impl Copy for S3 {}

// OR just
#[derive(Clone, Copy)]
struct S4 {}

fn ownership_example5() {
    let s3 = S3 {};
    let _ = s3.clone();
    let _a3 = s3;
    let _b3 = s3;

    let s4 = S3 {};
    let _ = s4.clone();
    let _a4 = s4;
    let _b4 = s4;
}