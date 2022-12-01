use near_sdk::collections::UnorderedSet;

use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Campaign {
  pub id: String,
  pub created_by: AccountId,
  pub managers: UnorderedSet<AccountId>,
  pub title: String,
  pub course: String, // Campaign course ie food security, water, tree planting, etc
  pub start_date: Timestamp,
  pub end_date: Timestamp,
  pub description: String,
  pub target: u128,  // Campaign target amount
  pub token: String, // The targeted token
  pub created_on: Timestamp,
}

impl Campaign {
  pub fn new(
    id: String,
    title: String,
    course: String,
    start_date: Timestamp,
    end_date: Timestamp,
    description: String,
    target: U128,
    token: String,
  ) -> Self {
    let created_by = env::predecessor_account_id();
    Self {
      id,
      created_by,
      managers: UnorderedSet::new(b"m"),
      title,
      course,
      start_date,
      end_date,
      description,
      target: u128::from(target),
      token,
      created_on: env::block_timestamp(),
    }
  }
}

#[near_bindgen]
impl Contract {
    pub fn create_campaign(&mut self){

    }
}
