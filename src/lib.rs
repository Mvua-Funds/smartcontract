use campaign::Campaign;
use causes::Cause;
use donations::Donation;
use events::Event;
use partners::Partner;

use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  collections::{UnorderedMap, UnorderedSet},
  env,
  json_types::U128,
  near_bindgen,
  serde::{Deserialize, Serialize},
  AccountId, Timestamp,
};

use std::str::FromStr;

pub mod account;
pub mod campaign;
pub mod causes;
pub mod constants;
pub mod donations;
pub mod errors;
pub mod events;
pub mod fungibletoken;
pub mod partners;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
  pub address: String,
  pub name: String,
  pub symbol: String,
  pub icon: String,
  pub decimals: u8,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Response<T> {
  pub results: Vec<T>,
  pub count: u64,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractStats {
  pub causes: u64,
  pub events: u64,
  pub campaigns: u64,
  pub donations: u64,
  pub tokens: u64,
  pub partners: u64,
  pub total_usd: f64,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Contract {
  pub gurdians: UnorderedSet<AccountId>,
  pub running: bool,
  pub causes: UnorderedSet<Cause>, // causes TRee planting
  pub events: UnorderedMap<String, Event>,
  pub campaigns: UnorderedMap<String, Campaign>,
  pub donations: UnorderedSet<Donation>,
  pub tokens: UnorderedMap<String, TokenMetadata>,
  pub partners: UnorderedMap<String, Partner>,
  pub total_usd: f64,

  pub causes_count: u64,
  pub events_count: u64,
  pub campaigns_count: u64,
  pub donations_count: u64,
  pub tokens_count: u64,
  pub partners_count: u64,
}

impl Default for Contract {
  fn default() -> Self {
    Self {
      gurdians: UnorderedSet::new(b"g"),
      running: true,
      causes: UnorderedSet::new(b"c"),
      events: UnorderedMap::new(b"e"),
      campaigns: UnorderedMap::new(b"a"),
      donations: UnorderedSet::new(b"d"),
      tokens: UnorderedMap::new(b"t".to_vec()),
      partners: UnorderedMap::new(b"z".to_vec()),
      total_usd: 0.0,

      causes_count: 0,
      events_count: 0,
      campaigns_count: 0,
      donations_count: 0,
      tokens_count: 0,
      partners_count: 0,
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
      causes: UnorderedSet::new(b"c"),
      events: UnorderedMap::new(b"e"),
      campaigns: UnorderedMap::new(b"a"),
      donations: UnorderedSet::new(b"d"),
      tokens: UnorderedMap::new(b"t".to_vec()),
      partners: UnorderedMap::new(b"z".to_vec()),
      total_usd: 0.0,

      causes_count: 0,
      events_count: 0,
      campaigns_count: 0,
      donations_count: 0,
      tokens_count: 0,
      partners_count: 0,
    }
  }

  pub fn add_token(&mut self, token: String, metadata: TokenMetadata) {
    self.tokens.insert(&token, &metadata);
    self.tokens_count += 1;
  }

  pub fn get_tokens(&self) -> Vec<TokenMetadata> {
    self.tokens.values().collect()
  }

  pub fn get_token(&self, address: String) -> Option<TokenMetadata> {
    self.tokens.get(&address)
  }

  pub fn get_donations_stats(&self) -> ContractStats {
    let stats = ContractStats {
      causes: self.causes_count,
      events: self.events_count,
      campaigns: self.events_count,
      donations: self.donations_count,
      tokens: self.tokens_count,
      partners: self.partners_count,
      total_usd: self.total_usd,
    };

    return stats;
  }
}
