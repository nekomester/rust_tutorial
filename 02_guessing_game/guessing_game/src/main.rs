use std::io;
use std::cmp::Ordering;
use rand::Rng;

// 2. 数当てゲーム
fn main() {
    println!("Guess the numger!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // コメントアウトで乱数値(=答え)を出力する
    // println!("The secrete number is: {}", secret_number);

    loop {
        println!("Plz input your guess!");

        // ユーザー入力取得
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line."); // 行読み込み失敗時

        // 1. 上で変数guessを定義してるが、再定義できる
        // 2. 下記コメントままだと、数値以外の入力でクラッシュする
        // let guess: u32 = guess.trim().parse()
        //     .expect("Plz type a number.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // Err(_) => continue,
            Err(_) => {
                println!("woops, Plz input number.");
                continue;
            },
        };

        println!("You guessed: {}", guess); // 次のように予想

        // 比較
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small."), // 小さい
            Ordering::Greater => println!("Too big."), // 大きい
            Ordering::Equal => {
                println!("You win!!!");
                break;
            },
        }
    }
}
