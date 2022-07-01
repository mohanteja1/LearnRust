use std::{collections::HashMap, vec};

fn main() {
    // vector, string, hashmap

    // vector allows you to store variable number of values next to each other

    let mut int_vector: Vec<i32> = Vec::new();
    int_vector.push(99);
    println!("int vector: {:?}", int_vector);

    // vec with initiailization in single line
    
    let v1: Vec<i32> = vec![12, 33, 55];
    println!("vec1 : {:?}", v1);

    // modifying vector 
    let mut v = Vec::new();
    v.push(44);
    v.push(333);
    v.push(3333);
    println!("vector: {:?}", v);
    // droping vector : freeing it : context change will free it
    {
        let v1 = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope and is freed here

    // reading elements in vector: 
    let v = vec![1,2,3,4,5,5];
    let third_element: &i32 = &v[2];
    println!("third element {:?}",third_element);

    match v.get(2) {
        Some(element) => {
            println!("element is : {}", element);
        },
        None => {
            println!("there is no third element");
        }
    }
    let val: Option<&i32> = v.get(3); // it returns optional

    // out of bound index;
    let v2 = vec![1,2,3,4,5];
    // let nope1 = &v[100];// will panic
    let option_nope = v.get(100);


    // you cannot push to the vector when you hold a immutable reference to elements
    let mut v3 = vec![1,23,4,444,4,4];
    let second: &i32 = &v3[1];
    println!("second value {}", second);
    // v3.push(33); // will cause error
    println!("v3: {:?}", v3);
    println!("second value {}", second);

    // iterating over vector : 
    let mut v = vec![1,2,3,4,5,5];
    for i in &v {
        println!("value: {}", i);
    }

    for i in &mut v {
        *i += 44; // deferencing operator
    }

    // Using an Enum to Store Multiple Types

    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let v4 = vec![
        SpreadSheetCell::Int(33),
        SpreadSheetCell::Float(44.3),
        SpreadSheetCell::Text(String::from("somstring")),
        SpreadSheetCell::Int(33)
        ];
    println!("vector with different types: {:?}", v4);
    let val1 = &v4[0];
    match val1 {
        SpreadSheetCell::Float(val) => println!("val : {}", &val),
        SpreadSheetCell::Int(val) => println!("val : {}", &val),
        SpreadSheetCell::Text(val) => println!("val : {}", &val),
    }

    // more here : https://doc.rust-lang.org/std/vec/struct.Vec.html

    // string : collection of characters (bytes)
    // String (string) : owned variant
    // &str (string slice) : borrowed variant
    // OsString, OsStr, CString, CStr

    let mut s: String = String::from("some string");
    let str1: &str = "initial data";
    let s2: String = str1.to_string();

    // utf-8 encoded: any character : all the below are valid string values
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // updating string: 
    let mut s = String::from("value");
    s.push_str(" :hello");
    println!("s: {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1: {}", s1);

    //push : takes a single character

    let mut s1 = String::from("lo");
    s1.push('l');
    let ch1: char = 'e';
    s1.push(ch1);
    println!("s1: {}", s1);

    // Concatenation with the + Operator or the format! Macro
    let s1 = String::from("Hello ");
    let s2 = String::from("world");
    let s3 = s1 + &s2; // you cannot use s1 after this as it is moved to s3
    // + is simply : fn add(self, s: &str) -> String { }
    // but s2 is not &str it is &String how is this addition possible as add accepts &str
    // deref coercion: &String is converted to &str using &s2[..]
    let s4: &str = &s2[..];

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s: {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s: {}", s);

    // indexing strings; 
    let s1 = String::from("hello");
    // let ch = s1[0]; // this won't work
    let s2 = "hello";
    // let ch = &s2[3]; // this won't work

    // bytes, scalar values, grapheme clusters
    // letters
    //  word “नमस्ते”
    // in bytes u8 values: [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135] 
    // in unicode scalar values: ['न', 'म', 'स', '्', 'त', 'े']
    // in grapheme cluster values: ["न", "म", "स्", "ते"]
    // so index won't give correct value and also indexing doesn't know which value to return
    // even if indexed it won't give O(1)
    
    let hello = "Здравствуйте";

    let s = &hello[0..4]; // will try to get first 4 bytes
    println!("s: {}", s); // Зд = 4 bytes
    // let s = &hello[0..1]; // will panic


    // Methods for Iterating Over Strings

    // to get the characters
    for c in  "नमस्ते".chars() {
        println!("{}", c);
    } // न म स  ् त  े = 6 chars

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }



    // hashmap: helps in associating a value with a key
    let mut team_scores = HashMap::new();
    team_scores.insert(String::from("blue"), 88);
    team_scores.insert(String::from("green"), 98);
    // all keys must be same type and all values must be same type
    println!("hashmap: {:?}", team_scores);

    // zip together two vectors into hashmap
    let teams = vec![String::from("blue"), String::from("green")];
    let scores = vec![11, 13];
    let team_scores: HashMap<_,_>= teams.into_iter().zip(scores.into_iter()).collect();
    println!("hashmap: {:?}", team_scores);
    // rust doesn't know which type is returned until collect ran, so we need to specify the types

    // hashmaps and ownership: 
    let field_name = String::from("some name"); // strings has move trait
    let field_value = 999; // i32 has copy trait

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name cannot be used as hashmap is the owner now and strings are moved
    // field_value can be used as i32 values are copied 
    // println!("field name: {}", field_name);
    println!("filed value: {}", field_value);

    //Accessing Values in a Hash Map

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("blue team"), 888);
    scores.insert(String::from("gream team"), 8777);
    let teamname = String::from("blue team");
    let score = scores.get(&teamname);
    if let Some(val) = score {
        println!("value is : {}", val);
    } else {
        println!("no value found");
    }

    // iterating values : 
    for (k, v) in &scores {
        println!("key : {}, value : {}", k, v);
    }

    //updating hashmap: 
     // over writing the values

     let mut scores = HashMap::new();
     scores.insert(String::from("blue"), 44);
     scores.insert(String::from("blue"), 66);
     println!("scores: {:?}", scores);

     // only inserting a value if there is no key value

     scores.insert(String::from("red"), 88);
     // will be inserted as it is not available before
     scores.entry(String::from("yellow")).or_insert(999);
     // won't be inserted 
     scores.entry(String::from("blue")).or_insert(9999);
     println!("{:?}", scores);
     let emptyEntry = scores.entry(String::from("green"));
    //  let valueEntry = scores.entry(String::from("blue")); // second mutable reference won't allows it
     println!("{:?}", emptyEntry);
     let emptyEntry = 88;
     let valueEntry = scores.entry(String::from("blue"));
     println!("{:?}", valueEntry);

     let text = "hello world in a new rust world";
     let mut map = HashMap::new();
     for word in text.split(" ").into_iter() {
         let count = map.entry(word).or_insert(0);
         *count += 1;
     }
     println!(" word count : {:?}", map);

     // hashmap in rust uses siphash for creating hash which takes time but secure
     // we can specify our own hash or other hashe available in crates.io


     // Exercises : : : : : : : : : : : ; : : ; : : ; ; ; ; 

     /*
        Given a list of integers, use a vector and return the median (when sorted,
        the value in the middle position) and mode (the value that occurs most often;
        a hash map will be helpful here) of the list.
      
        Convert strings to pig latin. The first consonant of each word is moved to the end
        of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with
        a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
        Keep in mind the details about UTF-8 encoding!


        Using a hash map and vectors, create a text interface to allow a user to add
        employee names to a department in a company. For example, “Add Sally to Engineering”
        or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department
        or all people in the company by department, sorted alphabetically.
     
     */


     /*
      1. Given a list of integers, use a vector and return the median (when sorted,
         the value in the middle position) and mode (the value that occurs most often;
         a hash map will be helpful here) of the list.
     */
     
     let integers: Vec<i32> = vec![44,55,6,66,7,777,77,66,55,44,55,11,22,33,444,34,54,9, 20, 20];
     println!("median: {}", find_median(&mut (integers.clone())));
     println!("original vec: {:?}", integers);
     print!("mode: {}", find_mode(&mut (integers.clone())));

     /*
      Convert strings to pig latin. The first consonant of each word is moved to the end
      of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with
      a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
      Keep in mind the details about UTF-8 encoding!
     */

     println!("pig latin: {} = {}", "first", to_pig_latin(String::from("first")));
     println!("pig latin: {} = {}", "apple", to_pig_latin(String::from("apple")));
     println!("pig latin: {} = {}", "elephant", to_pig_latin(String::from("elephant")));
     println!("pig latin: {} = {}", "door", to_pig_latin(String::from("door")));

     /*
        Using a hash map and vectors, create a text interface to allow a user to add
        employee names to a department in a company. For example, “Add Sally to Engineering”
        or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department
        or all people in the company by department, sorted alphabetically.
      */
      department_console_app();
}

