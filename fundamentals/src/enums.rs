pub fn _enums() {
    enum Continent {
        _Europe,
        _Asia,
        _Africa,
        _America,
        Oceania,
    }
    let continent = Continent::_Asia;
    let _contin = Continent::Oceania;

    match continent {
        Continent::_Europe => print!("E"),
        Continent::_Asia => print!("As"),
        Continent::_Africa => print!("Af"),
        Continent::_America => print!("Am"),
        Continent::Oceania => print!("O"),
    }
}