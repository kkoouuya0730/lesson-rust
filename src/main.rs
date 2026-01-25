// enum
fn main() {
    let quarter = Coin::Quarter(UsState::Alabama);
    let penny = Coin::Penny;
    let x = value_in_coins(quarter);
    println!("x is {}", x);
    println!("penny is {:#?}", penny);
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_coins(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
