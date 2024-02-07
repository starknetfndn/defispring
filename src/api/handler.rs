use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder};

use super::data::get_latest_round_data;

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
    // TODO: do we want to query by round?
    /*     let round = match query.get("round") {
        Some(v) => {
            let round = match v.parse::<u8>() {
                Ok(v) => v,
                Err(_) => return HttpResponse::BadRequest().finish(),
            };
            round
        }
        None => return HttpResponse::BadRequest().finish(),
    }; */

    println!("address: {:?}, protocol_id: {:?}", address, protocol_id);

    // TODO: a bit awkward structure
    let round_data = get_latest_round_data();
    let tree = round_data.protocol_trees[&protocol_id].clone();
    let calldata: Vec<String> = match tree.address_calldata(&address) {
        Ok(v) => v,
        Err(_) => vec![],
    };
    let serialized = HttpResponse::Ok().json(calldata);
    serialized
}
