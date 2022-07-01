use std::{collections::HashMap, hash::Hash};


pub struct Cache<F, I, O>
  where
    F: Fn(I) -> O,
    I: Eq + Hash +Clone,
    O: Clone {
    calculation: F,
    values: HashMap<I, O>,
}

impl<F, I, O> Cache<F, I, O>
where
F: Fn(I) -> O,
I: Eq + Hash +Clone,
O: Clone {

   pub fn new(calculation: F) -> Cache<F, I, O> {
        Cache {
            calculation,
            values: HashMap::new(),
        }
    }

   pub fn value(&mut self, arg: I) -> &O {
       self.values.entry(arg.clone())
       .or_insert((self.calculation)(arg))
    }
}

#[test]
fn test_multiple_calls() {
    let mut something = Cache::new(|x| x * 2);
    assert_eq!(2, *something.value(1));
    assert_eq!(4, *something.value(2));
}
