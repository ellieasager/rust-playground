use actix_web::http::Error;
use actix_web::web;
use aws_sdk_dynamodb::types::AttributeValue;
use serde_dynamo::aws_sdk_dynamodb_1::to_attribute_value;
use std::collections::HashMap;

use crate::common::{AppState, KickstarterError};
use crate::repositories::{campaigns_repository, UpdateCampaignRequest};
use crate::repositories::{Campaign, CampaignStatus, CreateCampaignRequest, GetCampaignsResponse};

pub async fn create_campaign(
    data: web::Data<AppState>,
    req: web::Json<CreateCampaignRequest>,
    param: web::Path<String>,
) -> Result<String, KickstarterError> {
    let user_id = param.clone();
    println!("creating a campaign for user_id {}", user_id);

    let campaign = Campaign::new(req, user_id);
    campaigns_repository::create_campaign(&data.db_client, campaign).await
}

pub async fn get_campaign_by_id_for_user_id(
    data: web::Data<AppState>,
    param: web::Path<(String, String)>,
) -> Result<Campaign, KickstarterError> {
    let user_id = param.clone().0;
    let campaign_id = param.clone().1;
    println!(
        "looking for campaign: {} for user_id {}",
        campaign_id, user_id
    );

    let campaign_result =
        campaigns_repository::get_campaign_by_id(&data.db_client, &campaign_id).await;

    if let Ok(campaign) = campaign_result {
        if campaign.user_id == user_id {
            return Ok(campaign);
        }
    }
    Err(KickstarterError::ItemNotFound)
}

pub async fn delete_campaign(
    data: web::Data<AppState>,
    param: web::Path<(String, String)>,
) -> Result<String, KickstarterError> {
    let user_id = param.clone().0;
    let campaign_id = param.clone().1;
    println!("deleting campaign: {} for user_id {}", campaign_id, user_id);

    let campaign = campaigns_repository::get_campaign_by_id(&data.db_client, &campaign_id)
        .await
        .unwrap();
    if campaign.user_id != user_id {
        Err(KickstarterError::ItemNotFound)
    } else if campaign.status != CampaignStatus::Created {
        Err(KickstarterError::BadRequest(
            ("you can only delete activity that has CREATED status").to_string(),
        ))
    } else {
        campaigns_repository::delete_campaign(&data.db_client, &campaign_id).await
    }
}

pub async fn start_campaign(
    data: web::Data<AppState>,
    param: web::Path<(String, String)>,
) -> Result<String, KickstarterError> {
    let user_id = param.clone().0;
    let campaign_id = param.clone().1;
    println!("starting campaign: {} for user_id {}", campaign_id, user_id);

    let campaign = campaigns_repository::get_campaign_by_id(&data.db_client, &campaign_id)
        .await
        .unwrap();
    if campaign.user_id != user_id {
        Err(KickstarterError::ItemNotFound)
    } else if campaign.status != CampaignStatus::Created {
        Err(KickstarterError::BadRequest(
            ("you can only start activity that has CREATED status").to_string(),
        ))
    } else {
        let update_expression = "set #status = :campaign_active";
        let expression_attribute_names =
            HashMap::from([("#status".to_string(), "status".to_string())]);
        let expression_attribute_values: HashMap<String, AttributeValue> = HashMap::from([(
            String::from(":campaign_active"),
            to_attribute_value(CampaignStatus::Active).unwrap(),
        )]);

        campaigns_repository::update_campaign(
            &data.db_client,
            &campaign_id,
            update_expression,
            Some(expression_attribute_names),
            Some(expression_attribute_values),
        )
        .await
    }
}

pub async fn update_campaign(
    data: web::Data<AppState>,
    req: web::Json<UpdateCampaignRequest>,
    param: web::Path<(String, String)>,
) -> Result<String, KickstarterError> {
    let user_id = param.clone().0;
    let campaign_id = param.clone().1;
    println!("updating campaign: {} for user_id {}", campaign_id, user_id);

    let campaign = campaigns_repository::get_campaign_by_id(&data.db_client, &campaign_id)
        .await
        .unwrap();
    if campaign.user_id != user_id {
        Err(KickstarterError::ItemNotFound)
    } else if campaign.status != CampaignStatus::Created {
        Err(KickstarterError::BadRequest(
            ("you can only update activity that has CREATED status").to_string(),
        ))
    } else {
        let update_expression = "set #name = :campaign_name, description = :campaign_descr, target_amount = :campaign_target";
        let expression_attribute_names = HashMap::from([("#name".to_string(), "name".to_string())]);
        let expression_attribute_values: HashMap<String, AttributeValue> = HashMap::from([
            (
                String::from(":campaign_name"),
                to_attribute_value(req.clone().name).unwrap(),
            ),
            (
                String::from(":campaign_descr"),
                to_attribute_value(req.clone().description).unwrap(),
            ),
            (
                String::from(":campaign_target"),
                to_attribute_value(req.clone().target_amount).unwrap(),
            ),
        ]);

        campaigns_repository::update_campaign(
            &data.db_client,
            &campaign_id,
            update_expression,
            Some(expression_attribute_names),
            Some(expression_attribute_values),
        )
        .await
    }
}

// All users can see "Active" campaigns unconditionally
pub async fn get_active_campaigns(
    data: web::Data<AppState>,
) -> Result<GetCampaignsResponse, Error> {
    println!("getting active campaigns");

    let key_condition_expression = "#status = :campaign_active";
    let projection_expression = "id, user_id, #name, description, target_amount, #status";
    let index_name = "status-index";
    let expression_attribute_names = HashMap::from([
        ("#name".to_string(), "name".to_string()),
        ("#status".to_string(), "status".to_string()),
    ]);
    let expression_attribute_values = HashMap::from([(
        String::from(":campaign_active"),
        to_attribute_value(CampaignStatus::Active).unwrap(),
    )]);

    let items = campaigns_repository::query_campaigns(
        &data.db_client,
        key_condition_expression,
        projection_expression,
        index_name,
        Some(expression_attribute_names),
        Some(expression_attribute_values),
    )
    .await
    .unwrap();

    Ok(GetCampaignsResponse { campaigns: items })
}

pub async fn get_campaigns_by_user_id(
    data: web::Data<AppState>,
    param: web::Path<String>,
) -> Result<GetCampaignsResponse, Error> {
    let user_id = param.clone();
    println!("listing campaigns for user_id {}", user_id.clone());

    let key_condition_expression = "user_id = :user_id";
    let projection_expression = "id, user_id, #name, description, target_amount, #status";
    let index_name = "user_id-index";
    let expression_attribute_names = HashMap::from([
        ("#name".to_string(), "name".to_string()),
        ("#status".to_string(), "status".to_string()),
    ]);
    let expression_attribute_values = HashMap::from([(
        String::from(":user_id"),
        to_attribute_value(user_id).unwrap(),
    )]);

    let items = campaigns_repository::query_campaigns(
        &data.db_client,
        key_condition_expression,
        projection_expression,
        index_name,
        Some(expression_attribute_names),
        Some(expression_attribute_values),
    )
    .await
    .unwrap();
    Ok(GetCampaignsResponse { campaigns: items })
}

// This method is for debugging only, let's not worry about it
pub async fn get_all_campaigns(data: web::Data<AppState>) -> Result<GetCampaignsResponse, Error> {
    println!("getting all campaigns");

    let items = campaigns_repository::scan_campaigns(&data.db_client)
        .await
        .unwrap();
    println!("found {:?} items", items.len());
    Ok(GetCampaignsResponse { campaigns: items })
}
