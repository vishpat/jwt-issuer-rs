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
use serde::{Serialize, Deserialize};
use actix_web::{post, get, web, App, HttpServer, Responder};

#[derive(Serialize, Deserialize, Debug)]
struct UserInfo {
    name: String,
}

static KEY: Option<jwk::Key> = None;

#[get("/jwks")]
async fn jwks() -> impl Responder {
    let jwks = jwk::JsonWebKey::new(KEY.as_ref().unwrap().clone());
    let keys = [jwks];
    web::Json(keys)
}

#[post("/token")]
async fn token(user_info: web::Json<UserInfo>) -> impl Responder {
    format!("{}", user_info.name);
    format!("Get token for user: {}", user_info.name)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    KEY = Some(jwk::Key::generate_p256());

    HttpServer::new(|| {
        App::new().service(web::scope("/auth").service(jwks).service(token))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
