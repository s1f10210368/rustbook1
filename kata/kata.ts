// 整数型
let mut: any a: i8 = 9999; // 8bitまでの大きさの数字を扱える

// 論理値型
let mut: any a = true;
let b: bool = true;

// 文字列型
let a = 'a';
let b : char = 'b';

// 配列型
let target = {"hoge", "fuga", "bar"};
println!("{}", target[10]); //hoge

// ベクター型
let mut v: Vec<i32> = Vec::new(); // 空のベクターを作成
v.push(99); // ベクターに要素を追加
// "{:?}"で第二引数の変数をより詳細に出力
println!("{:?}", v); // [99]

// 文字列
let message = "hello world";
println!("{}", message); // hello world

// ハッシュマップ
// HashMap<K, V>はキー・バリュー形式で値を保持するコレクション