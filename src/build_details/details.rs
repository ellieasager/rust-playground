#[derive(Debug, Clone)]
pub struct PhoneNumberE164(pub String);

#[derive(Debug, Clone)]
pub struct Details {
    pub given_name: String,
    pub preferred_name: Option<String>,
    pub middle_name: Option<String>,
    pub family_name: String,
    pub mobile_phone: Option<PhoneNumberE164>,
    pub date_of_birth: time::Date,
    pub last_seen: Option<time::OffsetDateTime>,
}
