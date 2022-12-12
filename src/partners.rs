use crate::*;

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Partner {
  pub id: String,
  pub created_by: AccountId,
  pub name: String,
}

#[near_bindgen]
impl Contract {
  pub fn register_as_partner(&mut self, id: String, name: String) -> String {
    let created_by = env::predecessor_account_id();
    let part = self.get_partner(id.clone());
    if part.is_none() {
      let partner = Partner {
        id: id.clone(),
        created_by,
        name,
      };
      self.partners.insert(&id.clone(), &partner);
      return "success".to_string();
    }
    return "failed".to_string();
  }

  pub fn get_partner(&self, id: String) -> Option<Partner> {
    self.partners.get(&id)
  }

  pub fn get_account_partners(&self, account_id: AccountId) -> Vec<Partner> {
    self
      .partners
      .values()
      .into_iter()
      .filter(|partner| partner.created_by == account_id)
      .collect()
  }
}
