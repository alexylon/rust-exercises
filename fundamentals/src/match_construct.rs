pub fn _match_construct() {
    match "value" {
        "val" => print!("value "),
        _ => print!("other "),
    }
    match 3 {
        3 => print!("three "),
        4 => print!("four "),
        5 => print!("five "),
        _ => print!("other "),
    }
    match '.' {
        ':' => print!("colon "),
        '.' => print!("point "),
        _ => print!("other "),
    }
}

pub fn _match_expressions() {
    enum CardinalPoint { _North, South, _West, _East }
    let direction = CardinalPoint::South;
    print!("{}", match direction {
        CardinalPoint::_North => 'N',
        CardinalPoint::South => 'S',
        _ => '*',
    });
}

pub fn _match_guards() {
    for n in -2..5 {
        println!("{} is {}.", n, match n {
            0 => "zero",
            1 => "one",
            _ if n < 0 => "negative",
            _ => "plural",
        });
    }
}