use super::details::Details;

pub struct DetailsBuilder(Details);

impl DetailsBuilder {
    pub fn new(given_name: &str, family_name: &str, date_of_birth: time::Date) -> Self {
        DetailsBuilder(Details {
            given_name: given_name.to_owned(),
            preferred_name: None,
            middle_name: None,
            family_name: family_name.to_owned(),
            mobile_phone: None,
            date_of_birth,
            last_seen: None,
        })
    }

    pub fn preferred_name(&mut self, preferred_name: &str) -> &mut Self {
      self.0.preferred_name = Some(preferred_name.to_owned());
      self
    }

    pub fn middle_name(&mut self, middle_name: &str) -> &mut Self {
      self.0.middle_name = Some(middle_name.to_owned());
      self
    }

    pub fn just_seen(&mut self) -> &mut Self {
      self.0.last_seen = Some(time::OffsetDateTime::now_utc());
      self
    }

    pub fn build(&self) -> Details {
      self.0.clone()
    }
}