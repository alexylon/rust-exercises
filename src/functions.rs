// Early exit

fn _f1(x: f64) -> f64 {
    if x <= 0. { return 0.; }
    x + 3.
}

fn _f2(x: f64) -> f64 {
// This is considered bad style
    if x <= 0. { return 0.; }
    return x + 3.;
}

fn _f3(x: f64) -> f64 {
// This is better
    if x <= 0. { 0. } else { x + 3. }
}

fn _f4(x: i32) {
    // empty tuple as a return value type
    if x <= 0 { return; }
    if x == 4 { return (); }
    if x == 7 { return {}; }
    print!("{}", x);
}

// Returning several values
fn _divide(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}

// Or you can return an enum, a struct, a tuple struct, an array, or a vector:
enum _E { _E1, E2 }

struct _S {
    a: i32,
    _b: bool,
}

struct _TS(f64, char);

fn _f10() -> _E { _E::E2 }

fn _f20() -> _S { _S { a: 49, _b: true } }

fn _f30() -> _TS { _TS(4.7, 'w') }

fn _f40() -> [i16; 4] { [7, -2, 0, 19] }

fn _f50() -> Vec<i64> { vec![12000] }

pub fn _print() {
    print!("{} ", match _f10() { _E::_E1 => 1, _ => -1 });
    print!("{} ", _f20().a);
    print!("{} ", _f30().0);
    print!("{} ", _f40()[0]);
    print!("{} ", _f50()[0]);
}

// This will print: "-1 49 4.7 7 12000".
// Let’s explain these five numbers.
// The invocation of f10 in the first print! invocation returns the enumeration E2, which
// is tried to match with E1, and, as it doesn't match, the catch-all case is taken, and so -1 is
// printed.
// The invocation of f20 returns a struct object containing the fields a and b, and from it
// the a field is extracted.
// The invocation of f30 returns a tuple-struct containing two fields, and from it the first
// field is extracted.
// The invocation of f40 returns an array containing four items, and from it the first item
// is extracted.
// The invocation of f50 returns a vector containing just one item, and from it the first
// and only item is extracted.
