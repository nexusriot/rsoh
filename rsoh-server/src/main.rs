use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

async fn command() -> impl Responder {
    HttpResponse::Ok().body("ls")
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/command", web::get().to(command))
    })
    .bind("127.0.0.1:3137")?
    .run()
    .await
}
