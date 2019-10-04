use crate::domain::model::identity::{email_address::EmailAddress, postal_address::PostalAddress, telephone::Telephone};

pub struct ContactInformation {
	email_address: EmailAddress,
	postal_address: PostalAddress,
	primary_telephone: Telephone,
	secondary_telephone: Telephone,
}