use actix_web::{web, App, HttpServer, Responder};
use defispring::api::{self, data_storage::update_api_data};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    update_api_data();

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/get_calldata").to(api::endpoints::get_calldata))
            .service(web::resource("/get_airdrop_amount").to(api::endpoints::get_airdrop_amount))
            .service(web::resource("/get_root").to(api::endpoints::get_root))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
