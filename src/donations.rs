

use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Donation {
    pub id: String,
    pub donor: AccountId,
    pub token: String,
    pub amount: u128, // Amount in token value
    pub amount_usd: f64, // Amount in USD
    pub event: Option<String>, // Any associated event ID
    pub campaign: Option<String>, // Any associated campaign ID
    pub created_at: Timestamp,
}


impl Donation{
    // pub fn new(){

    // }
}

#[near_bindgen]
impl Contract{
    pub fn create_donation(&mut self){
        // Store the donation details
    }

    pub fn near_donation(&mut self){
        // Register donations made in near
    }

    pub fn token_donation(&mut self){
        // Register donations made in other donations
    }
}