use build_details::details_builder::DetailsBuilder;

// use time::OffsetDateTime;

mod build_details;

fn main() {
    details_builder_playground();
}

fn details_builder_playground() {
    let mut builder = DetailsBuilder::new("Robert", "Builder", time::Date::from_calendar_date(1998, time::Month::November, 28).unwrap());
    builder
    .middle_name("the");
    let informal = true;
    if informal {
        builder.preferred_name("Bob");
    }
    let bob = builder.just_seen().build();
    println!("bob: {:?}", bob);
    // println!("bob: {:?}", builder.build());
    let bobbies = vec![builder.build(), builder.build(), builder.build()];
    println!("bobbies: {:?}", bobbies);
}
// #[derive(Debug, Clone)]
// pub struct PhoneNumberE164(pub String);

// #[derive(Debug, Default)]
// pub struct Details {
//   pub given_name: String,
//   pub preferred_name: Option<String>,
//   pub middle_name: Option<String>,
//   pub family_name: String,
//   pub mobile_phone: Option<PhoneNumberE164>,
// //   pub date_of_birth: time::Date,
// //   pub last_seen: Option<time::OffsetDateTime>,
// }
