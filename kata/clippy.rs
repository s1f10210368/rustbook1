fn main() {
    let my_foo = "Foo";
    for i in 0..3 {
        // "{}"で第二引数の変数を出力
        println!("{}", my_foo);
    }
}
// 上に cargo clippy を適用すると以下のように
fn main() {
    let my_foo = "Foo";
    for _i in 0..3 {
        // "{}"で第二引数の変数を出力
        println!("{}", my_foo);
    }
}