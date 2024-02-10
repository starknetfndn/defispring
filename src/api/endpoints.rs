use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder};

use super::{
    merkle_tree::felt_to_b16,
    processor::{get_raw_airdrop_amount, get_raw_calldata, get_raw_root},
};

pub async fn get_calldata(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let address = match query.get("address") {
        Some(v) => v,
        None => return HttpResponse::BadRequest().finish(),
    };

    let round = match query.get("round") {
        Some(v) => {
            let round = match v.parse::<u8>() {
                Ok(v) => Some(v),
                Err(_) => None,
            };
            round
        }
        None => None,
    };

    let calldata = get_raw_calldata(round, address);

    let serialized = HttpResponse::Ok().json(calldata);
    serialized
}

pub async fn get_airdrop_amount(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let address = match query.get("address") {
        Some(v) => v,
        None => return HttpResponse::BadRequest().finish(),
    };

    // Get the round parameter. Use the max found round if it's not given in query parameters
    let round = match query.get("round") {
        Some(v) => {
            let round = match v.parse::<u8>() {
                Ok(v) => Some(v),
                Err(_) => None,
            };
            round
        }
        None => None,
    };
    let amount = match get_raw_airdrop_amount(round, address) {
        Ok(value) => format!("{:#x}", value),
        Err(value) => return HttpResponse::BadRequest().json(value),
    };

    let serialized = HttpResponse::Ok().json(amount);
    serialized
}

pub async fn get_root(query: web::Query<HashMap<String, String>>) -> impl Responder {
    // Get the round parameter. Use the max found round if it's not given in query parameters
    let round = match query.get("round") {
        Some(v) => {
            let round = match v.parse::<u8>() {
                Ok(v) => Some(v),
                Err(_) => None,
            };
            round
        }
        None => None,
    };

    let root = match get_raw_root(round) {
        Ok(v) => v,
        Err(value) => return HttpResponse::BadRequest().json(value),
    };
    let serialized = HttpResponse::Ok().json(felt_to_b16(&root));
    serialized
}
