pub mod campaigns_repository;

use actix_web::{
    body::BoxBody, http::header::ContentType, web, HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum CampaignStatus {
    Created,
    Active,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Campaign {
    pub id: Uuid,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub target_amount: i64,
    pub status: CampaignStatus,
}

impl Campaign {
    pub fn new(req: web::Json<CreateCampaignRequest>, user_id: String) -> Self {
        let new_id = Uuid::new_v4();
        Campaign {
            id: new_id,
            user_id,
            name: req.name.clone(),
            description: req.description.clone(),
            target_amount: req.target_amount,
            status: CampaignStatus::Created,
        }
    }
}

impl Responder for Campaign {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CreateCampaignRequest {
    pub name: String,
    pub description: String,
    pub target_amount: i64,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UpdateCampaignRequest {
    pub name: String,
    pub description: String,
    pub target_amount: i64,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetCampaignsResponse {
    pub campaigns: Vec<Campaign>,
}

impl Responder for GetCampaignsResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
