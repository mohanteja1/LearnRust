use std::collections::HashMap;
use rand::Rng;
use _7_packages_modules_crates::{eat_at_house, eat_at_restaurant_fourth_time};


// use std::cmp::Ordering;
// use std::io;

// use std::{io, cmp::Ordering}; //in single line

// use std::io;
// use std::io::Write;
use std::io::{self, Write};

// to bring all public items defined in a path into scope : use *
use std::collections::*;

fn main() {

    // Packages: A Cargo feature that lets you build, test, and share crates
    // Crates: A tree of modules that produces a library or executable
    // Modules and use: Let you control the organization, scope, and privacy of paths
    // Paths: A way of naming an item, such as a struct, function, or module



    // packages and crates : 

    // a crate groups all the functionality under a single scope so the functionality is easy to share
    // a package has a toml file to generate the crates, it may contain one or more crates

    // Defining Modules to control Scope and Privacy
    // keywords: use, pub, as
    // modules lets us organize our code within the crates for readability and easy reuse.

    // check src/lib.rs file
    /*
        crate
            └── front_of_house
                ├── hosting
                │   ├── add_to_waitlist
                │   └── seat_at_table
                └── serving
                    ├── take_order
                    ├── serve_order
                    └── take_payment
     */
    // https://stackoverflow.com/questions/26946646/rust-package-with-both-a-library-and-a-binary

    // path for referring an item in the module tree
    // keywords: crate, self, super
    // absolute path: 
    // relative path:
    // to build: `cargo build` instead of `cargo run`
    // crate::front_of_house::hosting::add_to_waitlist();

    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("map: {:?}", map);

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("secret number: {}", secret_number);
    eat_at_house();
    // when a package contains lib and main crates : https://stackoverflow.com/questions/57756927/rust-modules-confusion-when-there-is-main-rs-and-lib-rs
    eat_at_restaurant_fourth_time();

}
