enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        //method def
    }
}
fn main() {
    let four = IpAddr::V4(127, 0, 0, 1);
    let six = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello world!"));
    m.call();

    let some_number = Some(13);
    let some_string = Some(String::from("Some"));

    let absent_number: Option<i32> = None;

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8_value {
        println!("three!");
    }

    let coin = Coin::Quarter(UsState::Alabama);
    let mut count: u32 = 0;
    match coin {
        Coin::Quarter(state) => println!("{:?}!", state),
        _ => count += 1,
    }
    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("{:?}!", state);
    } else {
        count += 1;
    }
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("from state {:?}", state);
                25
            }
        }
    }
}
