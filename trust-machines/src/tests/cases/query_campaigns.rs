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
        campaigns_repository::{
            create_campaign, get_campaign_by_id, query_campaigns, update_campaign,
        },
        Campaign, CampaignStatus,
    },
    tests::utils::methods::make_new_campaign,
};

#[rstest]
#[tokio::test]
pub async fn test_query_active_campaigns_ok() {
    let db_client = get_dynamodb_client().await;

    // Create campaign and don't start it to keep status == CREATED
    let test_prefix_created = "E2E_GET_ACTIVE_CAMPAIGNS_CREATED";
    let user_id = format!("{}_user", test_prefix_created.to_string());
    let campaign_created: Campaign = make_new_campaign(user_id.to_owned(), test_prefix_created);

    let response = create_campaign(&db_client, campaign_created.clone()).await;
    assert!(response.is_ok());
    let campaign_id_created = response.ok().unwrap();

    // Create another campaign and start it to make status == ACTIVE
    let test_prefix_active = "E2E_GET_ACTIVE_CAMPAIGNS_ACTIVE";
    let user_id = format!("{}_user", test_prefix_active.to_string());
    let campaign_active: Campaign = make_new_campaign(user_id.to_owned(), test_prefix_active);

    let response = create_campaign(&db_client, campaign_active.clone()).await;
    assert!(response.is_ok());
    let campaign_id_active = response.ok().unwrap();

    // update campaign_2 to status = ACTIVE
    let update_expression = "set #status = :campaign_active";
    let expression_attribute_names = HashMap::from([("#status".to_string(), "status".to_string())]);
    let expression_attribute_values: HashMap<String, AttributeValue> = HashMap::from([(
        String::from(":campaign_active"),
        to_attribute_value(CampaignStatus::Active).unwrap(),
    )]);

    let response = update_campaign(
        &db_client,
        &campaign_id_active,
        update_expression,
        Some(expression_attribute_names),
        Some(expression_attribute_values),
    )
    .await;
    assert!(response.is_ok());

    // Now get all active campaigns (should contain the active campaign we created and started)
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

    let campaigns_found = query_campaigns(
        &db_client,
        key_condition_expression,
        projection_expression,
        index_name,
        Some(expression_attribute_names),
        Some(expression_attribute_values),
    )
    .await
    .unwrap();

    assert!(campaigns_found.len() >= 1);
    let campaigns_ids_found: Vec<String> =
        campaigns_found.iter().map(|c| c.id.to_string()).collect();
    assert!(!campaigns_ids_found.contains(&campaign_id_created));
    assert!(campaigns_ids_found.contains(&campaign_id_active));
}

#[rstest]
#[tokio::test]
pub async fn test_query_campaigns_by_user_id_ok() {
    let db_client = get_dynamodb_client().await;

    // Create campaign and don't start it to keep status == CREATED
    let test_prefix_created = "E2E_GET_ACTIVE_CAMPAIGNS_CREATED";
    let user_id = format!("{}_user", test_prefix_created.to_string());
    let campaign_created: Campaign = make_new_campaign(user_id.to_owned(), test_prefix_created);

    let response = create_campaign(&db_client, campaign_created.clone()).await;
    assert!(response.is_ok());
    let campaign_id_created = response.ok().unwrap();

    // Create another campaign and start it to make status == ACTIVE
    let test_prefix_active = "E2E_GET_ACTIVE_CAMPAIGNS_ACTIVE";
    let campaign_active: Campaign = make_new_campaign(user_id.to_owned(), test_prefix_active);

    let response = create_campaign(&db_client, campaign_active.clone()).await;
    assert!(response.is_ok());
    let campaign_id_active = response.ok().unwrap();

    // update campaign_2 to status = ACTIVE
    let update_expression = "set #status = :campaign_active";
    let expression_attribute_names = HashMap::from([("#status".to_string(), "status".to_string())]);
    let expression_attribute_values: HashMap<String, AttributeValue> = HashMap::from([(
        String::from(":campaign_active"),
        to_attribute_value(CampaignStatus::Active).unwrap(),
    )]);

    let response = update_campaign(
        &db_client,
        &campaign_id_active,
        update_expression,
        Some(expression_attribute_names),
        Some(expression_attribute_values),
    )
    .await;
    assert!(response.is_ok());

    // Now get all campaigns belonging to our user regardless of the status
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

    let campaigns_found = query_campaigns(
        &db_client,
        key_condition_expression,
        projection_expression,
        index_name,
        Some(expression_attribute_names),
        Some(expression_attribute_values),
    )
    .await
    .unwrap();

    // We should find both campaigns we created - regarless if they have started
    assert!(campaigns_found.len() >= 2);
    let campaigns_ids_found: Vec<String> =
        campaigns_found.iter().map(|c| c.id.to_string()).collect();
    assert!(campaigns_ids_found.contains(&campaign_id_created));
    assert!(campaigns_ids_found.contains(&campaign_id_active));
}

#[rstest]
#[tokio::test]
pub async fn test_query_campaigns_bad_key_condition_expression_fail() {
    let db_client = get_dynamodb_client().await;

    let key_condition_expression = "invalid key condition expression";
    let projection_expression = "id, user_id, #name, description, target_amount, #status";
    let index_name = "user_id-index";
    let expression_attribute_names = HashMap::from([
        ("#name".to_string(), "name".to_string()),
        ("#status".to_string(), "status".to_string()),
    ]);
    let expression_attribute_values = HashMap::from([(
        String::from(":user_id"),
        to_attribute_value("some user name - it's irrelevant here").unwrap(),
    )]);

    let response = query_campaigns(
        &db_client,
        key_condition_expression,
        projection_expression,
        index_name,
        Some(expression_attribute_names),
        Some(expression_attribute_values),
    )
    .await;
    assert!(response.is_err());
    assert_eq!(KickstarterError::InternalServer, response.err().unwrap());
}
