fn main() {
    // 変数 - mutあり: 可変、mutなし: 不変
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 定数
    // 1. 型を必ず注釈する
    // 2. 命名規則: UPPER_SNAKE_CASE
    const MAX_NUMBER: u32 = 99_999;
    println!("The const value of MAX_NUMBER is: {}", MAX_NUMBER);


    // シャドーイング
    let y = 5;

    let y = y + 1; // let で再定義（let無しの場合はコンパイルエラー）

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y); // 12になる
    }

    println!("The value of y is: {}", y); // 6になる（直前とはスコープがことなるため）
}
