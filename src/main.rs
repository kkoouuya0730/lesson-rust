// データ型

use std::io;

fn main() {
    let c = 'z';
    let z = "zzzz";
    let heart_eyed_cat = '😻';

    println!("{}", c);
    println!("{}", z);
    println!("{}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("値の読み込みに失敗しました");

    let index: usize = index
        .trim()
        .parse()
        .expect("入力された値は数字ではありません");

    let element = a[index];

    println!(
        "The value of the element at index {} is; {}",
        index, element
    );
}
