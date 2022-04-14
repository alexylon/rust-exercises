pub fn _arrays() {
    let arr = [1, 2, 3, 4];

    println!("{:?}", arr);

    let mut x = ["This", "is", "a", "sentence"];
    x[2] = "a nice";
    print!("{} {} {} {}.", x[0], x[1], x[2], x[3]);


    // Arrays of Specified Size
    let mut x = [4.; 5000];
    x[2000] = 3.14;
    print!("{}, {}", x[1000], x[2000]);
}

pub fn _vectors() {
    let mut vector = vec![1, 2, 3, 4];
    vector.push(5);
    println!("{:?}", vector);

    let mut x = vec!["This", "is", "a", "sentence"];
    x.insert(1, "line");
    x.insert(2, "contains");
    x.remove(3);
    x.push("about Rust");
    x.pop();
    for i in 0..x.len() { print!("{} ", x[i]); }
}