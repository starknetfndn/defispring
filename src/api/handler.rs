use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder};

use super::{
    data::{get_raw_airdrop_amount, get_raw_calldata, get_raw_root},
    merkle_tree::felt_to_b16,
};

pub async fn get_calldata(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let address = match query.get("address") {
        Some(v) => v,
        None => return HttpResponse::BadRequest().finish(),
    };

    // Get the round parameter. Use the max found round if it's not given in query parameters
    let round = match query.get("round") {
        Some(v) => {
            let round = match v.parse::<u8>() {
                Ok(v) => v,
                Err(_) => 0_u8, // means "use latest round"
            };
            round
        }
        None => 0_u8, // means "use latest round"
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
                Ok(v) => v,
                Err(_) => 0_u8, // means "use latest round"
            };
            round
        }
        None => 0_u8, // means "use latest round"
    };

    let amount = format!("{:#x}", get_raw_airdrop_amount(round, address));

    let serialized = HttpResponse::Ok().json(amount);
    serialized
}

pub async fn get_root(query: web::Query<HashMap<String, String>>) -> impl Responder {
    // Get the round parameter. Use the max found round if it's not given in query parameters
    let mut round = match query.get("round") {
        Some(v) => {
            let round = match v.parse::<u8>() {
                Ok(v) => v,
                Err(_) => 0_u8,
            };
            round
        }
        None => 0_u8,
    };

    let root = match get_raw_root(round) {
        Ok(v) => v,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let serialized = HttpResponse::Ok().json(felt_to_b16(&root));
    serialized
}
