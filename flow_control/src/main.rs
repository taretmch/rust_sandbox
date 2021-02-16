fn main() {

    let number = 3;

    // --[ if 式 ]--------------------------------------------------------------
    // if キーワードから始め、条件式を与える。
    // 条件式は bool 型でなければならず、number を単体で与えてもコンパイルエラーになる。
    if number < 5 {
        println!("number {} is less than 5", number);
    } else if number % 4 == 0 {
        println!("number {} is divisible by 4", number);
    } else {
        println!("condition was false");
    }

    // if は式なので、let 文に代入することができる。
    let x = if number < 5 {
        10
    } else {
        1
    };

    println!("The value of x is {}", x);

    // --[ ループ ]-------------------------------------------------------------
    // Rust には3種類のループが存在する:
    //
    // 1) loop
    // 2) while
    // 3) for

    // --[ ループ: loop ]-------------------------------------------------------
    // loop は、明示的にやめさせるまでブロック内の処理を繰り返す構文である。
    let mut count = 0;
    loop {
        count = count + 1;
        if count > 10 {
            println!("loop finished!");
            break;
        }
        println!("loop again!");
    }

    // --[ ループ: while ]------------------------------------------------------
    // while によって、条件付きのループが表現できる。
    count = 0;
    while count < 10 {
        println!("count: {}", count);
        count = count + 1;
    }

    // --[ ループ: for ]--------------------------------------------------------
    // for によって、コレクションの各アイテムに対してコードを実行することができる
    let array = [10, 20, 30, 40, 50];

    for element in array.iter() {
        println!("The value is {}", element);
    }

    // --[ フィボナッチ数 ]-----------------------------------------------------
    println!("fibonacci number (1): {}", fibonacci_number(1));
    println!("fibonacci number (2): {}", fibonacci_number(2));
    println!("fibonacci number (4): {}", fibonacci_number(4));
    println!("fibonacci number (10): {}", fibonacci_number(10));
    println!("fibonacci number (50): {}", fibonacci_number(50));
}

fn fibonacci_number(n: i32) -> i64 {
    let mut x_before:  i64 = 1;
    let mut x_current: i64 = 2;
    let mut i              = 3;
    while i <= n {
        let tmp   = x_current;
        x_current = x_current + x_before;
        x_before  = tmp;
        i         = i + 1;
    }
    if n > 1 {
        x_current
    } else {
        x_before
    }
}
