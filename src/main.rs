// 関数

fn plus_one(x: i32) -> i32 {
    return x + 1;
}

fn plus_one2(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = plus_one(5);
    let x2 = plus_one2(5);
    println!("The value of x is: {}", x);
    println!("The value of x is: {}", x2);
}

// 文：値を返さない。何らかの動作をして値を返さない命令
// 式：値を返す
// Rustにおける「；」は式を文に変える演算子
// Rustは式志向言語
