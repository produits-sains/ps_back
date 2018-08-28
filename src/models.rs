//extern crate serde;

//use schema::{picard_raw_products};

//#[derive(Debug, Insertable)]
//#[table_name="picard_raw_products"]
//pub struct NewPicardProducts {
//  pub id: u32,
//  pub name: String,
//  pub ingr_txt: String,
//}

#[derive(Serialize)]
pub struct FindProductRules {
  pub xpath: String,
  pub regexp_rule: String,
  pub regexp_flags: String,
}
