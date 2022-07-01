use _11_writing_automated_tests::adder;


#[cfg(test)]
pub mod added_test{
    use super::*;
    #[test]
    fn adder_test() {
        assert_eq!(4, adder::add_two_numbers(2, 2));
    }
}