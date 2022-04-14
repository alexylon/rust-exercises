pub fn borrow_lifetime1() {
    let a = 12;
    let mut b = &a; // start of first borrow of a to b
    let c = &a; // start of borrow of a to c
    println!("{} {} ", b, c); // end of borrows of a to b and c
    b = &23; // start of second borrow of a to b
    println!("{}", b); // end of second borrow of a to b
}

pub fn borrow_lifetime2() {
    let n1 = 11;
    let result;
    {
        let n2 = 12;
        result = {
            let _m1 = &n1;
            let _m2 = &n2;
            _m1
        }
    }
    print!("{}", *result);
}

// let’s transform the previous program, by encapsulating in a function
// the code in the innermost block:
pub fn borrow_lifetime3() {
    let n1 = 11;
    let result;
    {
        let n2 = 12;
        fn func<'a>(_m1: &'a i32, _m2: &i32) -> &'a i32 {
            _m1
        }
        result = func(&n1, &n2);
    }
    print!("{}", *result);
}

trait Tr1 {
    fn f<'a>(flag: bool, b: &'a i32, c: (char, &'a i32))
             -> (&'a i32, f64, &'static i32);
}
// The use of the a lifetime specifier means: the first field of the return value borrows
// the same object already borrowed by the b argument and by the second field of the c
// argument, so it must live for less time than such objects.
// The use of the static lifetime specifier means: the third field of the return value refers
// to a static object, so it can live for any time, even as long as the whole process.

trait Tr2 {
    fn f<'a>(flag: bool, b: &'a i32, c: (char, &i32))
             -> (&'static i32, f64, &'a i32);
}

// In this case, the first field of the return value has a static lifetime, meaning that it is
// not constrained to live less than some other object. Instead, the third field has the same
// lifetime specifier as the b argument, meaning that it should live less than it, as it borrows
// the same object. The reference in the type of the c argument is not annotated, as the
// object it refers is not borrowed by any reference in the return value.

trait Tr3 {
    fn f<'a, 'b, T1, T2>(flag: bool, b: &'a T1,
                         c: (char, &'b i32)) -> (&'b i32, f64, &'a T2);
}

// This generic function has two lifetime parameters, and also two type parameters.
// The lifetime parameter a specifies that the third field of the return value borrows the
// object already borrowed by the b argument, while the lifetime parameter b specifies that
// the first field of the return value borrows the object already borrowed by the second field
// of the c argument. Moreover, the function has two type parameters, T1 and T2, used as
// usual.

