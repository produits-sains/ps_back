use rocket::Rocket;
use rocket_contrib::Json;

#[get("/find_product_rules")]
fn find_product_rules() -> Json<Vec<super::models::FindProductRules>> { 
  Json(vec![super::models::FindProductRules {
    xpath: "//a[contains(@class, 'productGTMClick')]/@href".to_string(),
    regexp_rule: "0+(\\d*)\\.html".to_string(),
    regexp_flags: "".to_string(),
  }])
}

pub fn register_routes(rocket: Rocket) -> Rocket {
    rocket.mount("/api/picard", routes![find_product_rules])
}
