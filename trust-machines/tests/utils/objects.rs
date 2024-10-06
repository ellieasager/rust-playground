use actix_web::web;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum CampaignStatus {
    Created,
    Active,
    Funded,
    Closed,
    Complete,
    Canceled,
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetCampaignsResponse {
    pub campaigns: Vec<Campaign>,
}
