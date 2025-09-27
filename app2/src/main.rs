use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn app2_index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("App2 currently not secured")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(app2_index))
    })
    .bind(("0.0.0.0", 9798))?
    .run()
    .await
}