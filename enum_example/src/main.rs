enum IpAddr {
    V4(String),
    V6(String)
}

enum IP {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    Color(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("call called")
    }
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Penny killer");
            return 1
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Quarter => 25
    }
}

fn main() {
    let home_addr = IpAddr::V4(String::from("127.0.0.1"));
    let loop_back = IpAddr::V6(String::from("::1"));
    let home_addr_2 = IP::V4(1,2,3,4);
    let loop_back_2 = IP::V6(String::from("::2"));
    println!("Hello, world!");
    let m = Message::Quit;
    m.call();

    println!("{}", value_in_cents(Coin::Dime))
}
