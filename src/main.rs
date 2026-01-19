// シャドーイング
fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x); // 12
    }

    println!("The value of x is: {}", x); // 6
}
