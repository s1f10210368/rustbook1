enum Color {
    red,
    blue, 
    green,
    Hex(String),
}

// 以下はどちらもColor型
let red = Color::Red;
let hex = Color::Hex("ffffff".to_string());