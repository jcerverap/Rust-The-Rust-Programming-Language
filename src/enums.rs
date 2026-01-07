#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddress {
    V4(String),
    V6(String),
}

pub fn do_enums() {

    println!("--- Enums Exercise ---");

    let mut my_direction = Direction::North;

    print_direction(my_direction);

    my_direction = Direction::South;
    print_direction(my_direction);

    my_direction = Direction::East;
    print_direction(my_direction);

    my_direction = Direction::West;
    print_direction(my_direction);

    do_ip_address();
    do_ip_address2();
    do_options();
    do_coin();
    do_coin2();
    do_lets();

}

fn print_direction(direction: Direction) {

    println!("My direction is: {:?}", direction);
   
    match direction {
        Direction::North => println!("Heading North"),
        Direction::South => println!("Heading South"),
        Direction::East  => println!("Heading East"),
        Direction::West  => println!("Heading West"),
    }
}

fn do_ip_address() {

    println!("--- IP Address Exercise ---");

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    
    println!("Home IP: {:?}, Loopback IP: {:?}", home, loopback);

}

fn do_ip_address2() {

    println!("--- IP Address Enum with Data Exercise ---");
    #[derive(Debug)]
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1")); 
    println!("Home IP: {:?}, Loopback IP: {:?}", home, loopback);
}

fn do_options() {

    println!("--- Option Enum Exercise ---");

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    println!("Some number: {:?}, Some string: {:?}, Absent number: {:?}", some_number, some_string, absent_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // This line would cause a compile-time error

    let sum = x + y.unwrap_or(0); // Using unwrap_or to provide a default value
    println!("Sum of x and y: {}", sum);
}

fn do_coin() {

    println!("--- Coin Enum Exercise ---");

    let coin = Coin::Penny;
    println!("Value of the coin: {} cents", value_in_cents(coin));

    let coin = Coin::Nickel;
    println!("Value of the coin: {} cents", value_in_cents(coin));

    let coin = Coin::Dime;
    println!("Value of the coin: {} cents", value_in_cents(coin));

    let coin = Coin::Quarter;
    println!("Value of the coin: {} cents", value_in_cents(coin));

    
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => {
            println!("Lucky nickel!");
            5
        },
        Coin::Dime => {
            println!("Lucky dime!");
            10
        },
        Coin::Quarter => {
            println!("Lucky quarter!");
            25
        }
    }
}


#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}
enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn do_coin2() {

    println!("--- Coin Enum with Option Exercise ---");

    let coin = Coin2::Penny;
    println!("Value of the coin: {} cents", value_in_cents2(coin));

    let coin = Coin2::Nickel;
    println!("Value of the coin: {} cents", value_in_cents2(coin));
    let coin = Coin2::Dime;
    println!("Value of the coin: {} cents", value_in_cents2(coin));

    let coin = Coin2::Quarter(UsState::Alabama);
    println!("Value of the coin: {} cents", value_in_cents2(coin));

    let coin = Coin2::Quarter(UsState::Alaska);
    println!("Value of the coin: {} cents", value_in_cents2(coin));


    let coin = Coin2::Quarter(UsState::Alabama);
    if let Some(description) = describe_state_quarter(coin) {
        println!("{}", description);
    }

    let coin = Coin2::Quarter(UsState::Alaska);
    if let Some(description) = describe_state_quarter2(coin) {
        println!("{}", description);
    }

    let coin = Coin2::Quarter(UsState::Alabama);
    if let Some(description) = describe_state_quarter3(coin) {
        println!("{}", description);
    }

}

fn value_in_cents2(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn do_lets() {

    println!("--- Lets and Match Exercise ---");

    let some_u8_value = 0u8;

    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    if let 3 = some_u8_value {
        println!("three");
    }
}

fn describe_state_quarter(coin: Coin2) -> Option<String> {
    if let Coin2::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

fn describe_state_quarter2(coin: Coin2) -> Option<String> {
    let state = if let Coin2::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn describe_state_quarter3(coin: Coin2) -> Option<String> {
    let Coin2::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}