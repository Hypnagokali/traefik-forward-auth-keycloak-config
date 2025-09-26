use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn app2_index(req: HttpRequest) -> impl Responder {
    match req.cookie("_forward_auth") {
        Some(cookie) => {
            let val = cookie.value().split("|").collect::<Vec<&str>>();
            let email = val.get(2).copied().unwrap_or("not logged in");
            HttpResponse::Ok().body(format!("Test app 2. User: {}", email))
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
            .route("/", web::get().to(app2_index))
    })
    .bind(("0.0.0.0", 9798))?
    .run()
    .await
}