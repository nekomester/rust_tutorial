// 外部ファイル front_of_house

// pub mod hosting {
//     pub fn add_to_waitlist() {}
//     fn seat_at_table() {}
// }

// さらに hosting を別ファイルへ切りだす
pub mod hosting;

mod serving {
    fn take_over() {}
    fn serve_over() {}
    fn take_payment() {}
}
