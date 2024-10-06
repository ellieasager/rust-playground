use reqwest::StatusCode;
use rstest::rstest;

use crate::utils::{
    methods::{
        create_test_campaign, get_own_test_campaigns, get_test_create_campaign_request,
        start_test_campaign,
    },
    objects::GetCampaignsResponse,
};

#[rstest]
#[tokio::test]
pub async fn test_get_own_campaigns_when_there_are_none() {
    let test_prefix = "E2E_GET_OWN_CAMPAIGNS";
    let user_id = format!("{}_user", test_prefix.to_string());
    let http_client = reqwest::Client::new();

    let get_campaigns_response = get_own_test_campaigns(&user_id, &http_client).await;
    assert_eq!(
        StatusCode::OK.to_string(),
        get_campaigns_response.status().to_string()
    );

    let campaigns_found: GetCampaignsResponse = get_campaigns_response.json().await.unwrap();
    println!("campaigns_found: {:?}", campaigns_found);
    assert!(campaigns_found.campaigns.is_empty())
}

#[rstest]
#[tokio::test]
pub async fn test_get_own_campaigns() {
    let http_client = reqwest::Client::new();

    // USER 1 SETUP: one created campaign and one active campaign

    let test_prefix_user_1_created = "E2E_GET_OWN_CAMPAIGNS_USER_1_CREATED";
    let user_id_1 = format!("{}_user", test_prefix_user_1_created.to_string());

    let campaing_id_for_user_1_created = create_test_campaign(
        &user_id_1,
        &get_test_create_campaign_request(test_prefix_user_1_created),
        &http_client,
    )
    .await;

    let test_prefix_user_1_active = "E2E_GET_OWN_CAMPAIGNS_USER_1_ACTIVE";
    let campaing_id_for_user_1_active = create_test_campaign(
        &user_id_1,
        &get_test_create_campaign_request(test_prefix_user_1_active),
        &http_client,
    )
    .await;

    let start_campaign_response_user_1 =
        start_test_campaign(&user_id_1, &campaing_id_for_user_1_active, &http_client).await;
    assert_eq!(
        StatusCode::OK.to_string(),
        start_campaign_response_user_1.status().to_string()
    );

    // USER 2 SETUP: one created campaign and one active campaign

    let test_prefix_user_2_created = "E2E_GET_OWN_CAMPAIGNS_USER_2_CREATED";
    let user_id_2 = format!("{}_user", test_prefix_user_2_created.to_string());

    let campaing_id_for_user_2_created = create_test_campaign(
        &user_id_2,
        &get_test_create_campaign_request(test_prefix_user_2_created),
        &http_client,
    )
    .await;

    let test_prefix_user_2_active = "E2E_GET_OWN_CAMPAIGNS_USER_2_ACTIVE";
    let campaing_id_for_user_2_active = create_test_campaign(
        &user_id_2,
        &get_test_create_campaign_request(test_prefix_user_2_active),
        &http_client,
    )
    .await;

    let start_campaign_response_user_2 =
        start_test_campaign(&user_id_2, &campaing_id_for_user_2_active, &http_client).await;
    assert_eq!(
        StatusCode::OK.to_string(),
        start_campaign_response_user_2.status().to_string()
    );

    // NOW TEST to confirm that user 1 can only see their own campaigns regardless of the status

    let get_campaigns_response = get_own_test_campaigns(&user_id_1, &http_client).await;
    assert_eq!(
        StatusCode::OK.to_string(),
        get_campaigns_response.status().to_string()
    );

    let campaigns_found_response: GetCampaignsResponse =
        get_campaigns_response.json().await.unwrap();
    let campaigns_found = campaigns_found_response.campaigns;

    // we should find at least 2 campaigns created for our user in this test,
    // but there could be more in database due to other tests creating data;
    // also other user's campaigns should not be in this list
    assert!(campaigns_found.len() >= 2);
    let campaigns_ids_found: Vec<String> =
        campaigns_found.iter().map(|c| c.id.to_string()).collect();
    assert!(campaigns_ids_found.contains(&campaing_id_for_user_1_created));
    assert!(campaigns_ids_found.contains(&campaing_id_for_user_1_active));
    assert!(!campaigns_ids_found.contains(&campaing_id_for_user_2_created));
    assert!(!campaigns_ids_found.contains(&campaing_id_for_user_2_active));
}
