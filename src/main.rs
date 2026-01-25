// enum
fn main() {
    let five = None;
    let seven = Some(7);
    let six = plus_one(five);
    let none = plus_one(None);
    let y = match seven {
        Some(x) => x,
        None => 0,
    };

    println!("five is {:?}", five);
    println!("five is {:?}", seven);
    println!("five is {:?}", six);
    println!("five is {:?}", none);
    println!("five is {:?}", y);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
