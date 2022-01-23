extern crate rand;

use std::{fs::{self, File}, io::{ self, Error, ErrorKind, Read}};
use std::net::IpAddr;
use std::cmp::Ordering;
use rand::Rng;

mod guess;

fn main() {
    // Error Handling 
    // Recoverable, unrecoverable
    // Result, panic!
    // file not found, array out of bound


    // Unrecoverable Errors with panic!
    // on panic -> program unwinds = rust back tracts its fn stack and clears data
    // or can be made to abort immediately and let os clear up data
    // to get small binary code switch to aborting from unwinding
    // [profile.release]
    // panic = "abort"
    // add above lines to cargo toml

    // panic!("crash and burn");

    let vec1 = vec![1,2,3,4,45];
    // vec1[99];

    //Using a panic! Backtrace

    // this command
    // RUST_BACKTRACE=1 cargo run



    // Recoverable Errors with Result

    // enum Result<T, E> { // T: success data type , E: error data type
    //     Ok(T),
    //     Err(E),
    // }

    let f1 = File::open("hello.txt");
    let file1 = match f1 {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                println!("file not found dude");
                match File::create("hello.txt") {
                    Ok(f) => f,
                    Err(e) => panic!("unable to create file: {}", e),
               }
            },
            other_err =>  panic!("problem opening file : {:?}", other_err),
        },
    };

    let f = File::open("hello.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|err| {
               panic!("problem creating file: {}", err);
            })
        } else {
            panic!("problem reading file: {}", err);
        }
    });

    // shortcut for panic on error : unwrap and expect

     

    // unwrap : unwraps the result Ok Data 
    // will panics if file is not available
    let f = File::open("hello.txt").unwrap();

    // expect : same as unwrap but we can add our panic message
    
    let f = File::open("hello.txt").expect("panic message: custome");

    // propagating errors :

    fn read_user_name_in_file(file_path: &str) -> Result<String, io::Error>{
        let f = File::open(file_path);
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e), // this will return out of function
        };
    
        let mut m = String::new();
        match f.read_to_string(&mut m) {
            Ok(_size) => {
                println!("string is : {}", m);
                Ok(m)
            },
            Err(e) => Err(e),
        }
    }

    let name = read_user_name_in_file("hello.txt")
                    .expect("reading user name failed");
   
    println!("name in file: {}", name);

    // A Shortcut for Propagating Errors: the ? Operator
    fn read_username_2(path: &str)-> Result<String, io::Error> {
        // ? operator is same as unwrap but doesnt panics
        // and if there is error transforms it to match return type error
        // Form Trait should be implemented in Returned error
        let mut f = File::open(path)?; 
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
    
    fn read_username_3(path: &str) -> Result<String, io::Error> {
        let mut s = String::new();
        File::open(path)?.read_to_string(&mut s)?;
        Ok(s)
    }

    fn read_username_4(path: &str) -> Result<String, io::Error> {
        fs::read_to_string(path)
    }

    // where can you use ? operator
    // let f = File::open("hello.txt")?; // only in a function that returns Result or Option

    // function returning option and using ? operator
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    fn returns_option(path: &str) -> Option<String> {
        let mut s = String::new();
        // you cannot use ? on functions that return results here
        // let x = File::open(path)?.read_to_string(&mut s)?;
        // Ok(s)
        None
    }

    // to convert Result to Option we have to use ok() method
    fn return_option() -> Option<String> {
        let mut s = String::new();
        let x = File::open("hello.txt").ok()?.read_to_string(&mut s).ok()?;
        Some(s)
    }

    // to convert Option to Result we have to use ok_or() method
    fn returns_result(text: &str) -> Result<String, io::Error> {
        let x = text.lines().next().ok_or(Error::new(ErrorKind::NotFound, "no next line"))?.to_string();
        Ok(x)
    }


    // main functions till now returned () empty tuple
    // c main functions return 0 on successfull run or other integers on failures
    // same conventions are followed in rust 
    // rust returns zero if main is successful and other integers on panics

    // we can also do this : 
    /*
    fn main() -> Result<(), Box<dyn Error>> {
      let f = File::open("hello.txt")?;
      Ok(())
    }
    Box<dyn Error> : means it can return any type of error
    */

    // to panic or not to panic
    // use result when ever you define a function that might fail
    // it depends

    // some times it is better to panic
    // Examples, Prototype Code, and Tests
    // prototype code : unwrap and expect are handy
    // test cases: better to panic on error

    // Cases in Which You Have More Information Than the Compiler

    let home  = "127.0.0.1".parse::<IpAddr>().unwrap();

    // here Ip address is valid one we know that so we can unwrap here without worrying

    // Guidelines for Error handling : 
    // when ever you code end up in a bad state,
    // bad state = when some assumption, guarantee, contract or invariant is broken 
    // and invalid values, contradictory values or missing values are passed

    // when failure is expected best to use Result
    // panic when contract is voilated

    // use rust type system to minimise error checkings
    // ex: values passed into function if they have types other than Option can never be empty
    // rust won't even compile if one wants to pass values other than required in that case

    fn guessing_game() {
        println!("guess a number between 1 to 100");
        let mut number = rand::thread_rng().gen_range(1, 100);
        loop {
            println!("input the number: ");
            let mut s = String::new();
            io::stdin().read_line(&mut s).expect("unable to read input");
            let x = match s.trim().parse::<i32>() {
                Ok(val) => val,
                Err(e) => {
                    println!("invalid input: unable to parse");
                    continue;
                }
            };
            if x < 1 || x > 100 {
                println!("The secret number will be between 1 and 100.");
                continue;
            }
            match x.cmp(&number) {
                Ordering::Equal => {
                    println!("you guessed it right");
                    break;
                },
                Ordering::Greater => println!("little high"),
                Ordering::Less => println!("little low"),
            }
        }
    }

    // guessing_game();

    // another better way : 

    fn guessing_game2() {
        println!("guess a number between 1 to 100");
        let mut number = rand::thread_rng().gen_range(1, 100);
        loop {
            println!("input the number: ");
            let mut s = String::new();
            io::stdin().read_line(&mut s).expect("unable to read input");
            let x = match s.trim().parse::<guess::Guess>() {
                Ok(val) => val,
                Err(e) => {
                    println!("invalid input: unable to parse");
                    continue;
                }
            };
            match x.value().cmp(&number) {
                Ordering::Equal => {
                    println!("you guessed it right");
                    break;
                },
                Ordering::Greater => println!("little high"),
                Ordering::Less => println!("little low"),
            }
        }
    }

    guessing_game2();
    




}




