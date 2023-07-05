pub mod events;

use events::{EventLog, EventLogVariant, PurchaseProduct};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, ext_contract, log, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise};

pub type ProductId = String;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Product {
  pub product_id: ProductId,
  pub name: String,
  pub total_supply: u64,
  pub price: Balance,
  pub desc: String, // description
  pub owner: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Shop {
  pub owner: AccountId,
  pub name: String,
  pub desc: String,
  pub total_product: u64,
}

// Define the contract structure
#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
  pub platform_name: AccountId,
  pub products_per_shop: UnorderedMap<AccountId, Vec<Product>>,
  pub product_by_id: LookupMap<ProductId, Product>,
  pub products: UnorderedMap<u128, Product>,
  pub shops: LookupMap<AccountId, Shop>,
  pub all_shops: UnorderedMap<u128, Shop>,
  pub total_shops: u128,
  pub total_products: u128,
}

#[ext_contract(ext_ft_contract)]
pub trait FungibleTokenCore {
  fn ft_transfer_call(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>, msg: String) -> Promise;
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn init() -> Self {
    Self {
      platform_name: env::signer_account_id(), // String
      products_per_shop: UnorderedMap::new(b"products_per_shop".try_to_vec().unwrap()),
      product_by_id: LookupMap::new(b"product by id".try_to_vec().unwrap()),
      products: UnorderedMap::new(b"products".try_to_vec().unwrap()),
      shops: LookupMap::new(b"shops".try_to_vec().unwrap()),
      all_shops: UnorderedMap::new(b"all shops".try_to_vec().unwrap()),
      total_shops: 0,
      total_products: 0,
    }
  }

  pub fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String, product_id: ProductId) {
    let mut product = self.get_product_by_id(product_id);
    let price = product.price;

    assert_eq!(price, env::attached_deposit(), "Not Correct price");

    product.total_supply -= 1;

    let payment_info = EventLog {
      standard: "e-commerce-1.0.0".to_string(),
      event: EventLogVariant::Purchase(vec![PurchaseProduct {
        owner_id: product.product_id,
        product_info: product.desc,
        price,
        memo: None,
      }]),
    };

    env::log_str(&payment_info.to_string());

    // Promise::new(self.platform_name.clone()).transfer(price)
  }

  pub fn new_shop(&mut self, name: String, desc: String) -> Shop {
    let owner = env::signer_account_id();
    assert!(!self.shops.contains_key(&owner), "Shop already exists");
    let total_shop = self.total_shops + 1;

    let shop = Shop { owner: env::signer_account_id(), name, desc, total_product: 0 };

    self.shops.insert(&owner, &shop);
    self.all_shops.insert(&total_shop, &shop);

    shop
  }

  pub fn get_shop_by_id(&self, owner: AccountId) -> Shop {
    self.shops.get(&owner).unwrap()
  }

  pub fn get_all_shops(&self) -> Vec<Shop> {
    let mut all_shop: Vec<Shop> = Vec::new();

    for i in 1..self.all_shops.len() + 1 {
      if let Some(shop) = self.all_shops.get(&(i as u128)) {
        all_shop.push(shop);
      }
    }

    all_shop
  }

  pub fn new_product(
    &mut self,
    product_id: ProductId,
    name: String,
    total_supply: u64,
    price: Balance,
    desc: String,
  ) -> Product {
    let owner = env::signer_account_id();
    assert!(self.shops.contains_key(&owner), "Your Shop not exists");
    let product =
      Product { product_id: product_id.clone(), name, total_supply, price, desc, owner: env::signer_account_id() };

    let mut products_set: Vec<Product> = self.products_per_shop.get(&owner).unwrap_or_else(|| Vec::new());
    products_set.push(product.clone());

    self.products_per_shop.insert(&owner, &products_set);
    self.product_by_id.insert(&product_id, &product);
    let total = self.total_products + 1;
    self.products.insert(&total, &product);

    product
  }

  pub fn get_all_products(&self) -> Vec<Product> {
    let mut all_products: Vec<Product> = Vec::new();

    for i in 1..self.products.len() + 1 {
      if let Some(product) = self.products.get(&(i as u128)) {
        all_products.push(product);
      }
    }

    all_products
  }

  pub fn get_product_by_id(&self, product_id: ProductId) -> Product {
    self.product_by_id.get(&product_id).unwrap()
  }

  pub fn get_products_by_owner(&self, owner: AccountId) -> Vec<Product> {
    self.products_per_shop.get(&owner).unwrap_or_else(|| Vec::new())
  }

  /// exercise homework
  ///
  pub fn update_price(&mut self, new_price: Balance, product_id: ProductId) {
    let mut product = self.get_product_by_id(product_id);
    product.price = new_price;
  }

  #[payable]
  pub fn payment(&mut self, product_id: ProductId) -> Promise {
    let mut product = self.get_product_by_id(product_id);
    let price = product.price;

    assert_eq!(price, env::attached_deposit(), "Not Correct price");

    product.total_supply -= 1;

    let payment_info = EventLog {
      standard: "e-commerce-1.0.0".to_string(),
      event: EventLogVariant::Purchase(vec![PurchaseProduct {
        owner_id: product.product_id,
        product_info: product.desc,
        price,
        memo: None,
      }]),
    };

    env::log_str(&payment_info.to_string());

    Promise::new(self.platform_name.clone()).transfer(price)
  }
}
