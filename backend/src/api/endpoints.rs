use actix_web::{web, HttpResponse, Responder};

use super::{
    merkle_tree::felt_to_b16,
    processor::{get_raw_airdrop_amount, get_raw_calldata, get_raw_root}, structs::CairoCalldata,
};
use actix_web::get;
use serde::Deserialize;
use utoipa::{IntoParams, OpenApi};

#[derive(OpenApi)]
#[openapi(
    paths(
        get_root,
        get_airdrop_amount,
        get_calldata
    ),
    components(
        schemas(CairoCalldata)
    ),
    tags(
        (name = "DeFi REST API", description = "DeFi airdrop endpoints")
    ),
)]
pub struct ApiDoc;


#[derive(Deserialize, Debug, IntoParams)]
pub struct GetCalldataParams {
    /// Which round to query for. Leave out or 0 for the latest round.
    round: Option<u8>,
    /// Which address to query for.
    address: String
}

#[utoipa::path(
    tag = "Generates calldata for the associated Cairo contract",
    responses(
        (status = 200, description= "Calldata for the Cairo contract", body = CairoCalldata),       
    ),
    params(
        GetCalldataParams
    ),    
)]
#[get("/get_calldata")]
pub async fn get_calldata(query: web::Query<GetCalldataParams>) -> impl Responder {
    // Get the round parameter. Use the max found round if it's not given in query parameters or is 0
    let round = if query.round == Some(0) { None } else { query.round };

    let calldata = get_raw_calldata(round, &query.address);
 
    match calldata {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(value) => HttpResponse::BadRequest().json(value)
    }
}

#[derive(Deserialize, Debug, IntoParams)]
pub struct GetAirdropAmountParams {
    /// Which round to query for. Leave out or 0 for the latest round.
    round: Option<u8>,
    /// Which address to query for.
    address: String
}

#[utoipa::path(
    tag = "Gets the allocated airdrop amount for a given address",
    responses(
        (status = 200, description= "The allocated amount in hex", body = u128),       
    ),
    params(
        GetAirdropAmountParams
    ),    
)]
#[get("/get_airdrop_amount")]
pub async fn get_airdrop_amount(query: web::Query<GetAirdropAmountParams>) -> impl Responder {
    // Get the round parameter. Use the max found round if it's not given in query parameters or is 0
    let round = if query.round == Some(0) { None } else { query.round };
    
    match get_raw_airdrop_amount(round, &query.address) {
        Ok(value) => HttpResponse::Ok().json(format!("{:#x}", value)),
        Err(value) => HttpResponse::BadRequest().json(value)
    }
}

#[derive(Deserialize, Debug, IntoParams)]
pub struct GetRootParams {
    /// Which round to query for. Leave out or 0 for the latest round.
    round: Option<u8>,
}

#[utoipa::path(
    tag = "Gets the root value of the merkle tree",
    responses(
        (status = 200, description= "Hash of the root value", body = String),       
    ),
    params(
        GetRootParams
    ),    
)]
#[get("/get_root")]
pub async fn get_root(query: web::Query<GetRootParams>) -> impl Responder {
    // Get the round parameter. Use the max found round if it's not given in query parameters or is 0
    let round = if query.round == Some(0) { None } else { query.round };

    match get_raw_root(round)  {
        Ok(v) => HttpResponse::Ok().json(felt_to_b16(&v)),
        Err(value) => return HttpResponse::BadRequest().json(value),
    }
}
