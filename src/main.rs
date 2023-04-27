#[derive(Debug)]
enum IpAddress {
    // We attach data to each variant of the enum directly, so there is no need for an extra struct.
    // Here, it’s also easier to see another detail of how enums work: the name of each enum variant
    // that we define also becomes a function that constructs an instance of the enum.
    // That is, IpAddr::V4() is a function call that takes a String argument and returns an instance
    // of the IpAddr type. We automatically get this constructor function defined as a result of
    // defining the enum.
    // Reference: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#enum-values
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let home = IpAddress::V4(127, 0, 0, 1);
    let loopback = IpAddress::V6(String::from("::1"));

    println!("home = {:?}, loopback = {:?}", home, loopback);

    value_in_cents(Coin::Quarter(UsState::Alaska));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from: {:?}", state);
            25
        }
    }
}