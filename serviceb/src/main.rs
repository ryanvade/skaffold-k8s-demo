use actix_web::{ http, web, App, HttpResponse, HttpServer, Responder };

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("{\"Service\": \"B\"}")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}

