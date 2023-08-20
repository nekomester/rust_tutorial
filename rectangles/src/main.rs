// ###############################################
// ### 構造体利用前
// ###############################################

// fn main() {
//     // let width1 = 30;
//     // let height1 = 50;
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         // area(width1, height1)
//         area(rect1)
//     );
// }

// // fn area(width1: u32, height1: u32) -> u32 {
// // width1 * height1
// // }
// fn area(dimentions: (u32, u32)) -> u32 {
//     dimentions.0 * dimentions.1
// }


// ###############################################
// ### 構造体利用後
// ###############################################
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    )
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
