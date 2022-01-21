fn print_hello_world() {
    print!("Hello, World");
}

fn comments() {
        // This is an example of a line comment
        // There are two slashes at the beginning of the line
        // And nothing written inside these will be read by the compiler
    
        // println!("Hello, world!");
    
        // Run it. See? Now try deleting the two slashes, and run it again.
    
        /* 
         * This is another type of comment, a block comment. In general,
         * line comments are the recommended comment style. But
         * block comments are extremely useful for temporarily disabling
         * chunks of code. /* Block comments can be /* nested, */ */
         * so it takes only a few keystrokes to comment out everything
         * in this main() function. /*/*/* Try it yourself! */*/*/
         */
    
        /*
        Note: The previous column of `*` was entirely for style. There's
        no actual need for it.
        */
    
        // You can manipulate expressions more easily with block comments
        // than with line comments. Try deleting the comment delimiters
        // to change the result:
        let x = 5 + /* 90 + */ 5;
        println!("Is `x` 10 or 100? x = {}", x);
}


fn formatted_print() {
        // In general, the `{}` will be automatically replaced with any
        // arguments. These will be stringified.
        println!("{} days", 31);
    
        // Without a suffix, 31 becomes an i32. You can change what type 31 is
        // by providing a suffix. The number 31i64 for example has the type i64.
    
        // There are various optional patterns this works with. Positional
        // arguments can be used.
        println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    
        // As can named arguments.
        println!("{subject} {verb} {object}",
                 object="the lazy dog",
                 subject="the quick brown fox",
                 verb="jumps over");
    
        // Special formatting can be specified after a `:`.
        println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    
        // You can right-align text with a specified width. This will output
        // "     1". 5 white spaces and a "1".
        println!("{number:>width$}", number=1, width=6);
    
        // You can pad numbers with extra zeroes. This will output "000001".
        println!("{number:0>width$}", number=1, width=6);
    
        // Rust even checks to make sure the correct number of arguments are
        // used.
        // println!("My name is {0}, {1} {0}", "Bond");
        // FIXME ^ Add the missing argument: "James"
    
        // Create a structure named `Structure` which contains an `i32`.
        #[allow(dead_code)]
        struct Structure(i32);
    
        // However, custom types such as this structure require more complicated
        // handling. This will not work.
        // println!("This struct `{}` won't print...", Structure(3));
        // FIXME ^ Comment out this line.
    
}

fn main() {
    print_hello_world();
    comments();
    formatted_print();

}