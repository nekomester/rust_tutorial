fn main() {
    let number = 7;
    if number < 5 {
        println!("Condition was true.");
    } else {
        println!("Condition was false.");
    }

    // else if
    let num2 = 5;
    if num2 % 4 == 0 {
        println!("Number is divisible by 4.");
    } else if num2 % 3 == 0 {
        println!("Number is divisible by 3.");
    } else if num2 % 2 == 0 {
        println!("Number is divisible by 2.");
    } else {
        println!("Number is not divisible by 4, 3 or 2.");
    }

    // 三項演算子はこう書くみたい
    let cond = false;
    let num3 = if cond { 5 } else { 6 };
    println!("The value of num3 is: {}", num3);
    // let num3 = if cond { 5 } else { "six" }; // これはコンパイルエラー（型不一致）
}
