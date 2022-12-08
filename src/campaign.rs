use std::collections::HashMap;

use crate::*;

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Campaign {
  pub id: String,
  pub created_by: AccountId,
  pub managers: Vec<AccountId>,
  pub title: String,
  pub cause: String, // Campaign course ie food security, water, tree planting, etc
  pub start_date: String,
  pub end_date: String,
  pub description: String,
  pub target: u128,  // Campaign target amount
  pub token: String, // The targeted token

  pub voters: Vec<AccountId>, // Temporary set of people who have donated to this campaign, on voting, the donor is stripped from this list, can't vote again.
  pub partner: Option<String>, // The winner of the campaign
  pub partners: HashMap<String, u64>, // Possible companies to effect the campaign -> ((Kenya redcross, 20), ...) company name and votes.

  pub start_year: u8,
  pub start_month: u8,
  pub start_day: u8,
  pub end_year: u8,
  pub end_month: u8,
  pub end_day: u8,

  pub created_on: Timestamp,
}

impl Campaign {
  pub fn new(
    id: String,
    title: String,
    cause: String,
    description: String,
    start_date: String,
    end_date: String,
    target: U128,
    token: String,
    start_year: u8,
    start_month: u8,
    start_day: u8,
    end_year: u8,
    end_month: u8,
    end_day: u8,
  ) -> Self {
    let created_by = env::predecessor_account_id();
    Self {
      id,
      created_by,
      managers: Vec::new(),
      title,
      cause,
      start_date,
      end_date,
      description,
      target: u128::from(target),
      token,
      created_on: env::block_timestamp(),
      voters: Vec::new(),
      partner: None,
      partners: HashMap::new(),
      start_year,
      start_month,
      start_day,
      end_year,
      end_month,
      end_day,
    }
  }

  pub fn get_voter(&self, v: AccountId) -> bool {
    let exists = self.voters.iter().find(|k| k == &&v);
    if exists.is_some() {
      return true;
    }
    false
  }
}

#[near_bindgen]
impl Contract {
  pub fn create_campaign(
    &mut self,
    id: String,
    title: String,
    cause: String,
    description: String,
    target: U128,
    token: String,
    start_date: String,
    start_year: u8,
    start_month: u8,
    start_day: u8,
    end_year: u8,
    end_month: u8,
    end_day: u8,
    end_date: String,
  ) {
    let campaign = Campaign::new(
      id.clone(),
      title,
      cause,
      description,
      start_date,
      end_date,
      target,
      token,
      start_year,
      start_month,
      start_day,
      end_year,
      end_month,
      end_day,
    );
    self.campaigns.insert(&id.clone(), &campaign);
  }

  pub fn get_campaign(&self, id: String) -> Option<Campaign> {
    self.campaigns.get(&id)
  }

  pub fn get_campaigns(&self) -> Vec<Campaign> {
    self.campaigns.values().collect()
  }

  pub fn add_campaign_partner(&mut self, id: String) -> String {
    let campaign = self.get_campaign(id.clone());
    if campaign.as_ref().is_some() {
      let mut c = campaign.unwrap();
      c.partners.insert(id.clone(), 0);
      self.campaigns.insert(&id.clone(), &c);
      return "done".to_string();
    }
    return "not found".to_string();
  }

  pub fn campaign_vote(&mut self, id: String, partner: String) -> String {
    let v = env::predecessor_account_id();
    let campaign = self.get_campaign(id.clone());
    if campaign.as_ref().is_some() {
      let mut c = campaign.unwrap();
      if c.get_voter(v) {
        let votes = c.partners.get_mut(&partner.clone()).unwrap();
        *votes += 1;
        self.campaigns.insert(&id.clone(), &c);
        return "done".to_string();
      }
      return "voter not found".to_string();
    }
    return "not found".to_string();
  }

  pub fn filter_campaigns(&self, period: String, year: u8, month: u8) -> Vec<Campaign> {
    // Periods of campaigns are eight start or end
    if period == "start".to_string() {
      let mut campaigns = Vec::new();
      self
        .campaigns
        .to_vec()
        .into_iter()
        .for_each(|(_id, campaign)| {
          if campaign.start_year == year && campaign.start_month == month {
            campaigns.push(campaign)
          }
        });
      return campaigns;
    } else {
      let mut campaigns = Vec::new();
      self
        .campaigns
        .to_vec()
        .into_iter()
        .for_each(|(_id, campaign)| {
          if campaign.end_year == year && campaign.end_month == month {
            campaigns.push(campaign)
          }
        });
      return campaigns;
    }
  }
}
