use std::vec;

// ベクタ
fn main() {
    let mut v = vec![1, 2, 3];

    let third: &i32 = &v[2];

    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // 最初の要素への参照が、ベクタの終端への変更を気にする必要はなさそうだが、
    // 新たな要素を追加するときに、今のベクタのある場所に全要素を隣り合わせに配置することができない可能性がある
    // そうなったら新しいメモリに割り当て、古い要素を新しいスペースにコピーする必要がある
    // その場合、最初の要素を指す参照は解放されたメモリを指すことになる
    let first = &v[0];
    v.push(6);
    println!("The first element is : {}", first);

    let v2 = vec![100, 321, 3];
    for i in &v2 {
        println!("{}", i);
    }

    let mut v3 = vec![100, 321, 3];
    for i in &mut v3 {
        // 参照外し演算子(*)
        *i += 50;
    }

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];
}
