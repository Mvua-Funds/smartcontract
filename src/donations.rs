use crate::*;

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct DonationDetails {
  pub donation: Donation,
  pub tokenmetadata: Option<TokenMetadata>,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
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
    if campaign == "null".to_string() {
      cid = None
    }
    let mut eid = Some(event.clone());
    if event == "null".to_string() {
      eid = None
    }

    let donation = Donation::new(
      id,
      donor.clone(),
      token,
      amount,
      amount_usd,
      target.clone(),
      eid,
      cid,
    );
    self.donations.insert(&donation);

    if target.clone() == "event".to_string() {
      let mut event_itself = self.get_event(event.clone()).unwrap();
      event_itself.add_voter(donor.clone());
      self.events.insert(&event.clone(), &event_itself);
    } else if target == "campaign".to_string() {
      let mut campaign_itself = self.get_campaign(campaign.clone()).unwrap();
      campaign_itself.add_voter(donor.clone());
      self.campaigns.insert(&campaign.clone(), &campaign_itself);
    }
  }

  #[payable]
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
      id,
      donor.clone(),
      token,
      amount,
      amount_usd,
      target.clone(),
      event.clone(),
      campaign.clone(),
    );
    
    env::attached_deposit();
  }

  pub fn get_campaign_donations(
    &self,
    id: String,
    page: usize,
    limit: usize,
  ) -> Response<DonationDetails> {
    let start_index = (page - 1) * limit;

    let donations: Vec<Donation> = self
      .donations
      .iter()
      .filter(|don| don.campaign == Some(id.clone()))
      .skip(start_index)
      .take(limit)
      .collect();

    let donations_ = self
      .donations
      .iter()
      .filter(|don| don.campaign == Some(id.clone()))
      .count();

    let mut results: Vec<DonationDetails> = Vec::new();

    for don in donations {
      let metadata = self.get_token(don.token.clone());
      let details = DonationDetails {
        donation: don,
        tokenmetadata: metadata,
      };
      results.push(details);
    }
    let response = Response {
      results,
      count: donations_ as u64,
    };
    return response;
  }

  pub fn get_event_donations(
    &self,
    id: String,
    page: usize,
    limit: usize,
  ) -> Response<DonationDetails> {
    let start_index = (page - 1) * limit;

    let donations: Vec<Donation> = self
      .donations
      .iter()
      .filter(|don| don.event == Some(id.clone()))
      .skip(start_index)
      .take(limit)
      .collect();

    let donations_ = self
      .donations
      .iter()
      .filter(|don| don.event == Some(id.clone()))
      .count();

    let mut results: Vec<DonationDetails> = Vec::new();

    for don in donations {
      let metadata = self.get_token(don.token.clone());
      let details = DonationDetails {
        donation: don,
        tokenmetadata: metadata,
      };
      results.push(details);
    }
    let response = Response {
      results,
      count: donations_ as u64,
    };
    return response;
  }

  pub fn token_donation(&mut self) {
    // Register donations made in other tokens
    // This functionality is done directly in fungible token file at deposit_tokens
  }
}
