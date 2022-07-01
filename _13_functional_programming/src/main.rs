use std::{thread, time::Duration};
use _13_functional_programming::Cache;



/**
 * 
 * Closures : |x| { body }, move, Fn, FnOnce, FnMut, 
 * Iterators: 
 */


fn generate_workout_2(intensity: u32, random_num: u32) {
    let mut expensive_calculation = Cache::new(|num| {
        println!("calculating slowly : gen3..");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("today do {} pushups: ", expensive_calculation.value(intensity));
        println!("next do {} situps: ", expensive_calculation.value(intensity));
    } else {
        if random_num == 3 {
            println!("take break today: remember to stay hydrated");
        } else {
            println!("today run for {} minutes", expensive_calculation.value(intensity));
        }

    }

}




fn generate_workout_1(intensity: u32, random_num: u32) {
    let expensive_fn = |num : u32| -> u32{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("today do {} pushups: ", simulate_expensive_computation(intensity));
        println!("next do {} situps: ", simulate_expensive_computation(intensity));
    } else {
        if random_num == 3 {
            println!("take break today: remember to stay hydrated");
        } else {
            println!("today run for {} minutes", simulate_expensive_computation(intensity));
        }

    }
}


fn generate_workout_0(intensity: u32, random_num: u32) {
    if intensity < 25 {
        println!("today do {} pushups: ", simulate_expensive_computation(intensity));
        println!("next do {} situps: ", simulate_expensive_computation(intensity));
    } else {
        if random_num == 3 {
            println!("take break today: remember to stay hydrated");
        } else {
            println!("today run for {} minutes", simulate_expensive_computation(intensity));
        }

    }
}


fn simulate_expensive_computation(intensity: u32) -> u32{
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
fn main() {

//   generate_workout_0(99, 33);
//   generate_workout_1(88, 99);

//   // closure 
//   let example_closure = |x| x;

//   let s = example_closure(String::from("hello"));
//   // let n = example_closure(5); // will give erros as it this the x in closure as string 

//   generate_workout_2(88, 77);
  

  // closures : capture data 
  let x = 4;
  let equal_to_x = |z| { z == x };
  let y = 4;
  assert!(equal_to_x(y));


  // cann't do the same with functions
  fn equal_to_x_fn(z: i32) -> bool {
     // z == x // you cannot do this
     true
  }

  // traits of clousre : Fn, FnMut, FnOnce

  // move keyword

  let some_vec = vec![1,2,3,4];
  let move_closure = | x | some_vec.contains(x);
  println!("you can print some_vec here {:?}", some_vec);
  let y = 4;
  println!("y is in some_vec: {}", move_closure(&y));


  // lets take the ownership of closure

  let some_vec = vec![1,2,3,4];
  let move_closure = move | x | some_vec.contains(x);
//   println!("you can print some_vec here {:?}", some_vec);
  let y = 4;
  println!("y is in some_vec: {}", move_closure(&y));


  // **************************************************
  // processing a series of items with Iterators : 
  // **************************************************

  let v1 = vec![1,2,3,4,5,6,7,8,9];
  let v1_iter = v1.iter();

  for x in v1_iter {
      println!("x : {}", x);
  }

  // the iterator trait and the next method

  pub trait Iterator1 {
      type Item;
      fn next(&mut self) -> Option<Self::Item>;
  }


}

