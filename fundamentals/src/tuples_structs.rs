pub fn _tuples() {
    let _data: (i32, f64, char) = (10000000, 183.19, 'Q');

    let mut data = (10000000, 183.19, 'Q');
    data.0 = -5;
    data.2 = 'x';
    println!("{}, {}, {}", data.0, data.1, data.2);

    let array = [12, 13, 14];
    let tuple = (42, true, "string");
    let i = 0;
    println!("{}", array[i]);
    // tuples cannot be accessed by a variable index
    // println!("{}", tuple.i);
    println!("{}", tuple.0);

    println!("{:?}", tuple);
}

pub fn _structs() {
    struct SomeData {
        integer: i32,
        fractional: f32,
        character: char,
        five_bytes: [u8; 5],
    }
    let data = SomeData {
        integer: 10_000_000,
        fractional: 183.19,
        character: 'Q',
        five_bytes: [9, 0, 250, 60, 200],
    };
    print!("{}, {}, {}, {}",
           data.five_bytes[3], data.integer,
           data.fractional, data.character);

    // mutable
    struct SomeData2 {
        integer: i32,
        fractional: f32,
    }
    let mut data = SomeData2 {
        integer: 10,
        fractional: 183.19,
    };
    data.fractional = 8.2;
    print!("{}, {}", data.fractional, data.integer);
}

pub fn _tuple_structs() {
    // a struct without names
    struct SomeData(
        i32,
        f32,
        char,
        [u8; 5],
    );
    let data = SomeData(
        10_000_000,
        183.19,
        'Q',
        [9, 0, 250, 60, 200],
    );
    print!("{}, {}, {}, {}",
           data.2, data.0, data.1, data.3[2]);
}

// # Change a Variable of the Caller

// This is working, but at a computational cost,
// which can be avoided. We have 3 copies here
fn _double_negatives1(mut a: [i32; 10]) -> [i32; 10] {
    for i in 0..10 {
        if a[i] < 0 { a[i] *= 2; }
    }
    a
}

pub fn _print_double_negatives1() {
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    arr = _double_negatives1(arr);
    print!("{:?}", arr);
}

// Passing Arguments by Reference. This is more efficient.
// The "&" symbol means “the (memory) address of the object”, and
// the "*" symbol means “the object that is present at this (memory) address”.
fn _double_negatives2(a: &mut [i32; 10]) {
    for i in 0..10 {
        if (*a)[i] < 0 {
            (*a)[i] *= 2;
            // this may be simplified by omitting the asterisk:
            // if a[i] < 0 { a[i] *= 2; }
            // the compiler attempts to take directly the variable reference,
            // but as it is not allowed to take directly a reference in this way,
            // the referred object is taken.
        }
    }
}

pub fn _print_double_negatives2() {
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    _double_negatives2(&mut arr);
    print!("{:?}", arr);
}

// Using references, you can do some virtuosity:
pub fn _references() {
    let a = &&&7;
    print!("{} {} {} {}", ***a, **a, *a, a);
}

// Mutability of References
pub fn _mutable_references() {
    let mut a: i32 = 10;
    let mut b: i32 = 20;
    let mut p: &mut i32 = &mut a; // line 3
    print!("{} ", *p);
    *p += 1; // line 5
    print!("{} ", *p);
    p = &mut b; // line 7
    print!("{} ", *p);
    *p += 1; // line 9
    print!("{} ", *p);
}

// However, it is better to insist that at line 3, the first mut word has a different meaning
// that the two others.
// The first mut word means that the p variable can be changed, meaning it can be
// re-assigned, making it to refer to another object, like it has been done at line 7. Without
// such mut word, p would refer always to the same object.
// The second and third mut words mean that the type of p allows it to change the value
// of the referred object. Without such mut words, p wouldn't be able to change the value of
// its referred object.