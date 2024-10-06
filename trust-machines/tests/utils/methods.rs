use reqwest::StatusCode;

use crate::utils::objects::CreateCampaignRequest;

use super::objects::UpdateCampaignRequest;

pub fn get_test_create_campaign_request(test_prefix: &str) -> CreateCampaignRequest {
    let campaign_name = format!("{}_name", test_prefix.to_string());
    let campaign_description = format!("{} description", test_prefix.to_string());
    let target_amount = 1100;

    let create_campaign_request = CreateCampaignRequest {
        name: campaign_name.to_owned(),
        description: campaign_description.to_owned(),
        target_amount: target_amount.to_owned(),
    };
    create_campaign_request
}

pub fn get_test_update_campaign_request(test_prefix: &str) -> UpdateCampaignRequest {
    let campaign_name = format!("{}_name", test_prefix.to_string());
    let campaign_description = format!("{} description", test_prefix.to_string());
    let target_amount = 1100;

    let update_campaign_request = UpdateCampaignRequest {
        name: campaign_name.to_owned(),
        description: campaign_description.to_owned(),
        target_amount: target_amount.to_owned(),
    };
    update_campaign_request
}

pub async fn create_test_campaign(
    user_id: &String,
    create_campaign_request: &CreateCampaignRequest,
    http_client: &reqwest::Client,
) -> String {
    let url = format!("http://localhost:8080/user/{}/campaigns/create", user_id);
    let response = http_client
        .post(url)
        .json(&create_campaign_request)
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::OK.to_string(), response.status().to_string());

    response.text().await.unwrap()
}

pub async fn start_test_campaign(
    user_id: &String,
    campaign_id: &String,
    http_client: &reqwest::Client,
) -> reqwest::Response {
    let url = format!(
        "http://localhost:8080/user/{}/campaigns/{}/start",
        user_id, campaign_id
    );
    let response = http_client.put(url).send().await.unwrap();

    response
}

pub async fn update_test_campaign(
    user_id: &String,
    campaign_id: &String,
    update_campaign_request: &UpdateCampaignRequest,
    http_client: &reqwest::Client,
) -> reqwest::Response {
    let url = format!(
        "http://localhost:8080/user/{}/campaigns/{}",
        user_id, campaign_id
    );
    let response = http_client
        .put(url)
        .json(&update_campaign_request)
        .send()
        .await
        .unwrap();

    response
}

pub async fn delete_test_campaign(
    user_id: &String,
    campaign_id: &String,
    http_client: &reqwest::Client,
) -> reqwest::Response {
    let url = format!(
        "http://localhost:8080/user/{}/campaigns/{}",
        user_id, campaign_id
    );
    let response = http_client.delete(url).send().await.unwrap();

    response
}

pub async fn get_test_campaign_by_id(
    user_id: &String,
    campaign_id: &String,
    http_client: &reqwest::Client,
) -> reqwest::Response {
    let url = format!(
        "http://localhost:8080/user/{}/campaigns/{}",
        user_id, campaign_id
    );
    let response = http_client.get(url).send().await.unwrap();

    response
}

pub async fn get_active_test_campaigns(
    user_id: &String,
    http_client: &reqwest::Client,
) -> reqwest::Response {
    let url = format!("http://localhost:8080/user/{}/campaigns/active", user_id);
    let response = http_client.get(url).send().await.unwrap();

    response
}

pub async fn get_own_test_campaigns(
    user_id: &String,
    http_client: &reqwest::Client,
) -> reqwest::Response {
    let url = format!("http://localhost:8080/user/{}/campaigns", user_id);
    let response = http_client.get(url).send().await.unwrap();

    response
}
