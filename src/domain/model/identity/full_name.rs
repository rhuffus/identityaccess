#[derive(Debug)]
pub struct FullName {
	pub first_name: String,
	pub last_name: String,
}

impl FullName {
	pub fn as_formatted_name(&self) -> String {
		format!("{} {}", self.first_name, self.last_name)
	}
}