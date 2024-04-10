fn main() {
    let mut message = "hello, foo"; // message に"hello, foo" を束縛している、これは再代入できないためエラーが出る
    println!("{}", message);

    message = "hello, bar";// mut をつけることで再代入が可能となる。
    println!("{}", message);
}
