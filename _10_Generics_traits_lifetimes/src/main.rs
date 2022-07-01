
use std::fmt::{Display, Debug};

use _10_Generics_traits_lifetimes::{Summary, NewsArticle, Tweet}; // as they are defined in lib.rs

fn main() {

    // Generic types, traits, lifetimes:

    let numbers = vec![1,2,3,4,5,99,33,44,22,101,33];
    let mut largest_num = numbers[0];
    for &num in &numbers {
        if num > largest_num {
            largest_num = num;
        }
    }

    fn largest1(numbers: Vec<i32>) -> i32 {
        let mut largest_num = numbers[0];
        for num in numbers {
            if num > largest_num {
                largest_num = num;
            }
        }
        largest_num
    }
    
    fn largest2(numbers: &Vec<i32>) -> i32 {
        let mut largest_num = numbers[0];
        for &num in numbers {
            if num > largest_num {
                largest_num = num;
            }
        }
        largest_num
    }
    
    fn largest3(numbers: &[i32]) -> i32 {
        let mut largest_num = numbers[0];
        for &num in numbers {
            if num > largest_num {
                largest_num = num;
            }
        }
        largest_num
    }
    println!("largest num : {}", largest_num);
    println!("largest num: {}", largest1(numbers));

    fn largest_i32(list: &[i32]) -> i32{
        let mut largest_num = list[0];
        for &num in list {
            if num > largest_num {
                largest_num = num;
            }
        }
        largest_num
    }

    fn largest_char(list: &[char]) -> char {
        let mut larger_char = list[0];
        for &item in list {
            if item > larger_char {
                larger_char = item;
            }
        }
        larger_char
    }

    let number_list = vec![1,2,3,3,4,44,33,22,11,22];
    println!("largest: {}", largest_i32(&number_list));

    let char_list = vec!['a', 'z', 'e', 'f', 'x' , 'w', 'b', 'c'];
    println!("largest: {}", largest_char(&char_list));

    // generic types in Functions

    fn largest_generic <T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest { // to compare T types, T should impl PartialOrd trait
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![1,2,3,3,4,44,33,22,11,22];
    println!("largest: {}", largest_generic(&number_list));

    let char_list = vec!['a', 'z', 'e', 'f', 'x' , 'w', 'b', 'c'];
    println!("largest: {}", largest_generic(&char_list));

    // generics in structs
    struct Point<T> {
        x: T,
        y: T,
    }

    let intpoint = Point { x: 44, y: 44};
    let floatpoint = Point { x: 33.33, y: 44.23 };
    // let wont_work = Point { x: 33, y: 33.33 };

    struct Point1 <T, U> {
        x: T,
        y: U,
    }
    let wont_work = Point1 { x: 33, y: 33.33 };

    // generics in enums: 
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // generics in method definitions : 
    struct Point3 <T> {
        x: T,
        y: T,
    }

    impl<T> Point3 <T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = Point3 { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    println!("p.x = {}", p.x());

    // we can declare methods specific to a certain type also
    struct Point4 <T> {
        x: T,
        y: T,
    }
    impl Point4<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    // multiple types in struct and method definitions
    struct Point5<X1, Y1> {
        x: X1,
        y: Y1,
    }
    impl <X1,Y1> Point5<X1,Y1> {
        fn mixup<X2 , Y2>(self, other: Point5<X2, Y2> )-> Point5<X1, Y2> {
            Point5 { x: self.x , y: other.y } // moving happens here
            // you cannot use points after this
        }
    }
    let p1 = Point5 { x: 5, y: 10.4 };
    let p2 = Point5 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Performance of Code Using Generics
    // generics = runtime cost but rust uses : monomorphization of the code
    // Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

    // code :
    let integer = Some(5);
    let float = Some(5.0);
    // Monomorphized code: 

    enum Option_i32 {
        Some(i32),
        None,
    }
    
    enum Option_f64 {
        Some(f64),
        None,
    }

    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
    // monomorphization makes Rust’s generics extremely efficient at runtime

    // Traits : defining shared behaviour
    // Traits simlar to interfaces in other languages but some differences


    // example aggregator - requires summary from : newsarticles , tweets

    // check lib.rs
    let tweet = Tweet {
        username: String::from("mt"),
        content: String::from("yo mama, check this out"),
        reply: String::from("gg"),
        retweet: false,
    };

    println!("summary of tweet: {}", tweet.summarize());

    // limitations to trait implemention on a type : 
    // local types can implement local traits
    // local traits can be implemented on non local types
    // non local traits can be implemented on local types
    // but non local traits cannot be implemented on non local types in a local project
    // this rule is called orphan rule 
    // why ? two crates (yours, external) can implement same trait and rust doesn't know which trait to use


    // Default implementations
    // check lib.rs file

    // trait as parameter
    fn notify(item: &impl Summary) {
        println!("breaking news: {}", item.summarize());
    }
    notify(&tweet);
    
    fn notify_big(item1: &impl Summary, item2: &impl Summary) {
        println!("breaking news: {}", item1.summarize());
        println!("breaking news: {}", item1.summarize());
    }
    notify_big(&tweet, &tweet);

    fn notify_simple<T: Summary>(item1: &T, item2: &T) {
        println!("breaking news: {}", item1.summarize());
        println!("breaking news: {}", item2.summarize());
    }
    
    notify_simple(&tweet, &tweet);

    // Specifying Multiple Trait Bounds with the + Syntax

    fn notify_multiple_triats(item: &(impl Summary + Display)) {

    }

    fn notify_multiple_traits2 <T: Summary + Display> (item: &T) {
    }

    // clearer traits with where clause 
    // ugly and hard to read
    fn notify_multiple_ugly <T: Summary + Display, U: Clone + Debug> (item1: &T, item2: &U) {
    }
    // good version with where 
    fn notify_multiple_better<T, U> (item: &T, item2: &U) -> i32
    where T: Summary + Display, U: Clone + Debug {
        88
    }

    // returning types that implement Traits
    fn returning_summarizable() -> impl Summary {
        Tweet {
            username: String::from("some idiot"),
            content: String::from("hi this is some dumb content"),
            reply: String::from("some reply"),
            retweet: false,
        }
    }

    let something_to_summarize = returning_summarizable();
    println!("{}", something_to_summarize.summarize());
    // helpful in context of closures and iterators

    // fn returning_summarizable_that_wont_work(news: bool) -> impl Summary {
    //     if news {
    //         NewsArticle {
    //             author: String::from("some dude"),
    //             content: String::from("some content"),
    //             headline: String::from("hello, earth"),
    //             location: String::from("solar system"),
    //         }
    //     } else {
    //         Tweet {
    //             username: String::from("same dude"),
    //             content: String::from("same content"),
    //             reply: String::from("no one replied"),
    //             retweet: String::from("some tweet"),
    //         }
    //     }
    // }

    fn largest_generic_new <T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![1,2,3,4,44,55,99,22,33,11,33,34];
    println!("largest: {}", largest_generic_new(&number_list));

    let char_list = vec!['y', 'r', 'j', 'j', 's' ];
    let large_char = largest_generic_new(&char_list);

    // using trait bounds to conditionally implement methods

    struct Pair <T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Pair<T> {
            Pair {x, y}
        }
    }

    impl <T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("largest member: x is {}", self.x);
            } else {
                println!("largest member: y is {}", self.y);
            }
        }
    }

   
   
    impl<T> Pair<T> where T: Summary + Display {
        fn summarize(&self) -> String {
            format!("summairzed : {}", self.x)
        }
    }

    trait Creator {
        fn get_creator() -> String;
    }

    // impl<T: Summary> Creator for T {
    //     fn get_creator() -> String {
    //         String::from("mohan")
    //     }
    // }


    let sompair = Pair::new(44, 55);
    println!("{:?}", sompair.cmp_display());
    // println!("{:?}", sompair.summarize());

    // blanket implementations: 


    // lifetime 
    // {
    //     let r;

    //     {
    //         let x = 5;
    //         r = &x; // error here
    //     }

    //     println!("r: {}", r);
    // }

    // borrow checker: 
    // {
    //     let r;           // ---------+-- 'a
    //                           //          |
    //     {                     //          |
    //         let x = 5;        // -+-- 'b  |
    //         r = &x;           //  |       |
    //     }                     // -+       |
    //                           //          |
    //     println!("r: {}", r); //          |
    // }                         // ---------+

    // Life time annotation syntax
    // &i32; // a reference
    // &'a i32; // a reference with lifetime
    // &'a mut i32;// a mutable reference with life time
    // https://www.reddit.com/r/rust/comments/bltnfv/simplest_best_explanation_of_lifetimes/

    // fn longest1(x: &str, y: &str) -> &str { // askes for life time
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }

    fn longest<'a >(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let x = "some string";
    let y = "some other string";
    let l_str = longest(x, y);
    println!("Longest str: {}", l_str);

    // life times are to be specified when references are returned or taken by the functions 
    // because to make sure the current to make sure that the reference that is shared
    // is removed after the end of the lifetime specified
    // above the life time is `a` is just a lifetime name which is nothing but current scope
    // the string slice (reference) returned by function will be removed after the curren scope ends


    fn longest2<'a>(x: &'a str, y: &str) -> &'a str { // life time is not specified for y as it is not returned
       if y.len() == 2 {
           return x
       }
       let rr = "ddd";
       rr
    }
    println!("{}", longest2("333", "33"));

    // fn longest3<'a> (x: &'a str) -> &'a str {
    //     let someString = String::from("hello");
    //     someString.as_str() // as string is created here you cannot pass the reference
    //     // cause it will cause dangling reference
    // }

    // life time definitions in struct definitions
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let novel = String::from("I write shit novels");
    let firstSentence = novel.split('.').next().expect("some error message");
    let i = ImportantExcerpt {
        part: firstSentence,
    };

    // life time elison rules 

    // event though the below code returns / takes references without lifetime definitions
    // it won;t throw the error because the compiler defines life times automatically in 
    // some simple situations
    fn first_word(word: &str) -> &str {
        let bytes_of_word = word.as_bytes();

        for (index, &item) in bytes_of_word.iter().enumerate() {
            if item == b' ' {
                return &word[0..index];
            }
        }
        &word[..]
    }

    // input lifetimes and output lifetimes , lifetime elision rules
    // three rules which compiler uses to find it requires programmer to explictly mention lifetimes
    // 1. rust defines different lifetimes for every input references it gets
    // 2. if fn gets only one input references, its lifetime will be automatically assigned to output reference
    // 3. if method(&self or &mut self as parameter) then life time of self is assigned to output reference

    // if above rules are not matched programmer need to define lifetimes

    // lifetime annotations in method definitions 
    
    impl <'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }
    impl <'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            // as rule (3) rust assigns output reference , the lifetime of self
            println!("{}", announcement);
            self.part
        }
    }
    // static lifetime 
    let strin = "somestring";
    // is same as 
    let strin: &'static str = "someString";
    // both the literals are stored in binary and will be available in throught the program
    // scope and availability are two things

    // Generic type parameters, trait bounds, lifetimes : dj remix

    fn longest_with_an_announcement<'a, T> (
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where T: Display {
        println!("{}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    
}




