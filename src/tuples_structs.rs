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