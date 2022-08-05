mod api;
use crate::api::post::{
    get_all_posts,
    submit_post
};

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    
    HttpServer::new(|| {
        App::new()
        .service(get_all_posts)
        .service(submit_post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

