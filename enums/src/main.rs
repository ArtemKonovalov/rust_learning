use std::ptr::{eq, null};
use crate::IpAddrs::V6;

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddress {
    V4(String),
    V6(String),
}

enum IpAddrs {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddrs {
    fn test(&self) -> String {
        return String::from("sdf");
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddress::V4(String::from("127.0.0.1"));
    let loopback = IpAddress::V6(String::from("::1"));

    let home = IpAddrs::V4(127, 0, 0, 1);
    let loopback = IpAddrs::V6(String::from("::1"));

    home.test();
    loopback.test();

    let some_char = Some('s');
    let absent_number: Option<i32> = None;
    let chr = some_char.expect("There no character");
    let five = Some(5);
    let six = plus_one(five);

    // --- if let
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => ()
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max)
    }

    let coin = Coin::Dime;
    let mut count = 0;
    match coin {
        Coin::Quarter(ref state) => println!("State quarter from {:?}!", state),
        _ => count += 1
    }

    // the same but with if let expression
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("Count: {}", count);
}

fn route(ip_kind: IpAddrKind) {}

// ----

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn dice_roll_action(dice_roll: u8) {
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other)
    }
}

fn dice_roll_action_without_other_usage(dice_roll: u8) {
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll()
    }
}

fn dice_roll_action_without_action(dice_roll: u8) {
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => ()
    }
}

fn add_fancy_hat() {}

fn remove_fancy_hat() {}

fn move_player(num_spaces: u8) {}

fn reroll() {}
