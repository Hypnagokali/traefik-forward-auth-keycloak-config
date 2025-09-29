use actix_web::{web::{self, Data}, App, HttpRequest, HttpResponse, HttpServer, Responder};
use jsonwebtoken::{decode, decode_header, Validation};

use crate::oidc::{jwks::JwksStore, Claims};

pub mod oidc;

async fn app1_index(req: HttpRequest, jwks_store: Data<JwksStore>) -> impl Responder {
    match req.headers().get("authorization") {
        Some(header) => {
            let token = header.to_str().unwrap().replace("Bearer ", "");

            if !jwks_store.has_jwks() {
                if let Err(err) = jwks_store.fetch_jwks().await {
                    println!("Failed to fetch JWKS: {:?}", err);
                    return HttpResponse::InternalServerError().body("Failed to fetch JWKS");
                }
            }

            let jwks = match decode_header(&token) {
                Ok(header) => {
                    if let Some(kid) = header.kid {
                        jwks_store.get_key(&kid)
                    } else {
                        None
                    }
                },
                Err(err) => {
                    println!("Failed to decode token header: {:?}", err);
                    return HttpResponse::BadRequest().body("Invalid token header");
                }
            };

            let key = match jwks {
                Some(key) => {
                    match key.to_decoding_key() {
                        Ok(decoding_key) => decoding_key,
                        Err(err) => {
                            println!("Failed to create decoding key: {:?}", err);
                            return HttpResponse::InternalServerError().body("Failed to create decoding key");
                        }
                    }
                },
                None => {
                    println!("No matching JWK found");
                    return HttpResponse::Unauthorized().body("No matching JWK found");
                }
            };

            let mut validation = Validation::new(jsonwebtoken::Algorithm::RS256);

            validation.validate_exp = false;
            validation.set_audience(&["test"]);

            let token_data = decode::<Claims>(
                &token,
                &key,
                &validation,
            );

            match token_data {
                Ok(data) => {
                    let roles = data.claims.roles.join(", ");
                    return HttpResponse::Ok().body(format!("Test app 1. User: '{}' with roles: '{}'", data.claims.name, roles))
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
    // hardcoded for now
    let jwks = Data::new(JwksStore::new("http://auth.his-test.local/realms/test/.well-known/openid-configuration"));
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(app1_index))
            .app_data(jwks.clone())
    })
    .bind(("0.0.0.0", 9797))?
    .run()
    .await
}
