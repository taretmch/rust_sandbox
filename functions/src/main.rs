fn main() {
    println!("Hellow, world!");

    // --[ 関数定義 ]-----------------------------------------------------------
    // main 関数を定義してわかる通り、関数を定義するときには fn キーワードを用いる。
    //
    // 命名規則はスネークケースである。全文字を小文字にし、単語間をアンダースコアで区切る。
    // 例) some_function
    // 関数呼び出し
    unit_function();
    first_arg_function(5);
    second_args_function(5, 6);

    // 文: なんらかの動作をして値を返さない命令
    // 式: 結果値に評価される命令

    // 文の例: 値の代入、関数定義
    // let x = (let y = 6); とはできない
    // 代入文が値を返さないので、x = y = 6 のようなことができない。
    let x = 6;

    // 式の例: 値、演算結果、関数呼び出し、ブロック
    let y = {
        let z = 3;
        z + x
    };
    println!("The value of y is {}", y);

    // 戻り値のある関数
    let ret_val = return_function();
    println!("The return value of function is {}", ret_val);

    println!("y + 1 = {}", plus_one(y));
}

// 関数定義
// 関数の定義と関数呼び出しの順番はどちらでもよく、そのスコープに定義さえされていればよい。
//
// 関数本体は、文と式を含むことができる。
fn unit_function() {
    println!("Another function.");
}

// 仮引数には型注釈が必要
fn first_arg_function(x: i32) {

    // 仮引数 x を表示
    println!("The value of parameter is {}", x);
}

// 仮引数を複数定義するには、カンマで区切る
fn second_args_function(x: i32, y: i32) {

    // 仮引数を表示
    println!("The value of first  parameter is {}", x);
    println!("The value of second parameter is {}", y);
}

// 戻り値のある関数
// 戻り値の型は -> によって宣言する。
fn return_function() -> i32 {
    5
}

// 返り値の終端にはセミコロンはつけない
// セミコロンをつけると文になってしまい、値が評価されないので。
fn plus_one(x: i32) -> i32 {
    x + 1
}
