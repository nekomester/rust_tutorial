pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// ここから

// 外部ファイル front_of_house.rs に切り出す
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         fn seat_at_table() {}
//     }
//     mod serving {
//         fn take_over() {}
//         fn serve_over() {}
//         fn take_payment() {}
//     }
// }

// ここの定義と実ファイル名でマッピング
mod front_of_house;

// useモジュール
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 絶対パスで add_to_waitlist() 実行
    // crate::front_of_house::hosting::add_to_waitlist();

    // 早退パスで add_to_waitlist() 実行
    // front_of_house::hosting::add_to_waitlist();

    // useモジュール
    hosting::add_to_waitlist();

    // breakfast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 下記はコンパイルエラー
    // meal.seasonal_fruit = String::from("blueberries");
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
    
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
