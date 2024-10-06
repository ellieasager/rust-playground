mod dto;
use crate::dto::greeting::Greeting;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

struct AppState {
    name: Mutex<Greeting>,
    open_sea_client: reqwest::Client,
}

#[derive(Clone, Serialize, Deserialize)]
struct GreetingRequest {
    name: String,
}


async fn greet(req: web::Json<GreetingRequest>) -> impl Responder {

    let greeting = Greeting { 
        name: req.name.clone(), 
    };

    println!("GREET");
    web::Json(greeting)
}

async fn check(data: web::Data<AppState>) -> impl Responder {
    println!("CHECK");
    let check = data.name.lock().unwrap();
    let open_sea_client = &data.open_sea_client;
    // url = "https://api.opensea.io/api/v2/chain/amoy/account/address/nfts"

    web::Json(check.name.clone())
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        name: Mutex::new(Greeting{ name: "test".to_string(), }),
        open_sea_client: reqwest::Client::new(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .route("check", web::get().to(check))
            .route("greet", web::post().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
