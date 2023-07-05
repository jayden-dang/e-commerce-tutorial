use std::fmt;

use near_sdk::{
  serde::{Deserialize, Serialize},
  Balance,
};

use crate::Product;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
#[non_exhaustive]
pub enum EventLogVariant {
  Purchase(Vec<PurchaseProduct>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PurchaseProduct {
  pub owner_id: String,
  pub product_info: String,
  pub price: Balance,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub memo: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EventLog {
  pub standard: String,

  #[serde(flatten)]
  pub event: EventLogVariant,
}

impl fmt::Display for EventLog {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!("EVENT_JSON:{}", &serde_json::to_string(self).map_err(|_| fmt::Error)?))
  }
}
