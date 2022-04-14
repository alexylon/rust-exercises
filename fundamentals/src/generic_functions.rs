fn f1<T>(a: T, _b: T) -> T { a }

pub fn run_f1() {
    let _a = f1(12u8, 13u8);
    let _b = f1(12i64, 13i64);
    // compilation errors:
    // let _c = f(12i16, 13u16);
    // let _d: i32 = f(12i16, 13i16);
    println!("{} {}", _a, _b);
}

fn swap<T1, T2>(a: T1, b: T2) -> (T2, T1) { (b, a) }

pub fn run_swap() {
    let x = swap(3i16, 4u16);
    let y = swap(5f32, true);
    print!("{:?} {:?}", x, y);
}

#[derive(Debug)]
struct S<T1, T2> {
    c: char,
    n1: T1,
    n2: T1,
    n3: T2,
}

pub fn generic_struct() {
    let s = S { c: 'a', n1: 34, n2: 782, n3: 0.02 };
    println!("{:?}", s);

    // Also for structs, the type parameter concretizations can be made explicit:
    let _s = S::<u16, f32> { c: 'a', n1: 34, n2: 782, n3: 0.02 };
}

#[derive(Debug)]
struct SE<T1, T2> (char, T1, T1, T2);

pub fn generic_tuple_struct() {
    let se = SE('a', 34, 782, 0.02);
    println!("{:?}", se);

    // Also for structs, the type parameter concretizations can be made explicit:
    let _se = SE::<u16, f32>('a', 34, 782, 0.02);
}

// In Rust, even enums can be generic:
enum Result1<SuccessCode, FailureCode> {
    Success(SuccessCode),
    Failure(FailureCode, char),
    Uncertainty,
}

pub fn generic_enum() {
    let mut _res = Result1::Success::<u32, u16>(12u32);
    _res = Result1::Uncertainty;
    _res = Result1::Failure(0u16, 'd');
}


// The Option<T> Standard Enum
pub fn test_option() {
    let mut v = vec![11, 22, 33];
    for _ in 0..5 {
        let item: Option<i32> = v.pop();
        match item {
            Some(number) => print!("{}, ", number),
            None => print!("#, "),
        }
    }
}

// Enum Standard Utility Functions
pub fn test_utility_option() {
    let mut a = Some(12);
    print!("{} {}; ", a.is_some(), a.is_none());
    a = None;
    print!("{} {}", a.is_some(), a.is_none());
}

// The Result<T, E> Standard Enum
fn divide1(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0. {
        Err(format!("Division by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

pub fn test_divide1() {
    print!("{:?}, {:?}",
           divide1(8., 2.), divide1(8., 0.));

    let r1 = divide1(8., 2.);
    let r2 = divide1(8., 0.);

    // Enum Standard Utility Functions
    println!("{} {}", r1.is_ok(), r1.is_err());
    println!("{} {}", r2.is_ok(), r2.is_err());
    println!("{}", r1.unwrap());
    // This one panics:
    println!("{}", r2.unwrap());
}

// This is a better version for production:
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0. {
        Err(format!("Division by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

fn show_divide(num: f64, den: f64) {
    match divide(num, den) {
        Ok(val) => println!("{} / {} = {}", num, den, val),
        Err(msg) => println!("Cannot divide {} by {}: {}",
                             num, den, msg),
    }
}

pub fn test_divide() {
    show_divide(8., 2.);
    show_divide(8., 0.);
}