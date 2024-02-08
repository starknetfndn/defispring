use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder};

use crate::api::structs::RoundTreeData;

use super::data::get_all_data;

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

    let round_data = get_all_data();
    let max_round = round_data.iter().max_by_key(|&p| p.round).unwrap().round;

    // Get the round parameter. Use the max found round if it's not given in query parameters
    let round = match query.get("round") {
        Some(v) => {
            let round = match v.parse::<u8>() {
                Ok(v) => v,
                Err(_) => max_round,
            };
            round
        }
        None => max_round,
    };

    println!(
        "round: {:?}, address: {:?}, protocol_id: {:?}",
        round, address, protocol_id
    );

    let relevant_data: Vec<RoundTreeData> = round_data
        .iter()
        .filter(|&p| p.protocol_id == protocol_id && p.round == round)
        .cloned()
        .collect();

    if (relevant_data.len() != 1) {
        let none: Vec<String> = Vec::new();
        return HttpResponse::Ok().json(none);
    }

    let calldata: Vec<String> =
        match relevant_data
            .get(0)
            .unwrap()
            .tree
            .address_calldata(round, protocol_id, &address)
        {
            Ok(v) => v,
            Err(_) => vec![],
        };
    let serialized = HttpResponse::Ok().json(calldata);
    serialized
}
