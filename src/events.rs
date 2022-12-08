use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct EventType {
  pub name: String, //Online event, physical
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Date {
  pub year: u8,
  pub month: u8,
  pub day: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Event {
  pub id: String,
  pub title: String,
  pub created_by: AccountId,
  pub managers: Vec<AccountId>,
  pub cause: String, // Campaign course ie food security, water, tree planting, etc
  pub date: Timestamp,
  pub description: String,
  pub target: u128,  // Campaign target amount
  pub token: String, // The targeted token
  // pub attending: u64, // NO of people who have said they are attending
  pub attendees: Vec<AccountId>, // Those attending
  pub venue: Option<String>,     // venue of the event
  pub event_type: String,
  pub channel: Option<String>, //Facebook, twitter spaces, youtube, google meet, etc
  pub channel_url: Option<String>,
  pub year: u8,
  pub month: u8,
  pub day: u8,
  pub created_on: Timestamp,

  pub voters: Vec<AccountId>, // Temporary set of people who have donated to this event, on voting, the donor is stripped from this list, can't vote again.
  pub partner: Option<String>, // The winner of the event
  pub partners: HashMap<String, u64>, // Possible companies to effect the event -> ((Kenya redcross, 20), ...) company name and votes.
}

impl Event {
  pub fn new(
    id: String,
    title: String,
    date: Timestamp,
    cause: String,
    description: String,
    target: U128,
    token: String,
    venue: String,
    event_type: String,
    channel: String,
    channel_url: String,
    dates: Date,
  ) -> Self {
    let created_by = env::predecessor_account_id();
    Self {
      id,
      title,
      created_by,
      managers: Vec::new(),
      cause,
      date,
      description,
      target: u128::from(target),
      token,
      attendees: Vec::new(),
      venue: Some(venue),
      event_type,
      channel: Some(channel),
      channel_url: Some(channel_url),
      created_on: env::block_timestamp(),
      year: dates.year,
      month: dates.month,
      day: dates.day,
      voters: Vec::new(),
      partner: None,
      partners: HashMap::new(),
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
  pub fn create_event(
    &mut self,
    id: String,
    title: String,
    date: Timestamp,
    cause: String,
    description: String,
    target: U128,
    token: String,
    venue: String,
    event_type: String,
    channel: String,
    channel_url: String,
    dates: Date,
  ) {
    let event = Event::new(
      id.clone(),
      title,
      date,
      cause,
      description,
      target,
      token,
      venue,
      event_type,
      channel,
      channel_url,
      dates,
    );
    self.events.insert(&id.clone(), &event);
  }

  pub fn get_event(&self, id: String) -> Option<Event> {
    self.events.get(&id)
  }

  pub fn add_event_partner(&mut self, id: String)-> String {
    let event = self.get_event(id.clone());
    if event.as_ref().is_some() {
      let mut e = event.unwrap();
      e.partners.insert(id.clone(), 0);
      self.events.insert(&id.clone(), &e);
      return "done".to_string();
    }
    return "not found".to_string();
  }

  pub fn event_vote(&mut self, id: String, partner: String) -> String {
    let v = env::predecessor_account_id();
    let event = self.get_event(id.clone());
    if event.as_ref().is_some() {
      let mut e = event.unwrap();
      if e.get_voter(v) {
        let votes = e.partners.get_mut(&partner.clone()).unwrap();
        *votes += 1;
        self.events.insert(&id.clone(), &e);
        return "done".to_string();
      }
      return "voter not found".to_string();
    }
    return "not found".to_string();
  }

  pub fn filter_events(&self, year: u8, month: u8) -> Vec<Offer> {
    let mut events = Vec::new();
    self.offers.to_vec().into_iter().for_each(|(_id, offer)| {
      if offer.offerer == account_id {
        offers.push(offer)
      }
    });
    offers
  }

}
