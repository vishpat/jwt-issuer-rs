extern crate jsonwebkey as jwk;
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

#[derive(Serialize, Deserialize, Debug)]
struct JWKS {
    keys: Vec<JsonWebKey>,
}

#[get("/jwks")]
async fn jwks(req: HttpRequest) -> impl Responder {
    let local_data = req
        .app_data::<web::Data<KeyData>>()
        .expect("Key Data not found");
    let jwks = vec![local_data.key.clone()];
    let keys = JWKS { keys: jwks };
    web::Json(keys)
}

#[post("/token")]
async fn token(user_info: web::Json<UserInfo>) -> impl Responder {
    format!("{}", user_info.name);
    format!("Get token for user: {}", user_info.name)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let key = jwk::Key::generate_p256();
    let my_jwk = jwk::JsonWebKey::new(key);
    let data = web::Data::new(KeyData { key: my_jwk });

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
