fn main() {

    // Ownership : 
    // memory:  same as other languages doesn't use garbage collections or manual cleanups,
    // but uses ownership with set of rules 

    // stack : used for datas whose size is known at compile time
    // heap : used for datas whose size is dynamic and not known at compile time

    // ownership Rules : 
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // variable scopes 
    {  // x is not valid here
        let x = 88; // x is valid from this point ownwards
        println!("x value : {}", x); // x is valid here
    } // x value is droped
    // println!("x values : {}", x); //  x is invalid here and out of scope

    // storing string in heap
    let mut s = String::from("");
    s.push_str("some other");
    println!("value of string: s : {}", s);

    // Rust uses drop function to clear the memory allocations
    // this is done at every scope end automatically 
    // which is similar to Resource Acquisition Is Initialization (RAII) pattern in C++

    // stored in heap
    let s1 = String::from("some string");
    let s2 = s1; 
    // you cannot call s1 after this as s1 is invalidated
    // println!("strings: s1:{}, s2: {}", s1, s2);

    // stored in stack
    let x = 44;
    let y = x; 
    // you can call x, y values with no issues 
    println!("int values: x:{}, y: {}", x, y);

    // to clone values stored in heap
    let s3 = String::from("some string 3");
    let s4 = s3.clone();
    println!("strings copied : s3: {}, s4: {}", s3, s4);

    let str1 = String::from("3333");
    taking_ownership(str1);
    // cannot use after passed to a function
    // println!("printing after used in some function, {}", str1);

    let x = 44;
    makes_copy(x);
    // here x is callable because it is copied when passed to a function
    println!("x after returning back passed to a function, {}", x);

    let s1 = gives_ownership();
    println!("str: s1: {}", s1);
    let s2 = String::from("some string");
    let s3 = takes_and_gives_back(s2);
    // println!("s2: {}", s2); // this won't work
    println!("s3: {}", s3);

    // getting and passing the ownership : tedious
    let (s3, len) = calculate_length_ownership(s3);
    println!("can use s3 now: {}, len: {}", s3, len);

    // references comes to rescue 
    let s4 = String::from("some string");
    let len = calculate_length_reference(&s4);
    println!("can use s4 now: s4 {}, len: {}", s4, len);

    // modify passed value with reference 
    let mut s5 = String::from("some string ");
    // &mut s5: mutable reference
    modify_string(&mut s5);
    println!("string : modified {}", s5);

    // only one mutable references in a scope for variable 
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // shows error here
    modify_string(r1);


    // another mutable can be created in a scope
    let mut s = String::from("hello");
    {
       let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;

    // multiple refernces can be created but not mutable refernces
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // modify_string(r3);
    println!("{:p}, {:p}", r1, r2); // prints references

    // see the address reference between mutabale ref and normal ref
    let mut str1 = String::from("hello ss");
    let r1 = &str1;
    println!("normal reference address: {:p}", r1);
    {
        let mut_r1 = &mut str1;
        println!("mutable ref address: {:p}", mut_r1);
    }
    println!("both are same");


    // Dangling refernces : 
    // dangle_ref();


    // summary

    // At any given time, you can have either but not both of:
    // One mutable reference.
    // Any number of immutable references.
    // References must always be valid.

    // ***************************************
    //     String and Slices
    // ***************************************

    let st1 = String::from("0123456789");
    let str1 = &st1[0..3]; // 012
    let str2 = &st1[3..st1.len()]; // &st1[3..] 3456789
    // internally slice stores starting , ending indexes, len 
    println!("{}, {}", str1, str2);
    println!("{:p}, {:p}", str1, str2);

    // on emojis it will panic
    // let em_st1 = String::from("😀😀😀😀😀😀😀");
    // let em_str1 = &em_st1[0..3];
    // let em_str2 = &em_st1[3..em_st1.len()];
    // println!("{}, {}", em_str1, em_str2);
    // println!("{:p}, {:p}", em_str1, em_str2);

    // program to stripe words from string
    let some_string = String::from("hello world is not laugh going to work");
    let first_word = get_first_word(&some_string);
    println!("first word: {}", first_word);

    let my_string = String::from("hello world");
    // first_word works on slices of `String`s
    let word = get_first_word(&my_string[..]);
    let my_string_literal = "hello world";
    // first_word works on slices of string literals
    let word = get_first_word(&my_string_literal[..]);
    // since string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = get_first_word(my_string_literal);


    // Array Slices 
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{:?}", &slice);


}

fn get_first_word(s: &str) -> &str {
    let bytes_array = s.as_bytes();
    println!("bytes_array: {:?}", bytes_array);

    for (index, &byte) in bytes_array.iter().enumerate() {
        if byte == b' ' {
            return &s[0..index];
        }
    }

    &s[..]
}


fn taking_ownership(someStr: String) {
    println!("string passed: {}", someStr);
}

fn makes_copy(num: i32) {
    println!("number passed: {}", num);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello ");
    some_string
}

fn takes_and_gives_back(someStr: String) -> String {
    someStr
}

fn calculate_length_ownership(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn calculate_length_reference(s: &String) -> usize {
    s.len()
}

fn modify_string(str: &mut String) {
    str.push_str("some more string");
}

// fn dangle_ref() -> &String {
//     let s = String::from("some");
//     &s
// }

