use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn hello(req: HttpRequest) -> impl Responder {
    match req.cookie("_forward_auth") {
        Some(cookie) => {
            let val = cookie.value().split("|").collect::<Vec<&str>>();
            let email = val.get(2).unwrap_or(&"default");
            HttpResponse::Ok().body(format!("Test app. User: {}", email))
        }
        None => {
            HttpResponse::Unauthorized().body("No _forward_auth cookie found")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
    })
    .bind(("0.0.0.0", 9797))?
    .run()
    .await
}
