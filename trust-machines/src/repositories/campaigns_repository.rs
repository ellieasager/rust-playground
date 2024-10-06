use std::collections::HashMap;

use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use aws_sdk_dynamodb::types::AttributeValue;
use serde_dynamo::aws_sdk_dynamodb_1::{from_item, from_items, to_attribute_value, to_item};

use crate::common::KickstarterError;
use crate::repositories::Campaign;

const CAMPAIGN_TABLE_NAME: &str = "campaigns";

pub async fn create_campaign(
    db_client: &aws_sdk_dynamodb::Client,
    campaign: Campaign,
) -> Result<String, KickstarterError> {
    let campaign_item: HashMap<String, AttributeValue> = to_item(campaign.clone())
        .map_err(|e| {
            println!("Error: {:?}", e);
            HttpResponse::BadRequest().body(e.to_string())
        })
        .unwrap();
    let _response = db_client
        .put_item()
        .table_name(CAMPAIGN_TABLE_NAME)
        .set_item(Some(campaign_item))
        .send()
        .await
        .map_err(|e| {
            println!("Error: {:?}", e);
            KickstarterError::InternalServer
        })
        .unwrap();

    Ok(campaign.id.to_string())
}

pub async fn get_campaign_by_id(
    db_client: &aws_sdk_dynamodb::Client,
    campaign_id: &String,
) -> Result<Campaign, KickstarterError> {
    let response = db_client
        .get_item()
        .table_name(CAMPAIGN_TABLE_NAME)
        .key("id", to_attribute_value(campaign_id).unwrap())
        .send()
        .await
        .map_err(|e| {
            println!("Error: {:?}", e);
            KickstarterError::InternalServer
        })
        .unwrap();

    if let Some(campaign_obj) = response.to_owned().item {
        let campaign: Campaign = from_item(campaign_obj)
            .map_err(|e| {
                println!("Error: {:?}", e);
                KickstarterError::InternalServer
            })
            .unwrap();
        Ok(campaign)
    } else {
        Err(KickstarterError::ItemNotFound)
    }
}

pub async fn update_campaign(
    db_client: &aws_sdk_dynamodb::Client,
    campaign_id: &String,
    update_expression: &str,
    expression_attribute_names: Option<HashMap<String, String>>,
    expression_attribute_values: Option<HashMap<String, AttributeValue>>,
) -> Result<String, KickstarterError> {
    let response = db_client
        .update_item()
        .table_name(CAMPAIGN_TABLE_NAME)
        .key("id", to_attribute_value(campaign_id).unwrap())
        .update_expression(update_expression)
        .set_expression_attribute_values(expression_attribute_values)
        .set_expression_attribute_names(expression_attribute_names)
        .send()
        .await
        .map_err(|e| {
            println!("Error: {:?}", e);
            KickstarterError::InternalServer
        });

    match response {
        Ok(_) => Ok(StatusCode::OK.to_string()),
        Err(_) => Err(KickstarterError::InternalServer),
    }
}

pub async fn delete_campaign(
    db_client: &aws_sdk_dynamodb::Client,
    campaign_id: &String,
) -> Result<String, KickstarterError> {
    let response = db_client
        .delete_item()
        .table_name(CAMPAIGN_TABLE_NAME)
        .key("id", to_attribute_value(campaign_id).unwrap())
        .send()
        .await
        .map_err(|e| {
            println!("Error: {:?}", e);
            KickstarterError::InternalServer
        });

    match response {
        Ok(_) => Ok(StatusCode::OK.to_string()),
        Err(_) => Err(KickstarterError::InternalServer),
    }
}

pub async fn query_campaigns(
    db_client: &aws_sdk_dynamodb::Client,
    key_condition_expression: &str,
    projection_expression: &str,
    index_name: &str,
    expression_attribute_names: Option<HashMap<String, String>>,
    expression_attribute_values: Option<HashMap<String, AttributeValue>>,
) -> Result<Vec<Campaign>, KickstarterError> {
    let response = db_client
        .query()
        .table_name(CAMPAIGN_TABLE_NAME)
        .index_name(index_name)
        .key_condition_expression(key_condition_expression)
        .projection_expression(projection_expression)
        .set_expression_attribute_values(expression_attribute_values)
        .set_expression_attribute_names(expression_attribute_names)
        .send()
        .await
        .map_err(|e| {
            println!("Error: {:?}", e);
            KickstarterError::InternalServer
        });

    if let Ok(result) = response.to_owned() {
        let campaigns_found: Vec<Campaign> = match result.items {
            Some(campaign_objs) => from_items(campaign_objs)
                .map_err(|e| {
                    println!("Error: {:?}", e);
                    KickstarterError::InternalServer
                })
                .unwrap(),
            None => Vec::new(),
        };
        return Ok(campaigns_found);
    }

    Err(KickstarterError::InternalServer)
}

// This method is for debugging only, let's not worry about it
pub async fn scan_campaigns(
    db_client: &aws_sdk_dynamodb::Client,
) -> Result<Vec<Campaign>, KickstarterError> {
    let response = db_client
        .scan()
        .table_name(CAMPAIGN_TABLE_NAME)
        .send()
        .await
        .map_err(|e| {
            println!("Error: {:?}", e);
            KickstarterError::InternalServer
        })
        .unwrap();

    let items: Vec<Campaign> = match response.to_owned().items {
        Some(items) => from_items(items)
            .map_err(|e| {
                println!("Error: {:?}", e);
                KickstarterError::InternalServer
            })
            .unwrap(),
        None => Vec::new(),
    };
    Ok(items)
}
