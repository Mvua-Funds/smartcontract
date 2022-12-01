use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  collections::UnorderedSet,
};

use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct EventType {
  pub name: String, //Online event, physical
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Event {
  pub id: String,
  pub title: String,
  pub created_by: AccountId,
  pub managers: UnorderedSet<AccountId>,
  pub course: String, // Campaign course ie food security, water, tree planting, etc
  pub date: Timestamp,
  pub description: String,
  pub target: u128,  // Campaign target amount
  pub token: String, // The targeted token
  // pub attending: u64, // NO of people who have said they are attending
  pub attendees: UnorderedSet<AccountId>, // Those attending
  pub venue: Option<String>,              // venue of the event
  pub event_type: String,
  pub channel: Option<String>, //Facebook, twitter spaces, youtube, google meet, etc
  pub channel_url: Option<String>,
  pub created_on: Timestamp,
}

impl Event {
  pub fn new(
    id: String,
    title: String,
    date: Timestamp,
    course: String,
    description: String,
    target: U128,
    token: String,
    venue: String, 
    event_type: String, 
    channel: String, 
    channel_url: String,
  ) -> Self {
    let created_by = env::predecessor_account_id();
    Self {
      id,
      title,
      created_by,
      managers: UnorderedSet::new(b"m"),
      course,
      date,
      description,
      target: u128::from(target),
      token,
      attendees: UnorderedSet::new(b"a"),
      venue: Some(venue),
      event_type,
      channel: Some(channel),
      channel_url: Some(channel_url),
      created_on: env::block_timestamp(),
    }
  }
}

#[near_bindgen]
impl Contract {
    pub fn create_event(&mut self){

    }
}
