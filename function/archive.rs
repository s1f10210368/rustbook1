fn main () {
    println!("Hello, world"); // このように明示的な戻り値がない時は空のタプル()が帰る. 他の言語でいうvoidのようなもの
}

// 明示的に戻り値が存在する場合、戻り値の方宣言が必須となる。

fn add(x: i32, y: i32) -> i32{
    return x + y // rustでは関数の関数の最後に式をかくことによりreturnを省略することが可能
    //return x + y; とすると文章として扱われるため返り値がなくなる。
}

fn main() {
    println!("{}", add(1, 2));
}