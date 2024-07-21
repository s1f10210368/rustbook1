enum Color {
    red,
    blue, 
    green,
    Hex(String),
}

// 以下はどちらもColor型、enumは異なるデータ構造も含有することができる。
let red = Color::Red;
let hex = Color::Hex("ffffff".to_string());

// Option、値があるかどうかを表すenum

enum Option<T> {
    Some(T),
    None,
}

// 上記では値を取り出すにはSomeかどうかの判別が必須になる。
// この判別コードによりnull考慮漏れのようなバグの混入を防げる