
fn main() {

    //declaring simple enums and using them
    #[derive(Debug)]
    enum IpAddressKind {
        V4,
        V6,
    }
    #[derive(Debug)]
    struct IpAddress{
        kind: IpAddressKind,
        address: String,
    }

    fn route(ip_address: IpAddressKind) {
        println!("ip address passed: {:?}", ip_address);
    }

    route(IpAddressKind::V4);
    route(IpAddressKind::V6);

    let home_address:IpAddress = IpAddress {
        kind: IpAddressKind::V4,
        address: String::from("192.33.44.12")
    };
    let loop_back: IpAddress = IpAddress {
        kind: IpAddressKind::V6,
        address: String::from("::1"),
    };
    println!("home address: {:?}", home_address);
    println!("loopback address: {:?}", loop_back);

    // another way: having values for the types
    
    enum IpAddressKind1 {
        V4(String),
        V6(String),
    }
    let home = IpAddressKind1::V4(String::from("182:11:22:88"));
    let loopback = IpAddressKind1::V6(String::from("::33"));

    // another way : having values of different types

    enum IpAddressKind2 {
        V4(u8, u8, u8, u8), 
        V6(String),
    }
    let home = IpAddressKind2::V4(183,222,33,99);
    let loopback = IpAddressKind2::V6(String::from("::33"));

    // another way:
    #[derive(Debug)]
    struct IpV4Add {
        address: String,
    }
    #[derive(Debug)]
    struct IpV6Add {
        address: String,
    }

    #[derive(Debug)]
    enum IpAddressKind3 {
        V4(IpV4Add),
        V6(IpV6Add)
    }
    println!("IpAddressKind3: ipv4 is: {:?}", IpAddressKind3::V4(IpV4Add { address: String::from("124:22:333:88") }));
    println!("IpAddressKind3: ipv6 is: {:?}", IpAddressKind3::V6(IpV6Add { address: String::from("124:22:333:88") }));

    // one more example
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // the same enum in structs 
    struct Quit; // unit struct
    struct Move { x: i32, y: i32 }
    struct Write(String); // tuple struct
    struct ChangeColor(i32, i32, i32); // tuple struct

    // but we cannot pass this kind of messages as single type into a function
    // like the below
    fn printMessage(msg: &Message) {
        println!("Message: {:?}", msg);
    }
    let someMessage = Message::Write(String::from("hello world"));
    printMessage(&someMessage);

    // adding methods to the enum : Message

    impl Message {
        fn print(&self) {
           // no defination
           printMessage(self);
        }
    }
    let message1 = Message::Write(String::from("hi"));
    message1.print();


    // Option Enum 
    // better way to regulate null pointer exceptions
    // https://doc.rust-lang.org/std/option/enum.Option.html

    let x: Option<u32> = Some(2);
    assert_eq!(x.is_none(), false);
    let x: Option<u32> = None;
    assert_eq!(x.is_none(), true);
    // so many useful methods available in Option check the link

    // match control flow operator
    #[derive(Debug, PartialEq, Clone, Copy)]
    enum Country {
        US,
        UK,
        Canada,
    }
    enum Coin {
        Paisa,
        Fiftypaisa,
        Rupee,
        Fiverupee,
        Tenrupee,
        Dollar(Country),
    }

    fn value_in_paisa(coin: Coin) -> u32 {
        match coin {
            Coin::Paisa => 1,
            Coin::Fiftypaisa => 50,
            Coin::Rupee => 100,
            Coin::Fiverupee => 500,
            Coin::Tenrupee => {
                print!("you can also use curly blocks");
                1000
            },
            Coin::Dollar(country) => {
                match country {
                   Country::Canada => 70000,
                   Country::UK => 67000,
                   Country::US => 4444444, 
                }
            }
        }
    }

    assert_eq!(value_in_paisa(Coin::Fiftypaisa), 50);
    assert_eq!(value_in_paisa(Coin::Dollar(Country::UK)), 67000);

    // match should cover all the variants
    // matching with Option<T>

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(x) => Some(x + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    assert_eq!(six.expect("none"), 6);
    assert_eq!(none.is_none(), true);

    // catch all pattern and _ place holder

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // like default case
    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // instead of other
    }

    // if let 
    let config_max = Some(44u8);
    match config_max {
        Some(max) => print!("max value is : {}", max),
        _ => ()  // unnecessary code we have to add
    }

    let config_max = Some(333);
    if let Some(max) = config_max  {
        print!("max value is : {}", max);
        assert_eq!(max, 333);
    }

    let coin = Coin::Dollar(Country::US);

    let mut count = 0; 
     match coin {
         Coin::Dollar(country) => {
             assert_eq!(country, Country::US);
         },
         _ => {
             print!("default");
             count += 1;
         },
     }

     let mut count= 0;
     if let Coin::Dollar(country) = coin {
        // assert_eq!(country, Country::US);
        println!("State quarter from {:?}!", country);
     } else {
         count += 1;
     }

}
