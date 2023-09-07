
fn main() {
    use std::collections::HashMap;

    // HashMapインスタンス
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // インスタンス（マクロ）
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![20, 30];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores2);

    // append
    scores.insert(String::from("Red"), 99);
    scores.insert(String::from("Green"), 5);
    println!("{:?}", scores);

    // key未存在時のみappend
    scores.entry(String::from("Blue")).or_insert(111);
    scores.entry(String::from("Orange")).or_insert(222);
    println!("{:?}", scores);
}