fn find_median(numbers: &mut Vec<i32> ) -> i32 {
   numbers.sort();
   if numbers.len() % 2 == 0 {
     numbers[numbers.len()/2]
   } else {
     numbers[numbers.len()/2 + 1]
   }
}

fn find_mode(numbers: &mut Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for number in numbers {
        let count = map.entry(*number).or_insert(0);
        *count += 1;
    }
    let mut currentHighest = 0;
    let mut higestCount = 0;
    for (x , y) in map {
       if y > higestCount {
         currentHighest = x;
         higestCount = y;
       }
    }
    return currentHighest;
}

fn to_pig_latin(in_str: String) -> String {
    let first_char = in_str.chars().next();
    if let Some(c) = first_char  {
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            format!("{}-{}", in_str, "hay")
        } else {
            if let Some(rem) = in_str.strip_prefix(c) {
                format!("{}-{}{}", rem, c, "ay")
            } else {
                in_str.to_string()
            }
        }
    } else {
        in_str.to_string()
    }
}

fn department_console_app() {
    let mut dep_people: HashMap<String, Vec<String>> = HashMap::new();
    let mut exit = false;
    while !exit {
        println!("add xperson to xdep || print || exit");
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => {
                if line.trim().eq_ignore_ascii_case("exit") {
                    println!("bye...");
                    exit = true;
                } else if line.trim().eq_ignore_ascii_case("print") {
                    println!("department and peoples");
                    println!("{:?}", dep_people);
                } else {
                    let words: Vec<_> = line.split(" ").collect();
                    if words.len() < 4
                    || !words[0].eq_ignore_ascii_case("add")
                    || !words[2].eq_ignore_ascii_case("to") {
                        println!("to add: ex: Add Sally to Engineering")
                    }
                    let mut people = dep_people
                                    .entry(words[3].trim().to_string())
                                    .or_insert(vec![]);
                    people.push(words[1].to_string());
                }
            },
            Err(err) => {
                println!("error: {:?} \n exiting...", err);
                exit = true;
            }
        }
    }
}