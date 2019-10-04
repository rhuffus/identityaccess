use diesel::associations::Identifiable;

pub struct TenantId {
	id: String
}

impl Identifiable for TenantId {
	type Id = String;
	fn id(self) -> Self::Id {
		self.id
	}
}