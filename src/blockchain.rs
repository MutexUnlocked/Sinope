use std::cell::RefCell;

pub struct Blockchain {
    vec: RefCell<Vec<i32>>,
}

impl Blockchain {
    pub fn new() -> Self{
       Blockchain {vec: RefCell::new(Vec::new())}
    }

    pub fn add(&self, value: i32){
        self.vec.borrow_mut().push(value);
    }
}