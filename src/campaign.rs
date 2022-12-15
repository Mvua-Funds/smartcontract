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
  pub current: u128,  // Campaign current amount
  pub current_usd: f64,  // Campaign current amount in usd
  pub token: String, // The targeted token

  pub voters: Vec<AccountId>, // Temporary set of people who have donated to this campaign, on voting, the donor is stripped from this list, can't vote again.
  pub partner: Option<String>, // The winner of the campaign
  pub partners: HashMap<String, u64>, // Possible companies to effect the campaign -> ((Kenya redcross, 20), ...) company name and votes.

  pub start_year: i32,
  pub start_month: i32,
  pub start_day: i32,
  pub end_year: i32,
  pub end_month: i32,
  pub end_day: i32,

  pub created_on: Timestamp,

  pub img: String,
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
    start_dates: String,
    end_dates: String,
    img: String,
  ) -> Self {

    let created_by = env::predecessor_account_id();
    let start_dates_ = start_dates.split(",").collect::<Vec<&str>>();
    let end_dates_ = end_dates.split(",").collect::<Vec<&str>>();

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
      current: 0,
      current_usd: 0.0,
      token,
      created_on: env::block_timestamp(),
      voters: Vec::new(),
      partner: None,
      partners: HashMap::new(),
      start_year: <i32 as FromStr>::from_str(start_dates_[0].trim()).unwrap(),
      start_month: <i32 as FromStr>::from_str(start_dates_[1].trim()).unwrap(),
      start_day: <i32 as FromStr>::from_str(start_dates_[2].trim()).unwrap(),
      end_year: <i32 as FromStr>::from_str(end_dates_[0].trim()).unwrap(),
      end_month: <i32 as FromStr>::from_str(end_dates_[1].trim()).unwrap(),
      end_day: <i32 as FromStr>::from_str(end_dates_[2].trim()).unwrap(),
      img,
    }
  }

  pub fn get_voter(&mut self, v: AccountId) -> bool {
    let exists = self.voters.iter().find(|k| k == &&v.clone());
    if exists.is_some() {
      self.voters.retain(|x| *x != v.clone());
      return true;
    }
    false
  }

  pub fn add_voter(&mut self, v: AccountId){
    self.voters.push(v);
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
    start_dates: String,
    end_date: String,
    end_dates: String,
    img: String,
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
      start_dates,
      end_dates,
      img,
    );
    self.campaigns.insert(&id.clone(), &campaign);
    self.campaigns_count += 1;
  }

  pub fn get_campaign(&self, id: String) -> Option<Campaign> {
    self.campaigns.get(&id)
  }

  // pub fn get_campaigns(&self) -> Vec<Campaign> {
  //   self.campaigns.values().collect()
  // }

  pub fn add_campaign_partner(&mut self, id: String, partner: String) -> String {
    let campaign = self.get_campaign(id.clone());
    if campaign.as_ref().is_some() {
      let mut c = campaign.unwrap();
      c.partners.insert(partner.clone(), 0);
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

  pub fn filter_campaigns(&self, period: String, year: i32, month: i32) -> Vec<Campaign> {
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

  pub fn get_campaigns(&self, page: usize, limit: usize) -> Response<Campaign>{

    let start_index = (page - 1) * limit;

    let campaigns: Vec<Campaign> = self.campaigns.values().into_iter()
            .skip(start_index)
            .take(limit)
            .collect();

    let response = Response{ 
      results: campaigns, 
      count: self.campaigns.len(),
    };
    return response;
  }

}
