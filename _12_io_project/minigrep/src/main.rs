use std::{env};
use std::process;
use minigrep::Config;



fn main() {

    // aim to make grep : globally searh a regular expression and print
    // grep input : file name and string 
    // ripgrep (grep in rust)

    // to use the following concepts we learned : 
    /*
      Our grep project will combine a number of concepts you’ve learned so far:
        Organizing code (using what you learned about modules in Chapter 7)
        Using vectors and strings (collections, Chapter 8)
        Handling errors (Chapter 9)
        Using traits and lifetimes where appropriate (Chapter 10)
        Writing tests (Chapter 11)
    */

    // commandline : cargo run searchstring filepath.txt 
    
    // let args: Vec<String> = env::args().collect();
    // println!("args: {:?}", args); // ["target/debug/minigrep", "hi", "hello.txt"]

    // use OsString if inputs contain invalid unicodes : use::std::env::args_os;
    // let slis = args.as_slice();
    // match slis {
    //   [ _, search_key, filepath] => {
    //     println!("{} , {}", search_key, filepath );
    //   },
    //   _ => {

    //   }
    // }
    
    // let search_key = &args[1];
    // let file_path = &args[2];
    // let contents = fs::read_to_string(file_path).expect("some thing wrong");
    // println!("with text: {}", contents);


    /*
    maintaining large code : 
      Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
      As long as your command line parsing logic is small, it can remain in main.rs.
      When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.
    */

    /*
     Responsibilities of main in this program: 
      Calling the command line parsing logic with the argument values
      Setting up any other configuration
      Calling a run function in lib.rs
      Handling the error if run returns an error
     */

     /*
      This TTD software development technique follows these steps:

        Write a test that fails and run it to make sure it fails for the reason you expect.
        Write or modify just enough code to make the new test pass.
        Refactor the code you just added or changed and make sure the tests continue to pass.
        Repeat from step 1!
     
     */

     let args: Vec<String> = env::args().collect();
     let config = Config::new(&args).unwrap_or_else(|err| {
       eprintln!("problem parsing given arguments, {}", err);
       process::exit(1);
     });
     if let Err(e) = minigrep::run(config) {
       eprintln!("application error: {}", e);
       process::exit(1);
     }
     
}
