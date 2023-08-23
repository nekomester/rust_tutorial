fn main() {
    another_function(5);

    print_labeled_measurement(3, 'h');

    println!("The value of fn five ret value is: {}", five());

    println!("The value of fn add_one ret value is: {}", add_one(123));
}

fn another_function(x: i32) {
    println!("This is anothre_function. The value is: {}", x);
}

fn print_labeled_measurement(value: i32, label: char) {
    println!("This is print_labeled_measurement. The value is: {}{}", value, label);
}

fn five() -> i32 {
    5 * 10 // 戻り値にはセミコロンはつけない
}

fn add_one(x: i32) -> i32 {
    x + 1 // 引数を取る場合も同様
}
