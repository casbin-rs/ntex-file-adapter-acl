use std::io;

use ntex::web::{self, App, HttpResponse, HttpServer};

#[ntex::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    loge::init();

    HttpServer::new(|| App::new().service(web::resource("/").to(|| async { HttpResponse::Ok() })))
        .bind("127.0.0.1:59090")?
        .run()
        .await
}
