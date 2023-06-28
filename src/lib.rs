use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Product {
  id: u32,
  owner: AccountId,
  name: String,
  price: u128,
  description: String,
  image: String,
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
  pub owner_id: AccountId,
  pub total_products: u32,
  pub products: UnorderedMap<u32, Product>,
  pub products_per_owner: UnorderedMap<AccountId, Vec<String>>,
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new() -> Self {
    Self {
      owner_id: env::signer_account_id(),
      total_products: 0,
      products: UnorderedMap::new(b"products".to_vec()),
      products_per_owner: UnorderedMap::new(b"products_per_owner".to_vec()),
    }
  }

  pub fn add_product(&mut self, name: String, price: u128, description: String, image: String) {
    let product = Product { id: self.total_products, owner: env::signer_account_id(), name, price, description, image };

    self.total_products += 1;
    self.products.insert(&product.id, &product);

    let mut products_per_owner = self.products_per_owner.get(&product.owner).unwrap_or(Vec::new());
    products_per_owner.push(product.name.clone());
  }

  // get product by id
  pub fn get_product(&self, id: u32) -> Product {
    self.products.get(&id).unwrap()
  }

  // get all products
  pub fn get_products(&self) -> Vec<(u32, Product)> {
    let mut all_products = Vec::new();

    for i in 0..self.products.len() {
      if let Some(product) = self.products.get(&(i as u32)) {
        all_products.push((product.id, product));
      }
    }

    all_products
  }

  pub fn update_product(&mut self, id: u32, name: String, price: u128, description: String, image: String) {
    let product = Product { id, owner: env::signer_account_id(), name, price, description, image };

    self.products.insert(&id, &product);
  }

  pub fn delete_product(&mut self, id: u32) {
    let owner = self.get_product(id).owner;
    assert_eq!(owner, env::signer_account_id(), "You are not the owner of this product");
    self.products.remove(&id);
  }

  // get products per owner
  pub fn get_products_per_owner(&self, owner: AccountId) -> Vec<(u32, String)> {
    let mut products = Vec::new();

    for i in 0..self.total_products {
      let product = self.products.get(&i).unwrap();
      if product.owner == owner {
        products.push((product.id, product.name.clone()));
      }
    }

    products
  }

  pub fn payment_product(&mut self, id: u32) {
    let price = self.products.get(&id).unwrap().price;
    assert!(env::account_balance() >= price, "Not enough balance to buy this product");
    assert!(price == env::attached_deposit(), "Attached deposit is not equal to the price of the product");

    let mut product = self.get_product(id);
    product.owner = env::signer_account_id();
    self.products.insert(&id, &product);
  }
}
