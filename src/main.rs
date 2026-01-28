use std::{collections::HashMap, hash::Hash};

// ベクタ ハッシュマップ
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("Blue");
    let score = scores.get("Blue");
    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }
    // let score = scores.get(&team_name);

    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];

    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // 値を上書きする
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 20);
    scores2.insert(String::from("Blue"), 24);

    println!("{:?}", scores2);

    // キーに値がなかった時のみ値を挿入する
    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Blue"), 10);
    scores3.entry(String::from("Yellow")).or_insert(50);
    scores3.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores3);

    // 古い値に基づいて値を更新する
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
