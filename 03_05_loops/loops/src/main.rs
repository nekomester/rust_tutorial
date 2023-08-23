// ループは3種ある
fn main() {
    // loop: 明示的にbreak;するまで処理を続ける
    // → break; 無しで永久loop
    let mut i: u32 = 0;
    loop {
        println!("loop count is {}.", i + 1);
        i += 1;
        if i == 10 {
            break;
        }
    }

    // break, continueは内側のloopが対象
    // ループラベルを定義、指定することで外側のloop に適用される
    // ループラベルは 'hoge （シングルクオート + ラベル名）
    let mut count: u32 = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    // while文
    let mut num4 = 3;
    while num4 != 0 {
        println!("{}", num4);
        num4 -= 1;
    }
    println!("LIFTOFF!!!");

    // for文
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {}", element);
    }
    for i in (1..=3).rev() {
        println!("{}!", i);
    }
    println!("LIFTOFF!!! using for");

    // 温度
    let c = 37;
    let f = 59;
    println!("摂氏 {} C は 華氏 {} F.", c, degree_c_to_f(c));
    println!("華氏 {} F は 摂氏 {} C.", f, degree_f_to_c(f));

    // フィボナッチ数列
    let target = 15;
    println!("フィボナッチ配列の {} 番目: {}", target, fibonacci(target))
}


// 摂氏から華氏算出
fn degree_c_to_f(c: i32) -> f32 {
    c as f32 * 1.8 + 32 as f32
}

// 華氏から摂氏算出
fn degree_f_to_c(f: i32) -> f32 {
    (f - 32) as f32 / 1.8
}

// フィボナッチ数列の指定数を生成
fn fibonacci(target: u32) -> u32 {
    let mut x = 1;
    let mut y = 0;
    for _ in 1..=target {
        (x, y) = (y, x + y);
    }
    y
}
