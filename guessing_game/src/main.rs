// rand クレートを外部依存として使用する
extern crate rand;

// 標準の IO ライブラリをインポート
use std::io;
use std::cmp::Ordering;
// Rng トレイトは、乱数生成器が実装するメソッドを定義しているもの
use rand::Rng;

fn main() {
    println!("Guess the number!");

    //--------------------------------------------------------------------------
    // thread_rng() 関数は乱数生成器を返す関数
    // 乱数生成器の gen_range メソッドは、Rng トレイトで定義されているメソッド
    // gen_range(1, 101) で1以上101未満の乱数を生成している
    //--------------------------------------------------------------------------
    let secret_number = rand::thread_rng().gen_range(1, 101);

    //--------------------------------------------------------------------------
    // 無限ループ
    //--------------------------------------------------------------------------
    loop {
        println!("Please input your guess.");

        //--------------------------------------------------------------------------
        // let 文によって変数を生成する
        // 変数の前に `mut` をつけると、変数が可変になる
        // 
        // 通常、`let foo = 5` のようにすると、不変な変数として定義される
        //
        // `String::new` は、String 型が持っている静的関数で
        // 空の String 型オブジェクトを生成する、すなわち空の文字列を生成する関数
        //--------------------------------------------------------------------------
        let mut guess = String::new();

        //--------------------------------------------------------------------------
        // `stdin()` 関数は、`std::io::Stdin` オブジェクトを返す関数。
        // Stdin 型は、標準入力へのハンドルを表す型である
        //
        // `read_line 関数は、標準入力からユーザーの入力を受け付ける関数。
        //
        // `&` は、C言語同様に変数を参照渡しするためのもの。
        // ただし、参照渡しも基本不変なので、可変にするには `&guess` ではなく `&mut guess` とする必要がある
        //
        // read_line() 関数は文字列型以外に値を返す。その汎用的な型は `io::Result` である。
        // Result 型は列挙型で、列挙子としては Ok と Err がある。
        // Result 型には、エラーハンドリングのための関数として expect 関数がある。
        // この関数は、Err の場合プログラムをクラッシュさせ、引数のメッセージを出力する。
        //--------------------------------------------------------------------------
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        //--------------------------------------------------------------------------
        // 文字列型を数値型 u32 に変換する
        // secret_number と guess を比較するときに型のミスマッチが起こらないようにする
        //
        // Rust において、既存の変数名を使ってその変数の値を覆い隠す (shadow) ことができる
        //
        // String 型の trim メソッドは、文字列の両端の空文字を削除するメソッドである
        // String 型の parse メソッドは、文字列をなんらかの数値に変換するメソッドである
        // guess の型を u32 と固定することによって、32ビットの非負整数に変換される
        //
        // parse メソッドは失敗する可能性がある（文字列が数値である保証はない）ので
        // Result 型を返す。
        //
        // Err の場合に continue を呼び出すことによって、ループをスキップできる
        //--------------------------------------------------------------------------
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        //--------------------------------------------------------------------------
        // println! マクロの {} はプレースホルダーを表す
        //--------------------------------------------------------------------------
        println!("You guessed: {}", guess);

        //--------------------------------------------------------------------------
        // 比較処理
        // Ordering 型も列挙型であり、以下の3つの列挙子からなる：
        //
        // 1) Less
        // 2) Greater
        // 3) Equal
        //
        // cmp メソッドは2値を比較するメソッドで、引数として比較対象の変数の参照を渡す。
        // cmp メソッドが返すのは Ordering 型の値である。
        //--------------------------------------------------------------------------
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                // break 文によって、ループを終了できる
                break;
            }
        }
    }
}
