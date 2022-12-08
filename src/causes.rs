

use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Cause {
    pub title: String, // name or title of the course Food Security, water
}

impl Cause{
    
}

#[near_bindgen]
impl Contract{
    pub fn create_cause(&mut self, title: String){
        let cause = Cause{title};
        self.causes.insert(&cause);
    }
}