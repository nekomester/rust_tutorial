fn main() {
    // move
    let s1 = String::from("DoReMi");
    let s2 = s1;
    println!("{}.", s2); // s1は解放(?)されてるためエラー

    // 参照
    let s3 = String::from("FaSoRa");
    let s4 = &s3;
    println!("s3: {}, s4: {}.", s3, s4); // 参照渡しならs3は解放されない

    // 参照（関数）
    let s5 = String::from("DoMiSoReFaRa");
    let s5_len = caluculate_length(&s5);
    println!("s5: {}, length: {}.", s5, s5_len); // 参照渡ししてるのでs5は表示可能

    // スライス
    let s6 = String::from("Hello World.");
    println!("slice result is \"{}\".", first_word(&s6));

    // スライス（配列）
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    for i in slice {
        println!("slimce  {}.", i);
    }
}

// 関数の引数に参照を取ることを「借用」という
fn caluculate_length(s: &String) -> usize {
    s.len()
}

// スライス
// &str は文字列スライスを意味する型
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
