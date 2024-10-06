use actix_web::{web, App, HttpServer};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::{self as ddb};
use dotenv::dotenv;

mod common;
mod handlers;
mod repositories;
mod tests;

use common::AppState;
use handlers::campaign::{
    create_campaign, delete_campaign, get_active_campaigns, get_all_campaigns,
    get_campaign_by_id_for_user_id, get_campaigns_by_user_id, start_campaign, update_campaign,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("server is running");

    let db_client = get_dynamodb_client().await;
    let data = web::Data::new(AppState { db_client });
    println!("Connection to the database established!");

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route(
                "/user/{user_id}/campaigns/create",
                web::post().to(create_campaign),
            )
            .route(
                "/user/{user_id}/campaigns/debugging",
                web::get().to(get_all_campaigns),
            )
            .route(
                "/user/{user_id}/campaigns/active",
                web::get().to(get_active_campaigns),
            )
            .route(
                "/user/{user_id}/campaigns/{id}",
                web::get().to(get_campaign_by_id_for_user_id),
            )
            .route(
                "/user/{user_id}/campaigns/{id}",
                web::put().to(update_campaign),
            )
            .route(
                "/user/{user_id}/campaigns/{id}/start",
                web::put().to(start_campaign),
            )
            .route(
                "/user/{user_id}/campaigns/{id}",
                web::delete().to(delete_campaign),
            )
            .route(
                "/user/{user_id}/campaigns",
                web::get().to(get_campaigns_by_user_id),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn get_dynamodb_client() -> Client {
    dotenv().ok();
    let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
    let sdk_config = aws_config::from_env().region(region_provider).load().await;
    ddb::Client::new(&sdk_config)
}
