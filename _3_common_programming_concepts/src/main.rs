
// declaring constants
const MAX_VALUES: u32 = 445;
fn main() {
    // declaring variables
    let x = 44;
    println!("value of x: {}", x);

    // declaring mutable variables
    let mut y = 44;
    println!("value of y: {}", y);
    y = 44; 
    println!("value of y: {}", y);
    y = 45; 
    println!("value of y: {}", y);

    // using declared constants
    println!("MAX_VALUES of x can be is : {}", MAX_VALUES);

    // shadowing 
    let m = 55;
    println!("value of m : {}", m);
    let m = "some string";
    println!("value of m after shadowing: {}", m);
    let m = m.len();
    println!("value of m after shadowing again: {}", m);
    let mx = 444;
    let mx = mx * 33;
    let mx = mx * (mx/ 44);
    println!("value of mx: {}", mx);

 // Data types : 
    //scalar datatypes (4) : integers, floating-point numbers, Booleans, characters

      // (1) integers : 
        // signed : i8, i16, i32, i64, : -2^(n-1) to 2^(n-1) - 1 
        // unsigned:  u8, u16, u32, u64 : 0 to 2^(n) - 1
        // isize, usize : size based on os architecture 
        // literals : Decimal : 1_000, Hex: 0xff, Octal: 0o77, Binary: 0b1111_0000, Byte: b'A'
     let x_signed:i8 = 9;      println!("x signed i8 {}", x_signed);
     let x_signed:i16 = 5555;      println!("x signed i16 {}", x_signed);
     let x_signed:i32 = 888888;      println!("x signed i32 {}", x_signed);
     let x_signed: i64 = -8999393939393;      println!("x signed i64 {}", x_signed);

     let x_unsigned: u32 = 898989;
     println!("x unsigned u32 {}", x_unsigned);
     let x_decimal = 1_0000;
     println!("x_decimal {}", x_decimal);
     let x_hex = 0xffff;
     println!("x_hes {}", x_hex);
     let x_octal = 0o444;
     println!("x_octal {}", x_octal);
     let x_binary = 0b11011001010;
     println!("x_binary {}", x_binary);
     let x_byte = b'R';
     println!("x_byte {}", x_byte);


    // floating point numbers: f32, f64
    let x_float32: f32 = 88.00; println!("x_float32 {}", x_float32);
    let x_float64: f64 = 6666.000; println!("x_float64 {}", x_float64);


    // Mathematical operations : + - * %
    let sum = 44 + 88; println!("sum {}", sum);
    let difference = 88 - 99; println!("difference {}", difference);
    let multiplication = 88 * 44; println!("multiplication {}", multiplication);
    let division = 44 / 33; println!("division {}", division);
    let reminder = 55 % 33; println!("reminder {}", reminder);

    // Boolean 
    let is_fine: bool = true; println!("isFine {}", is_fine);
    let is_bad: bool = false; println!("isBad {}", is_bad);

    // Character types : use singel quotes
    let some_char: char = 's'; println!("some char {}", some_char);
    let heart_eyed_cat = '😻'; println!("heart_eyed_cat {}", heart_eyed_cat);


  // compound types : tuples and arrays

    // tuples : 
    let tup: (i32, u64, &str, char) = (-33, 555, "some string", '😻');
    println!("tuple : {:?}", &tup);
    let (x, y, z, w) = tup;
    println!("tuple destructured: {}, {}, {}, {}", x, y, z, w);
    println!("access values without destructuring: {}, {}, {}, {}", tup.0, tup.1, tup.2, tup.3);

    // arrays :
    let numbers_array = [1,2,3,4,5,6]; println!("numbers_array: {:?} ", numbers_array);
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let first_month_name = months[0];
    println!("first_month_name: {}", first_month_name);
    // let some_month_index = 14; // out of bounds
    // println!("out of bound {}", months[some_month_index]);


    // functions
    simple_function(44);
    block_assignment();
    return_five();
    let x = 88;
    println!("next value of x: {}", increment(x));
    println!("previous value of x: {}", decreament(x));
    println!("empty funct returns tuple() : {:?}", empty_function());
    
    // comments: 
    // only // is used for both single line and muliline comments ```no /** */```
    /* single line , but doens't work for multiline */

    // multiline 
    // line 1
    // line 2

    // control flow : if, if else elseif ladders

    // if without brackets
    let x = 44;
    if x % 2 == 0 {
      println!("x is a even number");
    }

    // if with brackets
    let y = 23;
    if (y % 2 != 0) {
      println!("y is an odd number");
    }

    // if else 
    let some_condition = false;
    if some_condition {
      println!("if block/arm");
    } else {
      println!("else block/arm");
    }

    // if elseif 

    let x = 4;
    if x < 4 {
      println!("x is less than 4");
    } else if x > 4 {
      println!("x is greater than 4");
    } else {
      println!("x should be equal to 4")
    }

    // arm returns in if else / else if blocks
    let some_condition = true;
    let x = if some_condition {
      5 
    } else {
      8
      // if you return other than i32 it will fail
     // "six"
    };

    println!("value of x after conditional setting: {}", x);


    // Repeating code: loop, while, for

    // for ever loops 
    // loop {
    //   println!("loops continue");
    // }
    let mut x = 0;
    loop {
      if x > 5 {
        break;
      }
      println!("loop: hello");
      x = x + 1; 
    }
    // while 
    let mut number = 4;
    while number > 0 {
      println!("number: {}", number);
      number = number - 1;
    }
    // looping array : while
    let arr = [1,2,3,4,5];
    let mut index = 0;
    while index < arr.len() {
      println!("arr[{}] : {}",index, arr[index]);
      index = index + 1;
    }
    // the above is not good method for accessing array elements:
    // 1. slow due to conditional check,
    // 2.index out of bound will happen if not careful

    // a much more good method:
    
    for ele in arr.iter() {
      println!("arr element: {}", ele);
    }

    // a good method for iterating n times instead of while

    for num in (1..4).rev() {
      println!("number: {}", num);
    }

    // let mut stringl = "some string";
    // stringl = stringl + "333";
    // println!("strings {}", stringl);
  
}



fn simple_function(x: i32) {
  println!("the value of x: {}", x);
}

fn block_assignment() {
  let x = 5;
    // here y value is x + 1
    let y = {
        let x = x + 3;
        x + 1
    };

    println!("The value of y is: {:?}", y);
}

fn return_five() -> i32{
  5
}

fn increment(x: i32) -> i32 {
  x + 1
}

fn decreament(x: i32) -> i32 {
  return x - 1;
}

fn empty_function() {
  // empty function returns a tuple back ()
}
