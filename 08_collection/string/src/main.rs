fn main() {
    // 文字列連結 (String.push_str(add))
    let mut s = String::from("super");
    s.push_str(" mario bros.");
    println!("{}", s);

    // 文字列連結 (+演算子)
    let s1 = String::from("doremi");
    let s2 = String::from("fasora");
    let s3 = s1 + &s2; // s2 は &String で指定すること
    println!("{}", s3);

    // 文字列連結 (format! マクロ)
    let s4 = String::from("do");
    let s5 = String::from("mi");
    let s6 = String::from("so");
    let s7 = format!("{}-{}-{}", s4, s5 ,s6);
    println!("{}", s7);

    // 文字数
    let s8 = String::from("あいうえお");
    println!("chars len is {}.", s8.len()); // これは15（つまりバイト数）
    println!("chars chars count is {}.", s8.chars().count()); // これは5（ので文字数）
}
