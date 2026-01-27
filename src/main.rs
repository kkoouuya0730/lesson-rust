use std::vec;

// ベクタ
fn main() {
    // let data = "initial contents";

    // let s = data.to_string();

    // let s = "initial contents".to_string();

    // let mut s = String::from("foo");
    // s.push_str("bar");
    // s.push('b');
    // println!("The value of s is: {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // 文字列に添え字でアクセスできない
    let s3 = String::from("hello");
    // let h = s3[0];

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // let s = &hello[0..1];

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for c in "नमस्ते".bytes() {
        println!("{}", c);
    }
}
