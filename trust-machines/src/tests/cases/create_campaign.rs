use rstest::rstest;

#[allow(unused_imports)]
use crate::{
    get_dynamodb_client,
    repositories::{
        campaigns_repository::{create_campaign, get_campaign_by_id},
        Campaign,
    },
    tests::utils::methods::make_new_campaign,
};

#[rstest]
#[tokio::test]
pub async fn test_create_campaign() {
    let db_client = get_dynamodb_client().await;

    let test_prefix = "E2E_CREATE_CAMPAIGN";
    let user_id = format!("{}_user", test_prefix.to_string());

    let campaign_created: Campaign = make_new_campaign(user_id.to_owned(), test_prefix);

    let response = create_campaign(&db_client, campaign_created.clone()).await;
    assert!(response.is_ok());
    let campaign_id = response.ok().unwrap();

    let response = get_campaign_by_id(&db_client, &campaign_id).await;
    assert!(response.is_ok());
    let campaign_found: Campaign = response.ok().unwrap();
    assert_eq!(campaign_created, campaign_found);
}
