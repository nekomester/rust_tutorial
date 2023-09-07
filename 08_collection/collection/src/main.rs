fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // 添字記法
    let third: &i32 = &v[2];
    println!("the third element is {}.", third);

    // getメソッド -> OPTION<&T> を得る
    match v.get(2) {
        Some(third) => println!("the third element is {}.", third),
        None => println!("there is no third element."),
    }

    // 添字範囲外アクセス: []はエラー（パニック）、getメソッドはNone（エラーにならない）
    // let does_not_exist = v[100];
    let does_not_exist = v.get(100);

    // 全件表示（for文）
    for i in &v {
        println!("{}", i);
    }

    // 値変更とかできちゃう
    let mut w = vec![100, 33, 125];
    for i in &mut w {
        // * は参照外し演算子
        *i += 33;
    }
    for i in &w {
        println!("{}", i);
    }

}
