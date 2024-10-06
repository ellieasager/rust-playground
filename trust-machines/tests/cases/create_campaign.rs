use reqwest::StatusCode;
use rstest::rstest;

use crate::utils::{
    methods::{create_test_campaign, get_test_campaign_by_id, get_test_create_campaign_request},
    objects::{Campaign, CampaignStatus},
};

#[rstest]
#[tokio::test]
pub async fn test_create_campaign() {
    let test_prefix = "E2E_CREATE_CAMPAIGN";
    let user_id = format!("{}_user", test_prefix.to_string());
    let create_campaign_request = get_test_create_campaign_request(test_prefix);
    let http_client = reqwest::Client::new();

    let campaign_id = create_test_campaign(&user_id, &create_campaign_request, &http_client).await;

    let response = get_test_campaign_by_id(&user_id, &campaign_id, &http_client).await;
    assert_eq!(StatusCode::OK.to_string(), response.status().to_string());
    let campaign_found: Campaign = response.json().await.unwrap();

    assert_eq!(user_id, campaign_found.user_id);
    assert_eq!(create_campaign_request.name, campaign_found.name);
    assert_eq!(
        create_campaign_request.description,
        campaign_found.description
    );
    assert_eq!(
        create_campaign_request.target_amount,
        campaign_found.target_amount
    );
    // check campaign status == CREATED
    assert_eq!(CampaignStatus::Created, campaign_found.status);
}
