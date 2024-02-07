use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder};

use super::data::get_api_data;

pub async fn hello() -> impl Responder {
    "Hello, World!"
}

pub async fn get_calldata(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let param_value = match query.get("address") {
        Some(v) => v,
        None => return HttpResponse::BadRequest().finish(),
    };

    let tree = get_api_data();
    let calldata: Vec<String> = match tree.address_calldata(&param_value) {
        Ok(v) => v,
        Err(_) => vec![],
    };
    let serialized = HttpResponse::Ok().json(calldata);
    serialized
}
