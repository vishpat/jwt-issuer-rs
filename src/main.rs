extern crate jsonwebkey as jwk;
extern crate jsonwebtoken as jwt;
use actix_web::{get, post, web, App, HttpRequest, HttpServer, Responder};
use jwk::JsonWebKey;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct UserInfo {
    name: String,
}

struct KeyData {
    key: JsonWebKey,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct TokenClaims {
    exp: usize,
    sub: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct JWKS {
    keys: Vec<JsonWebKey>,
}

#[get("/jwks")]
async fn jwks(req: HttpRequest) -> web::Json<JWKS> {
    let local_data = req
        .app_data::<web::Data<KeyData>>()
        .expect("Key Data not found");
    let jwks = vec![local_data.key.clone()];
    let keys = JWKS { keys: jwks };
    web::Json(keys)
}

#[post("/token")]
async fn token(user_info: web::Json<UserInfo>, state: web::Data<KeyData>) -> impl Responder {
    let jwk = state.key.clone();
    let alg: jwt::Algorithm = jwk.algorithm.unwrap().into();

    let token = jwt::encode(
            &jwt::Header::new(alg),
            &TokenClaims {
                exp: 0,
                sub: user_info.name.clone(),
            },
            &jwk.key.to_encoding_key(),
        ).unwrap();
           
    token
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let key = jwk::Key::generate_p256();
    let mut my_jwk = jwk::JsonWebKey::new(key);
    my_jwk.set_algorithm(jwk::Algorithm::ES256).expect("Failed to set algorithm");
    let data = web::Data::new(KeyData { key: my_jwk});

    HttpServer::new(move || {
        App::new().service(
            web::scope("/auth")
                .app_data(data.clone())
                .service(jwks)
                .service(token),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
