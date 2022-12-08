use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Donation {
  pub id: String,
  pub donor: AccountId,
  pub token: String,
  pub amount: u128,             // Amount in token value
  pub amount_usd: f64,          // Amount in USD
  pub target: String,           // Target - is either; general, event, or campaign
  pub event: Option<String>,    // Any associated event ID
  pub campaign: Option<String>, // Any associated campaign ID
  pub created_at: Timestamp,
}

impl Donation {
  pub fn new(
    id: String,
    donor: AccountId,
    token: String,
    amount: U128,
    amount_usd: f64,
    target: String,
    event: Option<String>,
    campaign: Option<String>,
  ) -> Self {
    Self {
      id,
      donor,
      token,
      amount: u128::from(amount),
      amount_usd,
      target,
      event,
      campaign,
      created_at: env::block_timestamp(),
    }
  }
}

#[near_bindgen]
impl Contract {
  pub fn create_donation(
    &mut self,
    id: String,
    donor: AccountId,
    token: String,
    amount: U128,
    amount_usd: f64,
    target: String,
    event: String,
    campaign: String,
  ) {
    let mut cid = Some(campaign.clone());
    if campaign == "null" {
      cid = None
    }
    let mut eid = Some(event.clone());
    if event == "null" {
      eid = None
    }
    let donation = Donation::new(id, donor, token, amount, amount_usd, target, eid, cid);
    self.donations.insert(&donation);
  }

  pub fn near_donation(
    &mut self,
    id: String,
    token: String,
    amount: U128,
    amount_usd: f64,
    target: String,
    event: String,
    campaign: String,
  ) {
    // Register donations made in near
    let donor = env::predecessor_account_id();
    self.create_donation(
      id, donor, token, amount, amount_usd, target, event, campaign,
    )
  }

  pub fn token_donation(&mut self) {
    // Register donations made in other tokens
    // This functionality is done directly in fungible token file at deposit_tokens
  }
}
