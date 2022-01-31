// Six ways to invoke a closure:
pub fn closures1() {
    let factor = 2;
    let multiply = |a| a * factor;
    print!("{}", multiply(13));
    let multiply_ref = &multiply;

    print!(
        " {} {} {} {} {}",
        (*multiply_ref)(13),
        multiply_ref(13),
        // Or directly without using the variable:
        (|a| a * factor)(13),
        (|a: i32| a * factor)(13),
        |a| -> i32 { a * factor }(13));
}

pub fn closures2() {
    let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];

    // The two following statements are equivalent:
    arr.sort();
    arr.sort_by(|a, b| a.cmp(b));
    println!("{:?}", arr);

    // Therefore, to obtain the inverted order, you can use the
    // following statement:
    arr.sort_by(|a, b| b.cmp(a));
    println!("{:?}", arr);
}