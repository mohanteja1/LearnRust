fn main() {

    // how to write tests : 
    // 1. setup any need data or state
    // 2. run the code you want to test
    // 3. assert the results are what you expect

    // test attribute, should_panic and other few macros

    // the anatomy of test function 
    // check lib.rs file

    // commands 
    // cargo test        : run all tests
    // cargo test --help : lists all commands

    // tests run in threads
    // cargo test -- --test-threads=1 : forcing to use single thread
    // cargo test -- --show-output    : prints all the printlns in console
    // cargo test test_function_name  :  to run test functions containing that name

    // #[ignore] to ignore test cases

    // cargo test -- --ignored      : to run ignored

    
}
