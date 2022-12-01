

use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Course {
    pub title: String, // name or title of the course Food Security, water
}

impl Course{
    
}

#[near_bindgen]
impl Contract{
    pub fn create_course(&mut self, title: String){
        let course = Course{title};
        self.courses.insert(&course);
    }
}