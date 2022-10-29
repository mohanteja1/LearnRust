pub mod shoes {
 pub struct Shoes;
}

#[derive(PartialEq, Debug)]
pub struct Shoes {
    pub size: u32,
    pub style: String,
}

pub fn shoes_in_size(shoes: Vec<Shoes>, shoe_size: u32 ) -> Vec<Shoes> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}