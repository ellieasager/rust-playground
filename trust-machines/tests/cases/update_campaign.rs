use reqwest::StatusCode;
use rstest::rstest;

use crate::utils::{
    methods::{
        create_test_campaign, get_test_campaign_by_id, get_test_create_campaign_request,
        get_test_update_campaign_request, start_test_campaign, update_test_campaign,
    },
    objects::Campaign,
};

#[rstest]
#[tokio::test]
pub async fn test_update_campaign_ok() {
    let test_prefix_original = "E2E_UPDATE_CAMPAIGN_ORIGINAL";
    let user_id = format!("{}_user_0", test_prefix_original.to_string());
    let create_campaign_request = get_test_create_campaign_request(test_prefix_original);
    let http_client = reqwest::Client::new();

    let campaign_id = create_test_campaign(&user_id, &create_campaign_request, &http_client).await;

    let test_prefix_updated = "E2E_UPDATE_CAMPAIGN_UPDATED";
    let update_campaign_request = get_test_update_campaign_request(test_prefix_updated);

    let update_campaign_response = update_test_campaign(
        &user_id,
        &campaign_id,
        &update_campaign_request,
        &http_client,
    )
    .await;
    assert_eq!(
        StatusCode::OK.to_string(),
        update_campaign_response.status().to_string()
    );

    let get_campaign_response = get_test_campaign_by_id(&user_id, &campaign_id, &http_client).await;
    assert_eq!(
        StatusCode::OK.to_string(),
        get_campaign_response.status().to_string()
    );
    let campaign_found: Campaign = get_campaign_response.json().await.unwrap();
    assert_eq!(user_id, campaign_found.user_id);
    assert_eq!(update_campaign_request.name, campaign_found.name);
    assert_eq!(
        update_campaign_request.description,
        campaign_found.description
    );
    assert_eq!(
        update_campaign_request.target_amount,
        campaign_found.target_amount
    );
}

#[rstest]
#[tokio::test]
pub async fn test_update_other_user_campaign_fail() {
    let test_prefix_original = "E2E_UPDATE_CAMPAIGN_ORIGINAL";
    let user_id_1 = format!("{}_user_1", test_prefix_original.to_string());
    let create_campaign_request = get_test_create_campaign_request(test_prefix_original);
    let http_client = reqwest::Client::new();

    let campaign_id =
        create_test_campaign(&user_id_1, &create_campaign_request, &http_client).await;

    let start_campaign_response = start_test_campaign(&user_id_1, &campaign_id, &http_client).await;
    assert_eq!(
        StatusCode::OK.to_string(),
        start_campaign_response.status().to_string()
    );

    let test_prefix_updated = "E2E_UPDATE_CAMPAIGN_UPDATED";
    let update_campaign_request = get_test_update_campaign_request(test_prefix_updated);

    let user_id_2 = format!("{}_user_2", test_prefix_updated.to_string());
    let response_for_user_2 = update_test_campaign(
        &user_id_2,
        &campaign_id,
        &update_campaign_request,
        &http_client,
    )
    .await;
    println!(
        "response from test_update_campaign_ok: {:?}",
        response_for_user_2
    );
    assert_eq!(
        StatusCode::NOT_FOUND.to_string(),
        response_for_user_2.status().to_string()
    );

    let get_campaign_response =
        get_test_campaign_by_id(&user_id_1, &campaign_id, &http_client).await;
    assert_eq!(
        StatusCode::OK.to_string(),
        get_campaign_response.status().to_string()
    );
    let campaign_found: Campaign = get_campaign_response.json().await.unwrap();
    println!("campaign_found: {:?}", campaign_found);

    assert_eq!(user_id_1, campaign_found.user_id);
    assert_eq!(create_campaign_request.name, campaign_found.name);
    assert_eq!(
        create_campaign_request.description,
        campaign_found.description
    );
    assert_eq!(
        create_campaign_request.target_amount,
        campaign_found.target_amount
    );
}

#[rstest]
#[tokio::test]
pub async fn test_update_campaign_after_started_fail() {
    let test_prefix = "E2E_UPDATE_CAMPAIGN_";
    let user_id = format!("{}_user", test_prefix.to_string());
    let create_campaign_request = get_test_create_campaign_request(test_prefix);
    let http_client = reqwest::Client::new();

    let campaign_id = create_test_campaign(&user_id, &create_campaign_request, &http_client).await;

    let start_campaign_response = start_test_campaign(&user_id, &campaign_id, &http_client).await;
    assert_eq!(
        StatusCode::OK.to_string(),
        start_campaign_response.status().to_string()
    );

    let test_prefix_updated = "E2E_UPDATE_CAMPAIGN_UPDATED";
    let update_campaign_request = get_test_update_campaign_request(test_prefix_updated);

    let update_campaign_response = update_test_campaign(
        &user_id,
        &campaign_id,
        &update_campaign_request,
        &http_client,
    )
    .await;
    assert_eq!(
        StatusCode::BAD_REQUEST.to_string(),
        update_campaign_response.status().to_string()
    );

    let get_campaign_response = get_test_campaign_by_id(&user_id, &campaign_id, &http_client).await;
    assert_eq!(
        StatusCode::OK.to_string(),
        get_campaign_response.status().to_string()
    );
    let campaign_found: Campaign = get_campaign_response.json().await.unwrap();
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
}
