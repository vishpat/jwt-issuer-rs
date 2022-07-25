extern crate jsonwebkey as jwk;
//
//use serde_json;
//use jwk::{RsaPublic, ByteVec};
//
//
//fn main() -> Result<(), Box<dyn std::error::Error>> {
//    let key = jwk::Key::generate_p256();
//    println!("{}", serde_json::to_string_pretty(&key)?);
//    let mut my_jwk = jwk::JsonWebKey::new(key);
//
//    println!("{:?}", serde_json::to_string(&my_jwk)?);
//
//    Ok(())
//}
use actix_web::{get, post, web, App, HttpRequest, HttpServer, Responder};
use jwk::JsonWebKey;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
struct UserInfo {
    name: String,
}

#[get("/jwks")]
async fn jwks(req: HttpRequest) -> impl Responder {
    let local_data = req
        .app_data::<web::Data<JsonWebKey>>()
        .expect("user info missing");
    let jwks = [local_data.clone()];

    web::Json(jwks)
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
    let data = web::Data::new(my_jwk);

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
