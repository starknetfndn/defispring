use actix_web::{web, Responder};

use super::data::get_api_data;

pub async fn hello() -> impl Responder {
    "Hello, World!"
}

pub async fn get_calldata() -> impl Responder {
    let aaa = get_api_data();
    aaa.root.value.to_string()
    //"Hello, World!2"
}
