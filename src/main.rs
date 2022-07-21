use actix_web::{dev::Service as _, web, App, HttpServer};
use futures_util::future::FutureExt;
use mongodb::Client;

#[path = "app/constants/index.rs"]
mod constants;
#[path = "app/models/user.rs"]
pub(crate) mod model;
#[path = "routes/index.rs"]
mod routes;

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(web::scope("/routes"))
            .service(routes::index)
            .service(routes::create_user)
            .service(routes::update_user)
            .service(routes::get_all_users)
            .service(routes::delete_user)
            .service(routes::create_jwt_token)
            .service(routes::get_user)
            .service(routes::search_users)
            .service(routes::upload_file)
            .service(routes::download_file)
            .service(routes::check_token),
    );
}
#[actix_web::main]

async fn main() -> std::io::Result<()> {
    let client = Client::with_uri_str(constants::DB_URL)
        .await
        .expect("failed to connect");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .configure(init)
            .wrap_fn(|req, srv| {
                println!("Hi from start. You requested: {}", req.path());

                srv.call(req).map(|res| {
                    println!("you get response successfully");

                    res
                })
            })
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
