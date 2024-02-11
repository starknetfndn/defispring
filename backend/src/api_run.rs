use actix_web::{App, HttpServer};
use defispring::api::{
    self,
    data_storage::update_api_data,
    endpoints::{get_airdrop_amount, get_calldata, get_root, ApiDoc},
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    update_api_data();

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .service(get_calldata)
            .service(get_airdrop_amount)
            .service(get_root)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
