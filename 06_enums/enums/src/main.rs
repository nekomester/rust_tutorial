#[derive(Debug)]
enum SomeStatus {
    Prepare = 0,
    Open = 1,
    Close = 9,
}

impl SomeStatus {
    fn get_code(&self) -> i32 {
        match self {
            SomeStatus::Prepare => SomeStatus::Prepare as i32,
            SomeStatus::Open => SomeStatus::Open as i32,
            SomeStatus::Close => SomeStatus::Close as i32,
        }
    }

    fn get_name(&self) -> String {
        match self {
            SomeStatus::Prepare => String::from("prep."),
            SomeStatus::Open => String::from("open."),
            SomeStatus::Close => String::from("close."),
        }
    }
}

fn main() {
    println!("1. {:#?} {}", SomeStatus::Prepare, SomeStatus::Prepare as i32);
    println!("2. {:#?} {}", SomeStatus::Open, SomeStatus::Open as i32);
    println!("3. {:#?} {}", SomeStatus::Close, SomeStatus::Close as i32);

    let status = SomeStatus::Open;
    println!("status code {}, name {}.", status.get_code(), status.get_name());

    // match式
    match status {
        SomeStatus::Prepare => println!("準備中"),
        SomeStatus::Open => println!("公開"),
        SomeStatus::Close => println!("非公開"),
    }

    // Option<T> - unwrap() で T を取り出せる
    let ans = add_one(Some(7));
    println!("add 1 is {:#?}.", ans.unwrap());

    // if let - enumの比較
    if let SomeStatus::Close = status {
        println!("it is close, anyway.");
    } else {
        println!("it is not close yet.");
    }
}

fn add_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
