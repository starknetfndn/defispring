use actix_web::{web, App, HttpServer, Responder};
use api::data::update_api_data;
use defispring::api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    update_api_data();

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/get_calldata").to(api::handler::get_calldata))
            .service(web::resource("/get_token_amount").to(api::handler::get_token_amount))
            .service(web::resource("/get_root").to(api::handler::get_root))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
