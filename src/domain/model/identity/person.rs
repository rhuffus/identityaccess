use crate::domain::model::identity::contact_information::ContactInformation;
use crate::domain::model::identity::full_name::FullName;
use crate::domain::model::identity::tenant_id::TenantId;

pub struct Person {
	contact_information: ContactInformation,
	full_name: FullName,
	tenant_id: TenantId,
}