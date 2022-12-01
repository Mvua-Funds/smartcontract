use campaign::Campaign;
use courses::Course;
use donations::Donation;
use events::Event;
use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  collections::{UnorderedSet, UnorderedMap},
  env,
  json_types::U128,
  near_bindgen,
  serde::{Deserialize, Serialize},
  AccountId, Timestamp,
};

pub mod account;
pub mod campaign;
pub mod constants;
pub mod courses;
pub mod donations;
pub mod errors;
pub mod events;
pub mod fungibletoken;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
  pub address: AccountId,
  pub name: String,
  pub symbol: String,
  pub icon: String,
  pub decimals: u8,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Contract {
  pub gurdians: UnorderedSet<AccountId>,
  pub running: bool,
  pub courses: UnorderedSet<Course>, // causes
  pub events: UnorderedSet<Event>,
  pub campaigns: UnorderedSet<Campaign>,
  pub donations: UnorderedSet<Donation>,
  pub tokens: UnorderedMap<AccountId, TokenMetadata>,
}

impl Default for Contract {
  fn default() -> Self {
    Self {
      gurdians: UnorderedSet::new(b"g"),
      running: true,
      courses: UnorderedSet::new(b"c"),
      events: UnorderedSet::new(b"e"),
      campaigns: UnorderedSet::new(b"a"),
      donations: UnorderedSet::new(b"d"),
      tokens: UnorderedMap::new(b"t".to_vec()),
    }
  }
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new() -> Self {
    Self {
      gurdians: UnorderedSet::new(b"g"),
      running: true,
      courses: UnorderedSet::new(b"c"),
      events: UnorderedSet::new(b"e"),
      campaigns: UnorderedSet::new(b"a"),
      donations: UnorderedSet::new(b"d"),
      tokens: UnorderedMap::new(b"t".to_vec()),
    }
  }

  pub fn add_token(&mut self, token: AccountId, metadata: TokenMetadata) {
    self.tokens.insert(&token, &metadata);
  }
}
