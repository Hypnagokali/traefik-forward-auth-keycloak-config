use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use jsonwebtoken::{decode, DecodingKey, Validation};

#[derive(Debug, serde::Deserialize)]
struct Claims {
    pub name: String,
}

async fn app1_index(req: HttpRequest) -> impl Responder {
    match req.headers().get("authorization") {
        Some(header) => {
            let token = header.to_str().unwrap().replace("Bearer ", "");
            let mut validation = Validation::default();
            // ToDo: verify signature
            validation.insecure_disable_signature_validation();
            // Disable exp validation for testing purposes (needs discussion)
            validation.validate_exp = false;
            validation.set_audience(&["test"]);

            let token_data = decode::<Claims>(
                &token,
                &DecodingKey::from_secret(b"ignored for now"),
                &validation,
            );

            match token_data {
                Ok(data) => {
                    return HttpResponse::Ok().body(format!("Test app 1. User: {}", data.claims.name))
                }
                Err(err) => {
                    // Its possible to get an expired token from oauth2-proxy
                    // delete cookie here
                    println!("Token decode error: {:?}", err);
                    return HttpResponse::Unauthorized().body("Invalid token");
                }
            }
        }
        None => {
            return HttpResponse::Unauthorized().body("No Authorization header found");
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(app1_index))
    })
    .bind(("0.0.0.0", 9797))?
    .run()
    .await
}
