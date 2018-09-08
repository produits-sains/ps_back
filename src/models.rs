use rocket::response::{Response, Responder};
use rocket::response::status;
use rocket::Request;
use rocket::http::Status;
use rocket_contrib::Json;
use diesel;
use diesel::prelude::*;
use serde::Serialize;

use database::DbConn;
use schema::raw_products as raw_products;
use schema::ingr as ingr;

//use schema::raw_products::dsl::raw_products;

#[derive(Serialize)]
pub struct ApiFailure {
    success: bool,
    error: Option<String>,
}

impl ApiFailure {
    fn generic_failure() -> ApiFailure {
        ApiFailure {
             success: false,
             error: None,
        }
    }
}

pub enum ApiResult<T: Serialize> {
    Ok(Json<T>), 
    Err(status::BadRequest<Json<ApiFailure>>),
}


impl<'r, T: Serialize> Responder<'r> for ApiResult<T> {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        match self {
            ApiResult::Ok(v) => v.respond_to(req),
            ApiResult::Err(e) => e.respond_to(req),
        }
    }
}

/*
#[derive(Serialize, Debug, Queryable)]
pub struct FindProductRules {
    pub id: Option<i32>,
    pub vendor: String,
    pub xpath: String,
    pub regexp_rule: String,
    pub regexp_flags: String,
}

#[derive(Deserialize, Debug, FromForm, Insertable)]
#[table_name="find_product_rules"]
pub struct NewFindProductRules {
    pub vendor: Option<String>,
    pub xpath: String,
    pub regexp_rule: String,
    pub regexp_flags: String,
}
*/

#[derive(Serialize, Queryable)]
pub struct RawProduct {
    pub id: Option<i32>,
    pub vendor: String,
    pub vendor_id: String,
    pub name: String,
    pub url: String,
    pub ingr_txt: String,
    pub scraper_version: String,
    pub parsed: bool,
}

#[derive(Deserialize, FromForm, AsChangeset, Insertable)]
#[table_name="raw_products"]
pub struct NewRawProduct {
    pub vendor: String,
    pub vendor_id: String,
    pub name: String,
    pub url: String,
    pub ingr_txt: String,
    pub scraper_version: String,
}


impl RawProduct {
    pub fn create(conn: DbConn, product: NewRawProduct) -> ApiResult<RawProduct> {
        match diesel::insert_into(raw_products::table).values(&product).execute(&*conn) {
            Ok(_) => {
                RawProduct::read_first(conn)
            },
            Err(_) => ApiResult::Err(status::BadRequest(Some(Json(ApiFailure::generic_failure())))),
        }
    }

    pub fn read(conn: DbConn) -> ApiResult<Vec<RawProduct>> {
        match raw_products::table.load::<RawProduct>(&*conn) {
            Ok(v) => ApiResult::Ok(Json(v)),
            Err(_) => ApiResult::Err(status::BadRequest(Some(Json(ApiFailure::generic_failure())))),
        }
    }

    pub fn read_first(conn: DbConn) -> ApiResult<RawProduct> {
        match raw_products::table.order(raw_products::id.desc()).first(&*conn) {
            Ok(v) => ApiResult::Ok(Json(v)),
            Err(_) => ApiResult::Err(status::BadRequest(Some(Json(ApiFailure::generic_failure())))),
        }
    }

    pub fn get_first_unparsed(conn: DbConn) -> ApiResult<RawProduct> {
        match raw_products::table.filter(raw_products::parsed.eq(false)).first(&*conn) {
            Ok(v) => ApiResult::Ok(Json(v)),
            Err(_) => ApiResult::Err(status::BadRequest(Some(Json(ApiFailure::generic_failure())))),
        }
    }


    pub fn update(conn: DbConn, product: NewRawProduct) -> ApiResult<RawProduct> {
        match diesel::update(raw_products::table.filter(raw_products::vendor.eq(product.vendor.as_str())).filter(raw_products::vendor_id.eq(product.vendor_id.as_str()))).set(&product).execute(&*conn) {
            Ok(_) => {
                RawProduct::read_from_new_product(conn, product)
            },
            Err(_) => ApiResult::Err(status::BadRequest(Some(Json(ApiFailure::generic_failure())))),
        }
    }

    pub fn read_from_new_product(conn: DbConn, product: NewRawProduct) -> ApiResult<RawProduct> {
        match raw_products::table.filter(raw_products::vendor.eq(product.vendor.as_str())).filter(raw_products::vendor_id.eq(product.vendor_id.as_str())).first(&*conn) {
            Ok(v) => ApiResult::Ok(Json(v)),
            Err(_) => ApiResult::Err(status::BadRequest(Some(Json(ApiFailure::generic_failure())))),
        }
    }
}

#[derive(Serialize, Queryable)]
pub struct Ingr {
    pub id: Option<i32>,
    pub vendor: String,
    pub name: String,
    pub score: i32,
    pub last_update: i32,
}

#[derive(Deserialize, FromForm, AsChangeset, Insertable)]
#[table_name="ingr"]
pub struct NewIngr {
    pub vendor: Option<String>,
    pub name: String,
    pub score: i32,
    pub last_update: Option<i32>,
}

impl Ingr {
    pub fn create(conn: DbConn, ingr: NewIngr) -> ApiResult<Ingr> {
        match diesel::insert_into(ingr::table).values(&ingr).execute(&*conn) {
            Ok(_) => {
                Ingr::read_first(conn)
            },
            Err(_) => ApiResult::Err(status::BadRequest(Some(Json(ApiFailure::generic_failure())))),
        }
    }

    pub fn read(conn: DbConn) -> ApiResult<Vec<Ingr>> {
        match ingr::table.load::<Ingr>(&*conn) {
            Ok(v) => ApiResult::Ok(Json(v)),
            Err(_) => ApiResult::Err(status::BadRequest(Some(Json(ApiFailure::generic_failure())))),
        }
    }

    pub fn read_first(conn: DbConn) -> ApiResult<Ingr> {
        match ingr::table.order(ingr::id.desc()).first(&*conn) {
            Ok(v) => ApiResult::Ok(Json(v)),
            Err(_) => ApiResult::Err(status::BadRequest(Some(Json(ApiFailure::generic_failure())))),
        }
    }

    pub fn update(conn: DbConn, id: i32, ingr: NewIngr) -> ApiResult<Ingr> {
        match diesel::update(ingr::table.filter(ingr::id.eq(id))).set(&ingr).execute(&*conn) {
            Ok(_) => {
                Ingr::read_by_id(conn, id)
            },
            Err(_) => ApiResult::Err(status::BadRequest(Some(Json(ApiFailure::generic_failure())))),
        }
    }

    pub fn read_by_id(conn: DbConn, id: i32) -> ApiResult<Ingr> {
        match ingr::table.filter(ingr::id.eq(id)).first(&*conn) {
            Ok(v) => ApiResult::Ok(Json(v)),
            Err(_) => ApiResult::Err(status::BadRequest(Some(Json(ApiFailure::generic_failure())))),
        }
    }
}
