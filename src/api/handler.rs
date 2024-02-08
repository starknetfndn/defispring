use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder};

use super::{
    data::{get_raw_calldata, get_raw_root},
    merkle_tree::felt_to_b16,
};

pub async fn get_calldata(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let address = match query.get("address") {
        Some(v) => v,
        None => return HttpResponse::BadRequest().finish(),
    };
    let protocol_id = match query.get("protocol_id") {
        Some(v) => {
            let protocol_id = match v.parse::<u8>() {
                Ok(v) => v,
                Err(_) => return HttpResponse::BadRequest().finish(),
            };
            protocol_id
        }
        None => return HttpResponse::BadRequest().finish(),
    };

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

    let calldata = get_raw_calldata(round, address, protocol_id);

    let serialized = HttpResponse::Ok().json(calldata);
    serialized
}

pub async fn get_root(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let protocol_id = match query.get("protocol_id") {
        Some(v) => {
            let protocol_id = match v.parse::<u8>() {
                Ok(v) => v,
                Err(_) => return HttpResponse::BadRequest().finish(),
            };
            protocol_id
        }
        None => return HttpResponse::BadRequest().finish(),
    };

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

    let root = match get_raw_root(round, protocol_id) {
        Ok(v) => v,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let serialized = HttpResponse::Ok().json(felt_to_b16(&root));
    serialized
}
