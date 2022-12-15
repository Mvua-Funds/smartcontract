

use crate::*;

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate="near_sdk::serde")]
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
        self.causes_count += 1;
    }

    pub fn get_causes(&self)->Vec<Cause>{
        self.causes.iter().collect()
    }
}