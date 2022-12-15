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
  pub year: i32,
  pub month: i32,
  pub day: i32,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Event {
  pub id: String,
  pub title: String,
  pub created_by: AccountId,
  pub managers: Vec<AccountId>,
  pub cause: String, // Event course ie food security, water, tree planting, etc
  pub date: String,
  pub description: String,
  pub target: u128,     // Event target amount
  pub current: u128,    // Event current amount
  pub current_usd: f64, // Event current amount in usd
  pub token: String,    // The targeted token
  // pub attending: u64, // NO of people who have said they are attending
  pub attendees: Vec<AccountId>, // Those attending
  pub venue: Option<String>,     // venue of the event
  pub event_type: String,
  pub channel: Option<String>, //Facebook, twitter spaces, youtube, google meet, etc
  pub channel_url: Option<String>,
  pub year: i32,
  pub month: i32,
  pub day: i32,
  pub created_on: Timestamp,

  pub voters: Vec<AccountId>, // Temporary set of people who have donated to this event, on voting, the donor is stripped from this list, can't vote again.
  pub partner: Option<String>, // The winner of the event
  pub partners: HashMap<String, u64>, // Possible companies to effect the event -> ((Kenya redcross, 20), ...) company name and votes.
  pub img: String,
}

impl Event {
  pub fn new(
    id: String,
    title: String,
    date: String,
    cause: String,
    description: String,
    target: U128,
    token: String,
    venue: String,
    event_type: String,
    channel: String,
    channel_url: String,
    dates: String,
    img: String,
  ) -> Self {
    let created_by = env::predecessor_account_id();
    let dates_ = dates.split(",").collect::<Vec<&str>>();
    Self {
      id,
      title,
      created_by,
      managers: Vec::new(),
      cause,
      date,
      description,
      target: u128::from(target),
      current: 0,
      current_usd: 0.0,
      token,
      attendees: Vec::new(),
      venue: Some(venue),
      event_type,
      channel: Some(channel),
      channel_url: Some(channel_url),
      created_on: env::block_timestamp(),
      year: <i32 as FromStr>::from_str(dates_[0].trim()).unwrap(),
      month: <i32 as FromStr>::from_str(dates_[1].trim()).unwrap(),
      day: <i32 as FromStr>::from_str(dates_[2].trim()).unwrap(),
      voters: Vec::new(),
      partner: None,
      partners: HashMap::new(),
      img,
    }
  }

  pub fn get_voter(&mut self, v: AccountId) -> bool {
    let exists = self.voters.iter().find(|k| k == &&v);
    if exists.is_some() {
      self.voters.retain(|x| *x != v.clone());
      return true;
    }
    false
  }
  pub fn add_voter(&mut self, v: AccountId) {
    self.voters.push(v);
  }
}

#[near_bindgen]
impl Contract {
  pub fn create_event(
    &mut self,
    id: String,
    title: String,
    date: String,
    cause: String,
    description: String,
    target: U128,
    token: String,
    venue: String,
    event_type: String,
    channel: String,
    channel_url: String,
    dates: String,
    img: String,
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
      img,
    );
    self.events.insert(&id.clone(), &event);
    self.events_count += 1;
  }

  pub fn get_event(&self, id: String) -> Option<Event> {
    self.events.get(&id)
  }

  pub fn add_event_partner(&mut self, id: String, partner: String) -> String {
    let event = self.get_event(id.clone());
    if event.as_ref().is_some() {
      let mut e = event.unwrap();
      e.partners.insert(partner.clone(), 0);
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

  pub fn filter_events(&self, year: i32, month: i32) -> Vec<Event> {
    let mut events = Vec::new();
    self.events.to_vec().into_iter().for_each(|(_id, event)| {
      if event.year == year && event.month == month {
        events.push(event)
      }
    });
    events
  }

  pub fn get_events(&self, page: usize, limit: usize) -> Response<Event> {
    let start_index = (page - 1) * limit;

    let events: Vec<Event> = self
      .events
      .values()
      .into_iter()
      .skip(start_index)
      .take(limit)
      .collect();

    let response = Response {
      results: events,
      count: self.events.len(),
    };
    return response;
  }
}
