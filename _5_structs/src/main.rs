
#[derive(Debug)] // to print a struct instance one need to implement this debug
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {

    // declaration and usage of structs



    let user1 = User {
        username: String::from("some user"),
        email: String::from("someuser@gmail.com"),
        sign_in_count: 0,
        active: true,
    };
    println!("user struct: {:?}", user1);

    // mutable struct
    let mut user2: User = User {
        username: String::from("some user1"),
        email: String::from("someuser1@gmail.com"),
        sign_in_count: 8,
        active: true
    };
    println!("user2: {:?}", user2);
    user2.email = String::from("someuser1second@gmail.com");
    println!("user2: {:?}", user2);

    let user3: User = build_user(String::from("user3"), String::from("user3@gmail.com"));
    println!("user3 : {:?}", user3);

    // build mutable user 
    let mut user4 = build_user(String::from("ssss"), String::from("hello@gmail.com"));
    user4.username = String::from("hello user4");
    user4.active = false;
    user4.sign_in_count = 44;
    println!("user4 : {:?}", user4);


    // create instances from other instances with Struct update syntax

    let user5 = User {
        username: String::from("user5"),
        email: String::from("user5@gm.com"),
        ..user4  // not like javascript spread operator: only puts the missing ones
    };
    println!("user5: {:?}", user5); // user5, user5@gm.com, 44, false

    let user6 = User {
        username: user5.username,
        email: user5.email,
        ..user2
    };
    println!("user6: {:?}", user6); // user5, user5@gm.com, 8, true

  
    // tuple structs without named fields to create different types
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black: Color = Color(0,0,0);
    let origin: Point = Point(0,0,0);

    println!("color : {:?} ", black);
    println!("origin: {:?}", origin);

    // Unit like structs without any fields
    // why? chapter 10 traits
    #[derive(Debug)]
    struct Unit();
    
    let unit:Unit = Unit();
    println!("unit : {:?}", unit);


    // A Example program using structs 
    #[derive(Debug)] // debug trait
    struct Rect {
        width: u32,
        height: u32
    }
    fn calculate_area(rectangle: &Rect) -> u32{
        rectangle.width * rectangle.height
    }
    let rect1 = Rect {
        width: 44,
        height: 555
    };
    let area = calculate_area(&rect1);
    println!("area of rectangle {:#?}, is {}", rect1, area); // {:#?} pretty prints


    // Method syntax
    // methods are functions declared in struct, trait, enum object
    // self is always their first parameter: self = instance = this in js

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32
    }
    impl Rectangle {
        fn area(&self) -> u32 {  // declaring methods
            self.height * self.width
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let average_rect: Rectangle = Rectangle {
        width: 50,
        height: 100,
    };
    println!("area of rectangle1: {}", (&average_rect).area()); // accessing methods
    println!("area of rectangle1: {}", average_rect.area());

    let small_rect = Rectangle {
        height: 40,
        width: 48,
    };
    println!("will smallrect fit in average rect: {} ? :", average_rect.can_hold(&small_rect));

    let large_rect: Rectangle = Rectangle {
        height: 400,
        width: 500,
    };
    println!("will large rect fit in average rect: {} ? :", average_rect.can_hold(&large_rect));

    // Associated functions : functions in impl that don't take self as parameter (static methods)
    impl Rectangle {  // multiple impls blocks we can write
        fn square(side_len: u32) -> Rectangle {
            Rectangle {
                width: side_len,
                height: side_len
            }
        }
    }
    // calling associated functions
    let square = Rectangle::square(44);
    println!("area of square: {}", square.area());




}




fn build_user(username: String, email: String) -> User {
   User {
        username: username,
        email, // shorthand assignment
        sign_in_count: 0,
        active: true
    }
}