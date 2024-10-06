#[allow(unused_imports)]
use std::collections::HashMap;

#[allow(unused_imports)]
use aws_sdk_dynamodb::types::AttributeValue;
use rstest::rstest;
#[allow(unused_imports)]
use serde_dynamo::aws_sdk_dynamodb_1::to_attribute_value;

#[allow(unused_imports)]
use crate::{
    common::KickstarterError,
    get_dynamodb_client,
    repositories::{
        campaigns_repository::{create_campaign, get_campaign_by_id, update_campaign},
        Campaign, CampaignStatus,
    },
    tests::utils::methods::{make_new_campaign, update_name_descr_amount},
};

#[rstest]
#[tokio::test]
pub async fn test_update_campaign_status_ok() {
    let db_client = get_dynamodb_client().await;

    let test_prefix = "E2E_START_CAMPAIGN";
    let user_id = format!("{}_user", test_prefix.to_string());

    let campaign_created: Campaign = make_new_campaign(user_id.to_owned(), test_prefix);

    let response = create_campaign(&db_client, campaign_created.clone()).await;
    assert!(response.is_ok());
    let campaign_id = response.ok().unwrap();

    let response = get_campaign_by_id(&db_client, &campaign_id).await;
    assert!(response.is_ok());
    let campaign_found: Campaign = response.ok().unwrap();
    // check campaign status == CREATED
    assert_eq!(CampaignStatus::Created, campaign_found.status);

    let update_expression = "set #status = :campaign_active";
    let expression_attribute_names = HashMap::from([("#status".to_string(), "status".to_string())]);
    let expression_attribute_values: HashMap<String, AttributeValue> = HashMap::from([(
        String::from(":campaign_active"),
        to_attribute_value(CampaignStatus::Active).unwrap(),
    )]);

    let response = update_campaign(
        &db_client,
        &campaign_id,
        update_expression,
        Some(expression_attribute_names),
        Some(expression_attribute_values),
    )
    .await;
    assert!(response.is_ok());

    let response = get_campaign_by_id(&db_client, &campaign_id).await;
    assert!(response.is_ok());
    let campaign_started: Campaign = response.ok().unwrap();
    // check campaign status == ACTIVE
    assert_eq!(CampaignStatus::Active, campaign_started.status);
}

#[rstest]
#[tokio::test]
pub async fn test_update_campaign_fields_ok() {
    let db_client = get_dynamodb_client().await;

    let test_prefix_original = "E2E_UPDATE_CAMPAIGN_ORIGINAL";
    let user_id = format!("{}_user", test_prefix_original.to_string());
    let campaign_original: Campaign = make_new_campaign(user_id.to_owned(), test_prefix_original);

    let response = create_campaign(&db_client, campaign_original.clone()).await;
    assert!(response.is_ok());
    let campaign_id = response.ok().unwrap();

    let test_prefix_new = "E2E_UPDATE_CAMPAIGN_NEW";
    let campaign_new: Campaign = update_name_descr_amount(&campaign_original, test_prefix_new);

    let update_expression = "set #name = :campaign_name, description = :campaign_descr, target_amount = :campaign_target";
    let expression_attribute_names = HashMap::from([("#name".to_string(), "name".to_string())]);
    let expression_attribute_values: HashMap<String, AttributeValue> = HashMap::from([
        (
            String::from(":campaign_name"),
            to_attribute_value(campaign_new.name.to_owned()).unwrap(),
        ),
        (
            String::from(":campaign_descr"),
            to_attribute_value(campaign_new.description.to_owned()).unwrap(),
        ),
        (
            String::from(":campaign_target"),
            to_attribute_value(campaign_new.target_amount.to_owned()).unwrap(),
        ),
    ]);
    let response = update_campaign(
        &db_client,
        &campaign_id,
        update_expression,
        Some(expression_attribute_names),
        Some(expression_attribute_values),
    )
    .await;
    assert!(response.is_ok());

    let response = get_campaign_by_id(&db_client, &campaign_id).await;
    assert!(response.is_ok());
    let campaign_updated: Campaign = response.ok().unwrap();
    assert_eq!(campaign_new, campaign_updated);
}

#[rstest]
#[tokio::test]
pub async fn test_update_campaign_bad_update_expression_fail() {
    let db_client = get_dynamodb_client().await;

    let test_prefix_original = "E2E_UPDATE_CAMPAIGN_ORIGINAL";
    let user_id = format!("{}_user", test_prefix_original.to_string());
    let campaign_original: Campaign = make_new_campaign(user_id.to_owned(), test_prefix_original);

    let response = create_campaign(&db_client, campaign_original.clone()).await;
    assert!(response.is_ok());
    let campaign_id = response.ok().unwrap();

    let test_prefix_new = "E2E_UPDATE_CAMPAIGN_NEW";
    let campaign_new: Campaign = update_name_descr_amount(&campaign_original, test_prefix_new);

    let update_expression = "invalid update expression";
    let expression_attribute_names = HashMap::from([("#name".to_string(), "name".to_string())]);
    let expression_attribute_values: HashMap<String, AttributeValue> = HashMap::from([
        (
            String::from(":campaign_name"),
            to_attribute_value(campaign_new.name.to_owned()).unwrap(),
        ),
        (
            String::from(":campaign_descr"),
            to_attribute_value(campaign_new.description.to_owned()).unwrap(),
        ),
        (
            String::from(":campaign_target"),
            to_attribute_value(campaign_new.target_amount.to_owned()).unwrap(),
        ),
    ]);
    let response = update_campaign(
        &db_client,
        &campaign_id,
        update_expression,
        Some(expression_attribute_names),
        Some(expression_attribute_values),
    )
    .await;
    assert!(response.is_err());
    assert_eq!(KickstarterError::InternalServer, response.err().unwrap());

    let response = get_campaign_by_id(&db_client, &campaign_id).await;
    assert!(response.is_ok());
    let campaign_found: Campaign = response.ok().unwrap();
    // make sure the campaign didn't change
    assert_eq!(campaign_original, campaign_found);
}
