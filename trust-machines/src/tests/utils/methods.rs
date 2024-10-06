use uuid::Uuid;

use crate::repositories::{Campaign, CampaignStatus};

#[allow(dead_code)]
pub fn make_new_campaign(user_id: String, test_prefix: &str) -> Campaign {
    let campaign_name = format!("{}_name", test_prefix);
    let campaign_description = format!("{} description", test_prefix);
    let target_amount = 1100;

    let new_id = Uuid::new_v4();
    Campaign {
        id: new_id,
        user_id,
        name: campaign_name,
        description: campaign_description,
        target_amount,
        status: CampaignStatus::Created,
    }
}
#[allow(dead_code)]
pub fn update_name_descr_amount(original_campaign: &Campaign, new_prefix: &str) -> Campaign {
    let campaign_name = format!("{}_name", new_prefix);
    let campaign_description = format!("{} description", new_prefix);
    let target_amount = original_campaign.target_amount + 123;

    Campaign {
        id: original_campaign.id,
        user_id: original_campaign.user_id.to_owned(),
        name: campaign_name,
        description: campaign_description,
        target_amount,
        status: CampaignStatus::Created,
    }
}
