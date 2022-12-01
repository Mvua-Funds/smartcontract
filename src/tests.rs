#[cfg(test)]

pub mod tests {
  // use super::*;
  use crate::constants::*;
  use crate::*;
  use near_sdk::json_types::U128;
  use near_sdk::test_utils::test_env::alice;
  use near_sdk::test_utils::VMContextBuilder;
  use near_sdk::{testing_env, VMContext, ONE_NEAR};

  pub fn get_context(is_view: bool) -> VMContext {
    VMContextBuilder::new()
      .current_account_id(master())
      .signer_account_id(dalmasonto())
      .predecessor_account_id(supercode())
      .is_view(is_view)
      .attached_deposit(0)
      .account_balance(0)
      .build()
  }

  #[test]
  fn test_test() {
    assert!(true);
  }

  #[test]
  fn test_func_sample() {
    let _context = get_context(false);
    testing_env!(_context);

    let mut contract = Contract::new();
    
  }

}
