use std::time::SystemTime;
use rocket;
use rocket::Rocket;
use rocket_contrib::Json;

use database::DbConn;
use models::*;

//#[get("/find_product_rules")]
//fn api_find_product_rules(conn: DbConn) -> Json<Vec<super::models::FindProductRules>> { 
//    let rules = find_product_rules.load::<super::models::FindProductRules>(&*conn).expect("Error loading rules");
//    Json(rules)
//}
//
//#[get("/product_rules")]
//fn api_get_product_rules(conn: DbConn) -> Json<Vec<super::models::FindProductRules>> {
//    api_find_product_rules(conn)
//}

//#[post("/product_rules", data="<rule>")]
//fn api_post_product_rules(conn: DbConn, mut rule: Json<super::models::NewFindProductRules>) -> Json<Vec<super::models::FindProductRules>> {
//    rule.vendor = Some("picard".to_string());
//    insert_into(find_product_rules).values(&rule.into_inner()).execute(&*conn).expect("Unable to create new rules");
//    api_find_product_rules(conn)
//}

#[get("/raw_product")]
fn api_get_raw_product(conn: DbConn) -> ApiResult<Vec<RawProduct>> {
    RawProduct::read(conn)
}

#[post("/raw_product", data="<product>")]
fn api_post_raw_product(conn: DbConn, product: Json<NewRawProduct>) -> ApiResult<RawProduct> {
    RawProduct::create(conn, product.into_inner())
}

#[put("/raw_product", data="<product>")]
fn api_put_raw_product(conn: DbConn, product: Json<NewRawProduct>) -> ApiResult<RawProduct> {
    RawProduct::update(conn, product.into_inner())
}

#[get("/ingr")]
fn api_get_ingr(conn: DbConn) -> ApiResult<Vec<Ingr>> {
    Ingr::read(conn)
}

#[post("/ingr", data="<ingr>")]
fn api_post_ingr(conn: DbConn, mut ingr: Json<NewIngr>) -> ApiResult<Ingr> {
    ingr.vendor = Some("picard".to_string());
    ingr.last_update = Some(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i32); 
    Ingr::create(conn, ingr.into_inner())
}

#[get("/ingr/<id>")]
fn api_get_ingr_by_id(conn: DbConn, id: i32) -> ApiResult<Ingr> {
    Ingr::read_by_id(conn, id)
}

#[put("/ingr/<id>", data="<ingr>")]
fn api_put_ingr_by_id(conn: DbConn, id: i32, ingr: Json<NewIngr>) -> ApiResult<Ingr> {
    Ingr::update(conn, id, ingr.into_inner())
}

pub fn register_routes(rocket: Rocket) -> Rocket {
    rocket
        //.mount("/api/picard", routes![api_find_product_rules])
        //.mount("/api/picard", routes![api_get_product_rules])
        //.mount("/api/picard", routes![api_post_product_rules])
        .mount("/api/picard", routes![api_get_raw_product])
        .mount("/api/picard", routes![api_post_raw_product])
        .mount("/api/picard", routes![api_put_raw_product])
        .mount("/api/picard", routes![api_get_ingr])
        .mount("/api/picard", routes![api_post_ingr])
        .mount("/api/picard", routes![api_get_ingr_by_id])
        .mount("/api/picard", routes![api_put_ingr_by_id])
}
