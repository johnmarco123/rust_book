// enum IpAddr {
//     V4(u8,u8,u8,u8,),
//     V6(String),
// }
//
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 }, 
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// enum Person {
//     Name(Option<String>),
// }
#[derive(Debug)] // so we can inspect the state in a minute
enum Province {
    Toronto,
    Mississauga,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Option<Province>),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(Province) => {
            println!("State quarter from {:?}!", Province);
            25
        }
    }
}

fn main(){
    let coin = Coin::Quarter(Some(Province::Toronto));
    value_in_cents(coin);
}
